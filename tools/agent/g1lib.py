"""Shared helpers for the gap pass 1 apply scripts (apply-and-assert; callers write files only at the end)."""
import re

NONE_RE = re.compile(r'(## 7\+\. Gap Additions\n\n)(_None yet\._|None yet\.)\n')


def rep(text, a, b, count=1):
    assert text.count(a) == count, (a[:70], text.count(a))
    return text.replace(a, b)


def add_section7(text, header, blocks):
    """Append blocks (each starting '### 7.N <title>') to the phase's '## 7+. Gap Additions' section.

    Blocks are renumbered to follow the last 7.N already in the section. If the section still says
    'None yet', that line is replaced.
    """
    start = text.index('## 7+. Gap Additions')
    nxt = text.find('\n## ', start + 5)
    end = nxt + 1 if nxt != -1 else len(text)
    sec = text[start:end]
    nums = [int(n) for n in re.findall(r'^### 7\.(\d+) ', sec, re.M)]
    n0 = max(nums) if nums else 0
    out = []
    for i, b in enumerate(blocks, n0 + 1):
        assert b.startswith('### 7.N '), b[:40]
        out.append(b.replace('### 7.N ', f'### 7.{i} ', 1).rstrip('\n'))
    body = header + '\n\n' + '\n\n'.join(out) + '\n'
    if NONE_RE.search(sec):
        assert len(NONE_RE.findall(sec)) == 1
        new = NONE_RE.sub(lambda m: m.group(1) + body, sec)
    else:
        new = sec.rstrip('\n') + '\n\n' + body + ('\n' if nxt != -1 else '')
    return text[:start] + new + text[end:]
