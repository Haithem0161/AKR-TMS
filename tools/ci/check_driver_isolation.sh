#!/usr/bin/env bash
# Driver isolation (TS-02), the second check beside `cargo deny check bans`: the binaries that never
# talk to the CBS must not link the source adapters at all.
set -euo pipefail

cd "$(dirname "$0")/../../backend"
failed=0
for binary in akr-api akr-worker akr-assess akr-s3guard; do
    if cargo tree --locked -p "$binary" --edges normal,build --prefix none | grep -q '^akr-sources '; then
        echo "$binary depends on akr-sources" >&2
        failed=1
    fi
done
[ "$failed" -eq 0 ] && echo "driver isolation: akr-sources is linked into akr-ingestd only"
exit "$failed"
