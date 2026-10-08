# AKR-TMS visual identity

Status: **locked** (2026-10-07). Proposed 2026-10-06, approved by the owner on 2026-10-07 and locked the same day
("design is perfect, lock it in"), together with the screen contrast tokens below. Change it only with the owner's
approval. Deliberately unlike the AIIB purple used in digital-banking-system.

## Idea: "the first ledgers"

Mesopotamia gave the world its first written records, and most of the earliest clay tablets from Uruk are accounts:
who owed what to whom. AKR-TMS is, at heart, a record of who moved what to whom and why it was checked. The identity
draws on two Iraqi sources, used abstractly (geometry and colour only, no figures, so it suits conventional and
Islamic banks alike):

- **Cuneiform wedges** for the symbol: three notched wedges, as a stylus leaves them in clay, all pointing at one centre:
  many signals, one point of attention.
- **The glazed bricks of the Ishtar Gate** for the colour: deep lapis blue with gold, on warm clay neutrals.

The result should feel calm, exact and local: a regulator-grade tool that is clearly made for Iraq rather than a
translated Western product.

## Symbol and wordmark

- Symbol: three cuneiform wedges converging on a centre, inside a softly rounded square (a clay tablet), gold on lapis;
  a one-colour lapis version for light backgrounds. Files: plan/assets/logo-symbol.svg, logo-symbol-mono.svg;
  preview plan/assets/logo-preview.png. Variants rejected: pinwheel (cluttered), stacked (reads as fast-forward),
  outward radial (reads as a car-maker star), tick (reads as a bird).
- Wordmark: "AKR" in IBM Plex Sans SemiBold, "-TMS" in IBM Plex Sans Regular, tight tracking. Arabic lockup uses the
  Latin name (it is a product name) with an Arabic descriptor in IBM Plex Sans Arabic: "نظام مراقبة المعاملات ومخاطر
  العملاء".
- Minimum clear space: the height of one wedge on every side.

## Colour

| Token | Hex | Use |
|---|---|---|
| Lapis Night | #0F2147 | Navigation, headings, dark surfaces |
| Ishtar Blue | #1E4FA0 | Primary actions, links, selected states (7.8:1 on white) |
| Glaze Teal | #1F8A8A (text: #177272) | Secondary data series, info accents |
| Babylon Gold | #C8A24B | Brand accent only: logo, key highlights, active nav marker (never a status colour) |
| Clay | #F5F0E6 | App and page background |
| Paper | #FFFFFF | Cards, tables, data surfaces |
| Ink | #18202F | Body text |
| Stone | #5B6474 | Secondary text |
| Sand line | #DDD6C9 | Borders and dividers |
| Risk high | #B3261E | High risk, overdue, true match |
| Risk medium | #B26A00 (fill #F2A33A) | Medium risk, due soon |
| Risk low | #2F7D4F | Low risk, cleared, reconciled |

Rules: gold is never used for status, so "medium risk" amber and the brand gold never compete; status colours always
come with a text label or icon (colour is never the only signal).

### Screen contrast tokens (locked with the identity, 2026-10-07)

The product screens meet WCAG 2.2 AA: 4.5:1 for text, 3:1 for control borders and graphics (research file 31
section 5.3, requirement UX-33). The brand colours above stay as they are; these tokens are used where a brand or
status colour would be too faint.

| Token | Hex | Use | Contrast |
|---|---|---|---|
| Teal text | #177272 | Teal text on Paper or Clay; Glaze Teal #1F8A8A stays for graphics | 5.70 on Paper, 5.02 on Clay |
| Risk medium text | #9C5D00 (or darker) | Amber text, also on its tint #FDEBD7; #B26A00 stays for large text and graphics | 5.28 on Paper, 4.65 on Clay, 4.53 on the tint |
| Risk low text | #2D794C | Green text on Clay and on its tint; #2F7D4F stays for graphics | 5.32 on Paper, 4.68 on Clay |
| Control border | #7F8695 | Borders of inputs, selects, text areas and other controls; Sand line stays for card edges and dividers | 3.65 on Paper, 3.22 on Clay |

Screen rules: Babylon Gold is never text, a border or an icon on a light surface (2.41:1 on Paper); it stays in the
logo and on the Lapis rail (6.57:1). The amber fill #F2A33A is a fill only, with labels in Ink (7.83:1). Neighbouring
chart segments are separated by a white gap and labelled directly.

## Type

- Interface: IBM Plex Sans Arabic (Arabic) and IBM Plex Sans (Latin), one family designed to sit together, both open
  source. IBM Plex Mono for identifiers, account numbers and amounts, with tabular figures.
- Plan PDF: IBM Plex Serif for body text (comfortable long reading), IBM Plex Sans for headings, tables and captions.

## Interface character (for the mockups)

- Light theme by default: clay background, white data cards, lapis navigation rail, gold active marker.
- Dense but quiet: thin sand borders instead of heavy shadows (controls use the control border), 8 px radius,
  generous line height, clear hierarchy.
- Every number that matters has a reason or a trend beside it; every alert shows its reason codes.
- Arabic-first: layouts are designed right-to-left and mirrored for English; the mockups in the plan show the English
  version because the plan is in English.
