#!/usr/bin/env python3
"""Owner-decision round 1 fixes: phase 07 (verdicts O-07-1..11, the O-08-1 sentence, L-1, L-5 in tools/agent/owner-verdict.md)."""
import sys
sys.path.insert(0, "/home/haithem/Projects/AKR-TMS/tools/agent")
from ofix import Fixer

f = Fixer(round_no=1)
P = "docs/plan/phase-07.md"

# O-07-1: the re-run of a stale intake is enqueued where it commits (the profile activation), not in a refused decision
f.fix("O-07-1", P,
      "runs already queued keep their pinned version; audit `goaml_profile.activated {code, profile_version, from_version, change_request_id}` |",
      "runs already queued keep their pinned version; audit `goaml_profile.activated {code, profile_version, from_version, change_request_id}`; then enqueue `goaml_case_intake {case_id}` for every `proposed` bank case of the code that has a stored file (`stored_proposed_for_code`, section 7.6 item 4), so its intake result is taken again under the new active version before anyone decides it |")
f.fix("O-07-1", P,
      "`intake_result.profile_version` differs from the active version's -> enqueue `goaml_case_intake {case_id}` again and issue `intake_stale` (the approver decides after the job).",
      "`intake_result.profile_version` differs from the active version's -> issue `intake_stale`. This hook enqueues nothing: an issue is a refusal, phase 04 rolls the action back, and a job enqueued in a rolled-back transaction leaves no row. The intake is run again by the activation of the profile version (section 3.6, Activate), which commits, and the approver decides after that job.")
f.fix("O-07-1", P,
      "    async fn count_open(&self, conn: &mut PgConnection, code: &str) -> Result<i64, RepoError>;          // proposed + active\n    async fn set_intake(",
      "    async fn count_open(&self, conn: &mut PgConnection, code: &str) -> Result<i64, RepoError>;          // proposed + active\n    async fn stored_proposed_for_code(&self, tx: &mut Tx, code: &str) -> Result<Vec<Uuid>, RepoError>;  // proposed, evidence_id set\n    async fn set_intake(")
f.fix("O-07-1", P,
      "   4. Later run (`evidence_id` is set; item 7 asks for it when the active profile changed): `EvidenceStore::open(evidence_id)` gives the masked bytes.",
      "   4. Later run (`evidence_id` is set; the activation of a profile version of the code asks for it, section 3.6): `EvidenceStore::open(evidence_id)` gives the masked bytes.")
f.fix("O-07-1", P,
      "  - `goaml_case_intake` (payload `{case_id}`; enqueued when a bank case is proposed and when its intake is stale;\n",
      "  - `goaml_case_intake` (payload `{case_id}`; enqueued when a bank case is proposed, and for every `proposed` case\n    with a stored file when a profile version of its code is activated (section 3.6);\n")
f.fix("O-07-1", P,
      "after a profile activation -> 422 `intake_stale` and a new `goaml_case_intake` job, then approval succeeds;",
      "a profile activation enqueues one `goaml_case_intake` job for the `proposed` case; a decision before that job ran -> 422 `intake_stale` and no new job row; after it ran, approval succeeds;")

# O-07-2, O-07-3: the curl sequence
f.fix("O-07-2", P,
      "as CFG the POST -> 403 `E_SCOPE`; as DS the GET -> 403 `E_SCOPE`;",
      "as CFG the POST -> 403 `E_SCOPE`; as DS the GET -> 403 `E_FORBIDDEN` (DS holds `config.author`, which the route accepts, and Cedar denies); as SS the GET -> 403 `E_SCOPE`;")
f.fix("O-07-3", P,
      "the same key and body -> the stored 202 with `x-idempotent-replay: true`; as CHK approve -> the case `active`; as CFG `GET /api/v1/goaml/corpus-cases`",
      "the same key and body -> the stored 202 with `x-idempotent-replay: true`; after the intake job has completed (the script polls `GET /api/v1/goaml/corpus-cases` as A2 until `intake_result` is set), as CHK approve -> the case `active`; as CFG `GET /api/v1/goaml/corpus-cases`")

# O-07-4, O-07-5: the uploaded file of a case that left `proposed` before its intake; the audit event of the deletion
f.fix("O-07-4", P,
      "   1. Load the case. A state other than `proposed` -> done.",
      "   1. Load the case. A state other than `proposed` -> done; when the case has no `intake_result` (its request was withdrawn or rejected before the first run, so the uploaded file is still staged), first `delete_transient(\"goaml\", case_id)` and the audit event `goaml_corpus_case.transient_deleted {case_id, by: \"intake\"}`.")
