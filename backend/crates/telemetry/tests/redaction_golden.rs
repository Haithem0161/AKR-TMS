//! The golden test of redaction: four known values are fed through every sink, and none survives in what
//! the sink emits (the log bytes, the bytes posted to the collector, the Prometheus exposition).
//!
//! The values are synthetic. The name can only be caught by its field (free text is not searched for
//! names), which is why payloads and messages carry ids and codes and never a name.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::io::{Read, Write};
use std::net::TcpListener;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use akr_common::config::Env;
use akr_domain::classification::{Classification, ColumnClassification};
use akr_telemetry::metrics::detached;
use akr_telemetry::traces::otlp_layer;
use akr_telemetry::{JsonLayer, SharedRedactor};
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::layer::SubscriberExt;

const NAME: &str = "Golden Synthetic Holder";
const IBAN: &str = "IQ98NBIQ850123456789012";
const IBAN_SPACED: &str = "IQ98 NBIQ 8501 2345 6789 012";
const CARD: &str = "4111111111111111";
const CARD_SPACED: &str = "4111 1111 1111 1111";
const NATIONAL_ID: &str = "199012345678";
const PARTY_ID: &str = "0198c1de-7b7e-7cc3-9c0b-0a2f4d5e6f70";

const GOLDEN: [&str; 6] = [NAME, IBAN, IBAN_SPACED, CARD, CARD_SPACED, NATIONAL_ID];

fn redactor() -> SharedRedactor {
    let column = |name: &str, classification| ColumnClassification {
        schema_name: "canon".to_owned(),
        table_name: "party".to_owned(),
        column_name: name.to_owned(),
        classification,
        encrypted: false,
        keyed_hash: false,
        note: None,
    };
    let redactor = SharedRedactor::new();
    redactor.load_classification(&[
        column("full_name", Classification::Personal),
        column("iban", Classification::Financial),
        column("pan", Classification::Card),
        column("document_number", Classification::Personal),
    ]);
    redactor
}

fn assert_clean(sink: &str, output: &[u8]) {
    assert!(!output.is_empty(), "the {sink} sink emitted nothing");
    for golden in GOLDEN {
        assert!(
            !output
                .windows(golden.len())
                .any(|window| window == golden.as_bytes()),
            "the {sink} sink shows `{golden}`"
        );
    }
}

/// Every way a call site can pass a value: a classified field, an unclassified field, a body, a number,
/// an error, a debug rendering, the message, a span field and a field recorded later.
fn emit() {
    let span = tracing::info_span!(
        "transfer_review",
        correlation_id = PARTY_ID,
        full_name = NAME,
        counterparty = IBAN,
        late = tracing::field::Empty,
    );
    let _entered = span.enter();
    span.record("late", CARD_SPACED);

    let failure = std::io::Error::other(format!("no account {IBAN} for holder {NATIONAL_ID}"));
    tracing::info!(
        full_name = NAME,
        iban = IBAN,
        pan = CARD,
        document_number = NATIONAL_ID,
        note = IBAN_SPACED,
        seen = CARD,
        holder = NATIONAL_ID,
        card_number = 4111111111111111_u64,
        id_number = 199012345678_i64,
        request_body = format!("{{\"name\":\"{NAME}\",\"iban\":\"{IBAN}\"}}"),
        response_body = format!("{{\"holder\":\"{NAME}\"}}"),
        error = &failure as &dyn std::error::Error,
        parts = ?[IBAN, CARD, NATIONAL_ID],
        "payment from {IBAN} with card {CARD_SPACED} by holder {NATIONAL_ID}"
    );
    tracing::error!(
        payload = format!("{{\"name\":\"{NAME}\"}}"),
        "refused {CARD}"
    );
}

#[derive(Clone, Default)]
struct Capture(Arc<Mutex<Vec<u8>>>);

impl Write for Capture {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.lock().unwrap().extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl<'a> MakeWriter<'a> for Capture {
    type Writer = Capture;

