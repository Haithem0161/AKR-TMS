#!/usr/bin/env python3
"""Checks the waiver register deploy/waivers.toml.

Fails when a waiver has passed its end date, lacks a field, runs longer than the limit, or when
backend/deny.toml ignores an advisory that no waiver covers.
"""

import datetime
import sys
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
REGISTER = ROOT / "deploy" / "waivers.toml"
DENY = ROOT / "backend" / "deny.toml"
FIELDS = ("id", "scanner", "component", "owner", "reason", "expires")
SCANNERS = ("cargo-deny", "cargo-audit", "trivy")
MAX_DAYS = 90


def main() -> int:
    today = datetime.date.today()
    waivers = tomllib.loads(REGISTER.read_text(encoding="utf-8")).get("waiver", [])
    failures = []
    for index, waiver in enumerate(waivers, 1):
        label = waiver.get("id", f"waiver {index}")
        missing = [field for field in FIELDS if not waiver.get(field)]
        if missing:
            failures.append(f"{label}: no {', '.join(missing)}")
            continue
        if waiver["scanner"] not in SCANNERS:
            failures.append(f"{label}: scanner `{waiver['scanner']}` is not one of {', '.join(SCANNERS)}")
        expires = waiver["expires"]
        if not isinstance(expires, datetime.date):
            failures.append(f"{label}: `expires` is not a date")
        elif expires < today:
            failures.append(f"{label}: expired on {expires}")
        elif (expires - today).days > MAX_DAYS:
            failures.append(f"{label}: ends more than {MAX_DAYS} days ahead")

    waived = {waiver.get("id") for waiver in waivers}
    ignored = tomllib.loads(DENY.read_text(encoding="utf-8")).get("advisories", {}).get("ignore", [])
    for entry in ignored:
        advisory = entry["id"] if isinstance(entry, dict) else entry
        if advisory not in waived:
            failures.append(f"{advisory}: ignored in backend/deny.toml with no waiver")

    for failure in failures:
        print(failure, file=sys.stderr)
    print(f"waivers: {len(waivers)} on the register, {len(failures)} findings")
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main())