f.fix("O-07-5", P,
      "   8. After the transaction of a first run, in every path: `delete_transient(\"goaml\", case_id)`. An object left by a crash is deleted by `maintenance_sweep` after `evidence.staging_ttl_h`: its owner is not a run, which the sweep of section 3.2 step 13 treats as a missing run.",
      "   8. After the transaction of a first run, in every path: `delete_transient(\"goaml\", case_id)`, then the audit event `goaml_corpus_case.transient_deleted {case_id, by: \"intake\"}`. An object left by a crash is deleted by `maintenance_sweep` after `evidence.staging_ttl_h`: its owner is not a run, which the sweep of section 3.2 step 13 treats as a missing run; when the owner id is the id of a corpus case the sweep writes `goaml_corpus_case.transient_deleted {case_id, by: \"sweep\"}` in place of `goaml_check.transient_deleted`.")
f.fix("O-07-5", P,
      "9. **Audit events** (six more kinds in section 3.8, all on resource `goaml_corpus_case`): `goaml_corpus_case.proposed`, `goaml_corpus_case.retire_proposed`, `goaml_corpus_case.intake_ran`, `goaml_corpus_case.activated`, `goaml_corpus_case.retired` and `goaml_corpus_case.rejected`.",
      "9. **Audit events** (seven more kinds in section 3.8, all on resource `goaml_corpus_case`): `goaml_corpus_case.proposed`, `goaml_corpus_case.retire_proposed`, `goaml_corpus_case.intake_ran`, `goaml_corpus_case.transient_deleted` (`{case_id, by}`, `by` is `intake` or `sweep`; the uploaded file is gone, as `goaml_check.transient_deleted` says for a check run), `goaml_corpus_case.activated`, `goaml_corpus_case.retired` and `goaml_corpus_case.rejected`. The failed proposal of item 5 step 5 writes no event: no case row exists, as for a failed check request (section 3.2 step 4).")
f.fix("O-07-5", P,
      "    final or missing are deleted and the run flagged `transient_deleted` (audit `goaml_check.transient_deleted` with\n    `by: sweep`).",
      "    final or missing are deleted and the run flagged `transient_deleted` (audit `goaml_check.transient_deleted` with\n    `by: sweep`; for an object whose owner is a bank corpus case, `goaml_corpus_case.transient_deleted`, section 7.6\n    item 6).")
f.fix("O-07-5", P,
      "a file with a `<!DOCTYPE` -> refusal `not_maskable`; an EICAR file -> refusal `unsafe_file`.",
      "a file with a `<!DOCTYPE` -> refusal `not_maskable`; an EICAR file -> refusal `unsafe_file`; a case whose request is withdrawn before the job ran -> after the job no object under `transient/goaml/<case_id>/` and one `goaml_corpus_case.transient_deleted` event.")
f.fix("O-07-5", P,
      "Audit: `goaml_corpus_case.proposed`, `.intake_ran`, `.activated`, `.retire_proposed` appear,",
      "Audit: `goaml_corpus_case.proposed`, `.intake_ran`, `.transient_deleted`, `.activated`, `.retire_proposed` appear,")

# O-07-6: nothing in the system screens the key
f.fix("O-07-6", P,
      "The key is typed by the maker and read by the approver, so a key that names a person is refused at approval, as any reason text is.",
      "The key is typed by the maker and read by the approver. The form asks for a short code for the rule and says that it must not be a name (item 13), and the approver rejects a request whose key names a person; the system does not screen the key.")
f.fix("O-07-6", P,
      "for A2 and M the action \"Add a case\" (one XML file, a key, the expected outcome,",
      "for A2 and M the action \"Add a case\" (one XML file, a key with the hint \"A short code for the rule. Never a name.\", the expected outcome,")

# O-07-7: U+063B to U+0640 stay inside the class of patch P-01
f.fix("O-07-7", P,
      "| letter in U+0621 to U+063A or U+0641 to U+064A | a random letter of those two ranges |",
      "| letter in U+0621 to U+064A (the class `ء-ي` of patch `P-01`; the tatweel U+0640 is one of them) | a random letter of U+0621 to U+063A or U+0641 to U+064A, so a value the class accepts stays accepted |")
f.fix("O-07-7", P,
      "a letter U+0695 becomes U+06A9; the three validated samples,",
      "a letter U+0695 becomes U+06A9; a tatweel U+0640 becomes a letter of U+0621 to U+063A or U+0641 to U+064A, and an entity name that holds one still passes `P-01`; the three validated samples,")

