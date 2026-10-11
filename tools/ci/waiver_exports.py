#!/usr/bin/env python3
"""Writes what the release takes from the waiver register deploy/waivers.toml.

    waiver_exports.py vex <release> <out.json>      the release's VEX statement (CycloneDX)
    waiver_exports.py trivyignore <out.yaml>        the findings Trivy must not fail on
"""

import json
import sys
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
REGISTER = ROOT / "deploy" / "waivers.toml"


def waivers() -> list[dict]:
    return tomllib.loads(REGISTER.read_text(encoding="utf-8")).get("waiver", [])


def vex(release: str, out: Path) -> None:
    statement = {
        "bomFormat": "CycloneDX",
        "specVersion": "1.6",
        "version": 1,
        "metadata": {"component": {"type": "application", "name": "akr-tms", "version": release}},
        "vulnerabilities": [
            {
                "id": waiver["id"],
                "affects": [{"ref": waiver["component"]}],
                "analysis": {
                    "state": "not_affected",
                    "detail": waiver["reason"],
                },
                "properties": [
                    {"name": "akr:owner", "value": waiver["owner"]},
                    {"name": "akr:expires", "value": str(waiver["expires"])},
                ],
            }
            for waiver in waivers()
        ],
    }
    out.write_text(json.dumps(statement, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def trivyignore(out: Path) -> None:
    lines = ["vulnerabilities:"]
    for waiver in waivers():
        if waiver["scanner"] != "trivy":
            continue
        lines += [
            f"  - id: {waiver['id']}",
            f"    expired_at: {waiver['expires']}",
            f"    statement: {json.dumps(waiver['reason'])}",
        ]
    if len(lines) == 1:
        lines = ["vulnerabilities: []"]
    out.write_text("\n".join(lines) + "\n", encoding="utf-8")


def main() -> int:
    match sys.argv[1:]:
        case ["vex", release, out]:
            vex(release, Path(out))
        case ["trivyignore", out]:
            trivyignore(Path(out))
        case _:
            print(__doc__, file=sys.stderr)
            return 2
    return 0


if __name__ == "__main__":
    sys.exit(main())
