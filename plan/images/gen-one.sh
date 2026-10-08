#!/usr/bin/env bash
# Generate one mockup: assemble the prompt from the shared head/tail + body, run Codex in an isolated work dir,
# then move the result into images/. Usage: gen-one.sh <name-without-ext>   e.g. 02-investigation
set -u
cd "$(dirname "$0")"
n="$1"
cat prompts/_common-head.txt "prompts/$n.body.txt" prompts/_common-tail.txt > "prompts/$n.txt"
mkdir -p "work/$n"
start=$(date +%s)
node ~/.claude/skills/generate-image/scripts/gen.mjs --prompt-file "prompts/$n.txt" \
  --out "work/$n/$n.png" --size 1536x1024 --ref "$PWD/01-command-centre-v1.png" > "work/$n/log.txt" 2>&1
rc=$?
if [ $rc -eq 0 ] && [ -s "work/$n/$n.png" ]; then
  mv -f "work/$n/$n.png" "$n.png"; mv -f "work/$n/$n.png.json" "$n.png.json" 2>/dev/null
  echo "OK $n $(( $(date +%s) - start ))s"
else
  echo "FAIL $n rc=$rc (see images/work/$n/log.txt)"
fi
