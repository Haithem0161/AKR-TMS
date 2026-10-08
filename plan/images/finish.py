#!/usr/bin/env python3
"""Finish the Codex mockups: draw the real AKR-TMS logo on the navigation rail and real Arabic text where the
generated image has empty boxes (image models garble both). Reads images/NN.png, writes images/final/NN.png."""
import io
import pathlib
import subprocess
import sys

import numpy as np
from PIL import Image, ImageDraw, ImageFont

HERE = pathlib.Path(__file__).resolve().parent
ASSETS = HERE.parent / "assets"
OUT = HERE / "final"
TILE = "#1A2E5C"          # tablet tile, slightly lighter than the rail
GOLD = "#C8A24B"
ARABIC_FONT = ASSETS / "fonts" / "ibm-plex-sans-arabic-arabic-600-normal.woff2"
LATIN_FONT = ASSETS / "fonts" / "ibm-plex-sans-latin-400-normal.woff2"

# Corrections to text the model rendered: (erase box, origin, size px, colour, new text).
TEXT_PATCHES = {
    # The liaison officers' monthly report goes to the MLRO by the 10th (CBI controls 2025, ch. 11), not to the CBI.
    "01-command-centre": [((1030, 484, 1330, 508), (1037, 482), 17.4, (14, 20, 44), "Branch liaison monthly reports")],
}

# Corrections fitted to the model's lettering: (erase box, top-left of the new ink, size px, colour, new text, weight,
# horizontal squeeze). The model's type is narrower than Plex Sans, so tile labels are drawn taller and squeezed.
FITTED_PATCHES = {
    # Match states in the AML/CFT Office's terms (true, potential, incorrect; research file 28 section 3.4).
    "03-screening": [
        ((210, 781, 412, 806), (215, 786), 16.8, (17, 22, 45), "Potential match: escalate", "500", 0.82),
        ((756, 781, 972, 806), (762, 787), 16.8, (17, 22, 45), "Incorrect match", "400", 0.82),
        # Pair memory applies to cleared (incorrect) matches only (research file 31 section 12.2).
        ((197, 929, 720, 951), (200, 933), 13, (17, 22, 45),
         "Remember for this pair (incorrect matches only); re-check if either record changes", "400", 1),
    ],
    # A threshold change needs a risk reason; "too many alerts" is a capacity reason (research file 20 L-18).
    "05-scenario-studio": [
        ((1108, 251, 1450, 276), (1112, 255), 15.8, (24, 30, 52), "Risk-based: 3-branch deposits found low risk", "400", 1),
    ],
}

# Checkboxes to show unticked: (erase box, box, corner radius, outline colour, outline width).
UNTICK = {
    "03-screening": [((164, 924, 193, 953), (167, 927, 189, 949), 4, (128, 136, 150), 2)],
}


def fitted_text(im: Image.Image, top_left, size_px, colour, text, weight, squeeze) -> None:
    """Draw `text` so its ink box starts at `top_left`, squeezed horizontally by `squeeze`."""
    font = ImageFont.truetype(str(ASSETS / "fonts" / f"ibm-plex-sans-latin-{weight}-normal.woff2"), size_px)
    l, t, r, b = font.getbbox(text)
    w, h = r - l + 4, b - t + 4
    mask = Image.new("L", (w, h), 0)
    ImageDraw.Draw(mask).text((2 - l, 2 - t), text, font=font, fill=255)
    if squeeze != 1:
        mask = mask.resize((max(1, round(w * squeeze)), h), Image.LANCZOS)
    im.paste(Image.new("RGB", mask.size, colour), (top_left[0] - 2, top_left[1] - 2), mask)

# Empty "Name in Arabic script" boxes in 03-screening.png: (left, top, right, bottom) of the border, and the text.
ARABIC = {
    "03-screening": [
        ((174, 178, 713, 249), "مصطفى حسن كاظم التميمي"),
        ((927, 178, 1490, 249), "مصطفي حسن كاظم التميمي"),
    ],
}


