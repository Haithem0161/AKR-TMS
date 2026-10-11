#!/usr/bin/env python3
"""Insert a block of text into a phase file before a marker line (the first line equal to MARKER).
Usage: owner-insert.py PHASE_FILE BLOCK_FILE [MARKER]   (MARKER default: '## Decisions')
Refuses when the first heading line of the block is already in the phase file."""
import sys
path, block = sys.argv[1], open(sys.argv[2]).read()
marker = sys.argv[3] if len(sys.argv) > 3 else '## Decisions'
text = open(path).read()
head = next(l for l in block.split('\n') if l.startswith('### '))
if head in text:
    sys.exit(f'already there: {head}')
lines = text.split('\n')
idx = [i for i, l in enumerate(lines) if l == marker]
if len(idx) != 1:
    sys.exit(f'marker found {len(idx)} times: {marker}')
i = idx[0]
out = lines[:i] + block.rstrip('\n').split('\n') + [''] + lines[i:]
open(path, 'w').write('\n'.join(out))
print(f'inserted {len(block.splitlines())} lines before line {i+1} of {path}')
