#!/usr/bin/env bash
# Refuses a frontend package that ships in the bundle under a licence outside the allow-list. Development
# tools are not shipped and are not judged here.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
ALLOWED='["MIT","MIT-0","Apache-2.0","BSD-2-Clause","BSD-3-Clause","ISC","0BSD","MPL-2.0","CC0-1.0","Unlicense"]'

refused="$(pnpm --dir "$ROOT/frontend" licenses list --prod --json |
    jq -r --argjson allowed "$ALLOWED" '
        to_entries[] | select(.key as $licence | $allowed | index($licence) | not)
        | .key as $licence | .value[] | "\(.name) \(.versions | join(","))  \($licence)"')"

if [ -n "$refused" ]; then
    echo "frontend licences: outside the allow-list:" >&2
    echo "$refused" >&2
    exit 1
fi
echo "frontend licences: every shipped package is inside the allow-list"