def logo_tile(size: int) -> Image.Image:
    """Render the symbol (three gold wedges) on a rounded tablet tile at `size` px, supersampled."""
    svg = (ASSETS / "logo-symbol.svg").read_text().replace('fill="#0F2147"', f'fill="{TILE}"')
    big = size * 4
    png = subprocess.run(["rsvg-convert", "-w", str(big), "-h", str(big)], input=svg.encode(),
                         capture_output=True, check=True).stdout
    return Image.open(io.BytesIO(png)).convert("RGBA").resize((size, size), Image.LANCZOS)


def rail_info(arr: np.ndarray):
    row = arr[980]
    rail_w = next(x for x in range(arr.shape[1]) if row[x].sum() > 250)
    rail_col = np.median(arr[600:1000, 3:rail_w - 3].reshape(-1, 3), axis=0)
    return rail_w, tuple(int(c) for c in rail_col)


def placeholder_box(arr: np.ndarray, rail_w: int, rail_col):
    reg = arr[0:105, 0:rail_w - 4]
    diff = np.abs(reg - np.array(rail_col)).sum(axis=2)
    m = diff > 22
    cols = np.where(m.sum(axis=0) > 25)[0]
    rows = np.where(m.sum(axis=1) > 25)[0]
    if len(cols) and len(rows):
        return int(cols.min()), int(rows.min()), int(cols.max()), int(rows.max())
    return None


def finish(name: str) -> None:
    src = HERE / f"{name}.png"
    im = Image.open(src).convert("RGB")
    arr = np.asarray(im).astype(int)
    rail_w, rail_col = rail_info(arr)
    box = placeholder_box(arr, rail_w, rail_col)
    draw = ImageDraw.Draw(im)
    if box:
        x0, y0, x1, y1 = box
        draw.rectangle((x0 - 4, y0 - 4, x1 + 4, y1 + 4), fill=rail_col)   # erase the placeholder
        cx, cy = (x0 + x1) / 2, (y0 + y1) / 2
        size = int(max(52, min(64, min(x1 - x0, y1 - y0))))
    else:
        cx, cy, size = rail_w / 2, 52, 58
    tile = logo_tile(size)
    im.paste(tile, (int(cx - size / 2), int(cy - size / 2)), tile)

    for erase, origin, size_px, colour, text in TEXT_PATCHES.get(name, []):
        draw.rectangle(erase, fill=tuple(int(c) for c in arr[erase[3] + 2, erase[2] - 10]))
        draw.text(origin, text, font=ImageFont.truetype(str(LATIN_FONT), size_px), fill=colour)

    for erase, top_left, size_px, colour, text, weight, squeeze in FITTED_PATCHES.get(name, []):
        draw.rectangle(erase, fill=tuple(int(c) for c in arr[erase[3] + 2, erase[2] - 10]))
        fitted_text(im, top_left, size_px, colour, text, weight, squeeze)

    for erase, rect, radius, outline, width in UNTICK.get(name, []):
        draw.rectangle(erase, fill=tuple(int(c) for c in arr[erase[3] + 2, erase[2] + 4]))
        draw.rounded_rectangle(rect, radius=radius, fill=(255, 255, 255), outline=outline, width=width)

    for (l, t, r, b), text in ARABIC.get(name, []):
        font = ImageFont.truetype(str(ARABIC_FONT), 30)
        bbox = draw.textbbox((0, 0), text, font=font, direction="rtl", language="ar")
        w, h = bbox[2] - bbox[0], bbox[3] - bbox[1]
        x = r - 18 - w - bbox[0]
        y = t + (b - t - h) / 2 - bbox[1]
        draw.text((x, y), text, font=font, fill="#18202F", direction="rtl", language="ar")

    OUT.mkdir(exist_ok=True)
    im.save(OUT / f"{name}.png", optimize=True)
    print(f"{name}: rail {rail_w}px, placeholder {box}, logo {size}px")


if __name__ == "__main__":
    names = sys.argv[1:] or sorted(p.stem for p in HERE.glob("0[1-8]-*.png") if "-v1" not in p.stem)
    for n in names:
        finish(n)
