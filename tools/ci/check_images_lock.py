#!/usr/bin/env python3
"""Fails when deploy/ names an image that deploy/images.lock does not pin.

Every `image:` of a compose file and every `FROM` or image `ARG` of a Dockerfile must name a repository
of the lock file. A reference with a digest must carry the digest of its line; a reference without one
must be a line marked `build`.
"""

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
LOCK = ROOT / "deploy" / "images.lock"
REFERENCE = re.compile(r"^\s*(?:image:|FROM|ARG \w+_IMAGE=)\s*(\S+)")


def lock_lines() -> dict[str, str]:
    pinned = {}
    for number, line in enumerate(LOCK.read_text(encoding="utf-8").splitlines(), 1):
        if not line.strip() or line.lstrip().startswith("#"):
            continue
        fields = line.split(None, 2)
        if len(fields) != 3:
            sys.exit(f"{LOCK}:{number}: expected name, digest and source")
        name, digest, _source = fields
        if digest != "build" and not re.fullmatch(r"sha256:[0-9a-f]{64}", digest):
            sys.exit(f"{LOCK}:{number}: `{digest}` is neither a sha256 digest nor `build`")
        if name in pinned:
            sys.exit(f"{LOCK}:{number}: `{name}` is listed twice")
        pinned[name] = digest
    return pinned


def references() -> list[tuple[Path, int, str]]:
    found = []
    files = [*ROOT.glob("deploy/compose/*.yaml"), *ROOT.glob("deploy/images/*/Dockerfile")]
    for path in sorted(files):
        for number, line in enumerate(path.read_text(encoding="utf-8").splitlines(), 1):
            match = REFERENCE.match(line)
            if match and not match.group(1).startswith("$"):
                found.append((path, number, match.group(1)))
    return found


def main() -> int:
    pinned = lock_lines()
    stages = {"build", "runtime"}
    failures = []
    for path, number, reference in references():
        if reference in stages:
            continue
        name, _, digest = reference.partition("@")
        name = re.sub(r":[^/]*$", "", name)
        where = f"{path.relative_to(ROOT)}:{number}"
        if name not in pinned:
            failures.append(f"{where}: `{name}` is not in deploy/images.lock")
        elif digest and pinned[name] != digest:
            failures.append(f"{where}: `{name}` is pinned to another digest in deploy/images.lock")
        elif not digest and pinned[name] != "build":
            failures.append(f"{where}: `{reference}` has no digest")
    for failure in failures:
        print(failure, file=sys.stderr)
    print(f"images.lock: {len(pinned)} images, {len(failures)} references not pinned")
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main())