# O-07-8 and L-5: the base review and the "What is kept" paragraph name the exception; the review says that dates stay
f.fix("O-07-8", P,
      "  of the run (DR-69); nothing derived from them holds a value, a name or the narrative. The run rows are `secret-str`,",
      "  of the run (DR-69); nothing derived from them holds a value, a name or the narrative. The one exception is the\n  masked file of a corpus case the bank adds (section 7.6): it keeps enumeration codes, booleans, dates, date-times\n  and the bank's own goAML identifiers, no route returns it, and nothing in it names a person. The run rows are `secret-str`,")
f.fix("O-07-8", P,
      "element names). Never a value from the file, never the libxml2 message, never the rejection text. The requester\n",
      "element names). Never a value from the file, never the libxml2 message, never the rejection text. (This is the rule\nfor a check run. A corpus case the bank adds keeps a masked copy of its file; section 7.6 item 2 says what stays in\nit.) The requester\n")
f.fix("L-5", P,
      "What is kept is the masked file: no name, identifier, account number, address, telephone number, amount or narrative of the original is in it, and no route returns it to anyone.",
      "What is kept is the masked file: no name, identifier, account number, address, telephone number, amount or narrative of the original is in it, and no route returns it to anyone. Dates and date-times stay as they were, a date of birth among them, as do enumeration codes such as nationality and gender, because the rules read them; with every name and number beside them replaced they do not name a person, and the file is `secret-str` evidence that only the corpus runner opens.")

# O-07-9: the form of the Office code
f.fix("O-07-9", P,
      "`office_code`, when given, is the `rule_code` of a rejection pattern of the active version (else 422 `rule_unknown` at `/office_code`);",
      "`office_code`, when given, has the form `OFF-nn` that the column CHECK admits and is the `rule_code` of a rejection pattern of the active version (else 422 `rule_unknown` at `/office_code`);")

# O-07-10: what a refusal at activation does on each path
f.fix("O-07-10", P,
      "not `proposed`, or the open count above the cap -> the target refuses and the request ends `failed`; `activate` (`activated_at` = the request's `activated_at`);",
      "not `proposed`, or the open count above the cap -> the target refuses (phase 04 section 3.2: an immediate approval is rolled back with the error, and a scheduled activation ends `failed`); `activate` (`activated_at` = the request's `activated_at`);")
f.fix("O-07-10", P,
      "`retire`: `lock`; not `active` -> refused; `retire` with the request id; audit `goaml_corpus_case.retired {case_id, change_request_id}`.",
      "`retire`: `lock`; not `active` -> refused in the same way; `retire` with the request id; audit `goaml_corpus_case.retired {case_id, change_request_id}`.")

# O-07-11: the crate listing of section 2
f.fix("O-07-11", P,
      "│   └── jobs/      # + kinds goaml_check, goaml_watch, goaml_corpus (akr-worker)",
      "│   └── jobs/      # + kinds goaml_check, goaml_watch, goaml_corpus, goaml_case_intake (akr-worker; the last in section 7.6)")
f.fix("O-07-11", P,
      "├── api/           # + router goaml; change target goaml_profile",
      "├── api/           # + router goaml; change targets goaml_profile and goaml_corpus_case (section 7.6)")
f.fix("O-07-11", P,
      "├── worker/        # + the three job handlers, SandboxedXsdValidator, ProxySchemaFetcher",
      "├── worker/        # + the four job handlers (the fourth in section 7.6), SandboxedXsdValidator, ProxySchemaFetcher")
f.fix("O-07-11", P,
      "│   │              #   PackageChecks (zip), RejectionParser, the corpus runner",
      "│   │              #   PackageChecks (zip), RejectionParser, the corpus runner, CaseMasker (section 7.6)")

# O-08-1 (linked): who ends a case that did not meet its expectation
f.fix("O-08-1", P,
      "cannot be approved: its maker withdraws the request and the case becomes `rejected`.",
      "cannot be approved: the approver rejects the request or its maker withdraws it, and the case becomes `rejected`.")

# L-1: a DELETE without the privilege never reaches the guard trigger
f.fix("L-1", P,
      "Negative checks: `DELETE FROM rpt.goaml_profiles` fails with\n   `profile versions are never deleted`;",
      "Negative checks: `DELETE FROM rpt.goaml_profiles` as the owner `akr_migrate` fails with\n   `profile versions are never deleted` (as `akr_app` it fails with permission denied, the last check of this item);")
f.fix("L-1", P,
      "`DELETE` fails with `goAML corpus cases are never deleted`; an UPDATE of a `rejected` row fails;",
      "a `DELETE` fails, as `akr_app` with permission denied (no DELETE grant) and as the owner `akr_migrate` with `goAML corpus cases are never deleted`; as `akr_app` an UPDATE of the `state` of a `rejected` row fails with the guard's `is final` message;")

f.done()
