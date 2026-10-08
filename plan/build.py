#!/usr/bin/env python3
"""Build the AKR-TMS plan PDF: plan.md -> plan.html -> AKR-TMS-plan.pdf (Chromium headless).

Usage: build.py [en|ar] [--html-only]. "en" (default) builds the English plan; "ar" builds the Arabic edition from
plan-ar.md (right to left, extra stylesheet assets/plan-ar.css).
"""
import pathlib
import re
import subprocess
import sys

import markdown

ROOT = pathlib.Path(__file__).resolve().parent
CHROMIUM = "/snap/bin/chromium"

# HTML is kept next to assets/ and images/ so relative paths resolve. keep_together: Chromium breaks hyphenated
# names at the hyphen ("AKR-" / "TMS"), so each one is wrapped in a no-wrap span (class "nw" in plan.css).
EDITIONS = {
    "en": dict(src="plan.md", html="plan.html", pdf="AKR-TMS-plan.pdf", html_attrs='lang="en"',
               title="AKR-TMS: a plan for Iraqi banks", css=["assets/plan.css"],
               keep_together=["AKR-TMS", "World-Check", "Pio-Tech"]),
    "ar": dict(src="plan-ar.md", html="plan-ar.html", pdf="AKR-TMS-plan-ar.pdf", html_attrs='lang="ar" dir="rtl"',
               title="AKR-TMS: خطة للمصارف العراقية", css=["assets/plan.css", "assets/plan-ar.css"],
               keep_together=["AKR-TMS", "World-Check", "Pio-Tech"]),
}
LANG = next((a for a in sys.argv[1:] if a in EDITIONS), "en")
ED = EDITIONS[LANG]
SRC = ROOT / ED["src"]
OUT_HTML = ROOT / ED["html"]
OUT_PDF = ROOT / ED["pdf"]

TEMPLATE = """<!doctype html>
<html {html_attrs}>
<head>
<meta charset="utf-8">
<title>{title}</title>
{css}
</head>
<body>
{body}
</body>
</html>
"""


def keep_together(body: str, names: list[str]) -> str:
    """Wrap each name in <span class="nw"> in text only: never inside a tag's attributes or an inline SVG."""
    if not names:
        return body
    pat = re.compile("|".join(map(re.escape, names)))
    out, in_svg = [], False
    for part in re.split(r"(<[^>]+>)", body):
        if part.startswith("<"):
            in_svg = part.startswith("<svg") or (in_svg and not part.startswith("</svg"))
            out.append(part)
        else:
            out.append(part if in_svg else pat.sub(lambda m: f'<span class="nw">{m.group(0)}</span>', part))
    return "".join(out)


def build_html() -> None:
    text = SRC.read_text(encoding="utf-8")
    body = markdown.markdown(
        text,
        extensions=["tables", "attr_list", "md_in_html", "sane_lists", "smarty"],
        extension_configs={"smarty": {"smart_angled_quotes": False}},
        output_format="html5",
    )
    body = keep_together(body, ED.get("keep_together", []))
    css ="\n".join(f'<link rel="stylesheet" href="{href}">' for href in ED["css"])
    OUT_HTML.write_text(TEMPLATE.format(html_attrs=ED["html_attrs"], title=ED["title"], css=css, body=body),
                        encoding="utf-8")
    print(f"HTML: {OUT_HTML}")


def build_pdf() -> None:
    cmd = [
        CHROMIUM, "--headless", "--disable-gpu", "--no-pdf-header-footer",
        "--run-all-compositor-stages-before-draw", "--virtual-time-budget=15000",
        f"--print-to-pdf={OUT_PDF}", OUT_HTML.as_uri(),
    ]
    res = subprocess.run(cmd, capture_output=True, text=True)
    if not OUT_PDF.exists() or res.returncode != 0:
        print(res.stderr[-2000:], file=sys.stderr)
        sys.exit(1)
    pages = subprocess.run(["pdfinfo", str(OUT_PDF)], capture_output=True, text=True).stdout
    n = next((l.split()[-1] for l in pages.splitlines() if l.startswith("Pages:")), "?")
    print(f"PDF: {OUT_PDF} ({n} pages)")


if __name__ == "__main__":
    build_html()
    if "--html-only" not in sys.argv:
        build_pdf()