    fn make_writer(&'a self) -> Self::Writer {
        self.clone()
    }
}

#[test]
fn no_golden_value_reaches_the_log() {
    let capture = Capture::default();
    let layer = JsonLayer::new("akr-golden", "node-1", redactor(), capture.clone());
    tracing::subscriber::with_default(tracing_subscriber::registry().with(layer), emit);

    let output = capture.0.lock().unwrap().clone();
    assert_clean("log", &output);

    let text = String::from_utf8(output).unwrap();
    assert_eq!(text.lines().count(), 2);
    for kept in [
        PARTY_ID,
        "transfer_review",
        "[redacted:iban]",
        "[redacted:card]",
        "[redacted:national-id]",
        &format!("[redacted:{}]", NAME.len()),
    ] {
        assert!(text.contains(kept), "the log lost `{kept}`: {text}");
    }
}

struct Collected {
    request_line: String,
    content_type: String,
    body: Vec<u8>,
}

/// One HTTP request, answered 200: what a collector does with a batch.
fn one_shot_collector() -> (String, mpsc::Receiver<Collected>) {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind");
    let addr = listener.local_addr().expect("addr");
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        let (mut stream, _) = listener.accept().expect("accept");
        stream
            .set_read_timeout(Some(Duration::from_secs(10)))
            .expect("timeout");
        let mut raw = Vec::new();
        let mut buf = [0u8; 8192];
        let (head_end, content_length) = loop {
            let n = stream.read(&mut buf).expect("read");
            assert!(n > 0, "the exporter closed the connection before sending");
            raw.extend_from_slice(&buf[..n]);
            if let Some(pos) = raw.windows(4).position(|w| w == b"\r\n\r\n") {
                let head = String::from_utf8_lossy(&raw[..pos]).to_string();
                let len = head
                    .lines()
                    .find_map(|line| {
                        let (name, value) = line.split_once(':')?;
                        name.eq_ignore_ascii_case("content-length")
                            .then(|| value.trim().parse::<usize>().ok())
                            .flatten()
                    })
                    .expect("a content-length");
                break (pos + 4, len);
            }
        };
        while raw.len() < head_end + content_length {
            let n = stream.read(&mut buf).expect("read body");
            assert!(n > 0, "the body was cut short");
            raw.extend_from_slice(&buf[..n]);
        }
        let head = String::from_utf8_lossy(&raw[..head_end]).to_string();
        let header = |wanted: &str| {
            head.lines()
                .find_map(|line| {
                    let (name, value) = line.split_once(':')?;
                    name.eq_ignore_ascii_case(wanted)
                        .then(|| value.trim().to_owned())
                })
                .unwrap_or_default()
        };
        stream
            .write_all(b"HTTP/1.1 200 OK\r\ncontent-length: 0\r\nconnection: close\r\n\r\n")
            .expect("answer");
        tx.send(Collected {
            request_line: head.lines().next().unwrap_or_default().to_owned(),
            content_type: header("content-type"),
            body: raw[head_end..head_end + content_length].to_vec(),
        })
        .expect("hand back");
    });
    (format!("http://{addr}"), rx)
}

#[test]
fn no_golden_value_reaches_the_collector() {
    let (endpoint, collected) = one_shot_collector();
    let (layer, export) = otlp_layer(
        Some(&endpoint),
        "akr-golden",
        Env::Test,
        "node-1",
        redactor(),
    )
    .unwrap()
    .expect("an endpoint gives a layer");

    tracing::subscriber::with_default(tracing_subscriber::registry().with(layer), emit);
    export.flush();

    let got = collected
        .recv_timeout(Duration::from_secs(15))
        .expect("the collector received a batch");
    assert_eq!(got.request_line, "POST /v1/traces HTTP/1.1");
    assert_eq!(got.content_type, "application/x-protobuf");
    assert_clean("trace", &got.body);

    let contains = |needle: &str| {
        got.body
            .windows(needle.len())
            .any(|w| w == needle.as_bytes())
    };
    for kept in [
        "transfer_review",
        "akr-golden",
        "node-1",
        PARTY_ID,
        "[redacted:iban]",
        "[redacted:card]",
    ] {
        assert!(contains(kept), "the batch lost `{kept}`");
    }
}

#[test]
fn a_dead_collector_costs_spans_and_nothing_else() {
    let port = TcpListener::bind("127.0.0.1:0")
        .unwrap()
        .local_addr()
        .unwrap()
        .port();
    let (layer, export) = otlp_layer(
        Some(&format!("http://127.0.0.1:{port}")),
        "akr-golden",
        Env::Test,
        "node-1",
        redactor(),
    )
    .unwrap()
    .unwrap();

    let started = std::time::Instant::now();
    tracing::subscriber::with_default(tracing_subscriber::registry().with(layer), || {
        let _span = tracing::info_span!("nobody_is_listening").entered();
    });
    export.flush();
    drop(export);
    assert!(started.elapsed() < Duration::from_secs(15));
}

#[test]
fn no_golden_value_reaches_the_metrics() {
    let (recorder, handle) = detached(redactor()).unwrap();
    metrics::with_local_recorder(&recorder, || {
        metrics::counter!(
            "akr_golden_total",
            "full_name" => NAME,
            "account" => IBAN,
            "card" => CARD_SPACED,
            "holder" => format!("id {NATIONAL_ID}"),
            "party" => PARTY_ID,
            "kind" => "golden"
        )
        .increment(1);
        metrics::gauge!("akr_golden_gauge", "pan" => CARD).set(1.0);
        metrics::histogram!("akr_golden_seconds", "note" => IBAN_SPACED).record(0.2);
    });

    let text = handle.render();
    assert_clean("metric", text.as_bytes());
    assert!(!text.contains(PARTY_ID), "a label carries an id: {text}");
    for kept in [
        "akr_golden_total",
        "akr_golden_gauge",
        "akr_golden_seconds",
        r#"kind="golden""#,
    ] {
        assert!(text.contains(kept), "the exposition lost `{kept}`: {text}");
    }
}
