# Design System — "The First Ledgers" (AKR-TMS)

Status: **planning draft 2026-10-07**. The visual identity it implements is **locked**
([../plan/identity.md](../plan/identity.md), approved and locked by the owner on 2026-10-07). This document turns that
identity into tokens, type, components, screen specs and accessibility rules for the build. It follows the shape of
the DBS design system (`docs/DESIGN-SYSTEM.md` in the DBS repository) and keeps that system's rules that were never
about the palette.

Sources: the identity (locked); research file 31 sections 4 (Arabic-first), 5 (accessibility and measured contrast),
10 (screen inventory), 11 (interaction rules IR-01 to IR-93) and 12 (mockup changes); requirements UX-01 to UX-71;
MODULES.md section 21 (frontend architecture); tech-stack.md (frontend).

**Authority.**
1. Where this document and the identity disagree on a brand value (a hex, a face, the mark), the identity wins and
   this document is corrected. Identity values change only with the owner's approval.
2. Values marked **Derived** below are not in the identity. They are measured, they stay inside its hues, and they
   are confirmed by the owner in the phase 08 design-token task (open item DS-O2). Until then they are the working
   values.
3. Values marked **Proposed** (the dark theme, section 2.8) are not used in a release until the owner approves them
   (open item DS-O1).
4. Everything else here (components, layout, rules) changes through the normal planning flow: edit this document in
   the same change as the code.

## 0. What the DBS taught, and what is kept

The DBS shipped twelve phases on a bespoke token layer and replaced it in phase 12.5, for a reason worth repeating:
**the tokens were fine and the component layer never existed.** Eleven shared components carried forty screens, so
every screen re-invented its own card, input and table, and the product drifted apart. A design system is the
components that make the palette unavoidable, not the palette.

AKR-TMS therefore starts where the DBS ended:

- **Untitled UI React (MIT tier), vendored** at `frontend/src/uui/`, exactly as in the DBS, as the component layer
  (React Aria underneath, which tech-stack.md names). Its component anatomy, spacing scale and focus behaviour are
  adopted. Its colours, radii and shadows are **redirected to the AKR-TMS identity** through one bridge file
  (`frontend/src/styles/akr.css`, section 6), so a re-pull from upstream is a copy plus the RTL migration, not a merge.
- **A shared component layer** at `frontend/src/components/` wraps the vendored tree wherever a rule must hold on
  every screen (section 7). A rule applied at forty call sites is a rule missed at some of them.
- The DBS i18n kit (`direction`, `bidi`, `datetime`, `fold`, `AriaLocale`) is copied and extended (section 3.5).

## 1. Principles

1. **Calm, exact, local.** A regulator-grade tool that is clearly made for Iraq: clay page, white data cards, a lapis
   rail, one accent. Thin lines instead of heavy shadows. No decoration that carries no information. Geometry and
   colour only, no figures, so the product suits conventional and Islamic banks alike (identity).
2. **Exceptions are loud, health is silent.** Status appears only where something needs attention (overdue, a true
   match, a failed check). A healthy row is plain text. A badge counts items that need action, not items that exist
   (IR-64).
3. **Arabic is the original.** Every screen is designed right to left and mirrored for English. Every string ships in
   Arabic and English from its first commit. Logical CSS only. Western digits in data (UX-18).
4. **Status is never colour alone.** Every status pairs its colour with a word. This is load-bearing: the red, amber
   and green triad cannot be separated for colour-blind readers by any choice of shades (section 2.4), so the word is
   the signal and the colour is a reinforcement (UX-34).
5. **Evidence before score; reasons beside numbers.** Every number that matters has its reason or its trend beside
   it; every alert shows its reason codes; a score is never shown alone (identity; UX-08, UX-42).
6. **Neutral by construction.** Screens seen by branch staff, operations staff and notification channels carry no
   AML vocabulary, no thresholds and no outcomes. The tipping-off rule is a property of the components, not a thing
   each screen remembers (section 11; UX-02, UX-69).
7. **Honest about time and gaps.** Every data screen shows its data-as-of time; degraded figures say so beside the
   figure; a refused request never looks like an empty list, and a suspicion-tier refusal looks exactly like "not
   found" (IR-02, IR-70, IR-74).
8. **Never render a control that cannot work.** The rail and every action come from the role and the API's
   affordances (IR-01; API.md section 1).

## 2. Colour

### 2.1 Token layers

Colours have three layers, as in the DBS:

```
identity value       #1E4FA0 Ishtar Blue, #F5F0E6 Clay, #B3261E Risk high ...    (plan/identity.md)
  ↓
semantic token       --color-action, --color-page, --color-risk-high, --color-control ...   (akr.css)
  ↓
component code       bg-action, bg-page, text-risk-high, border-control
```

**Component code names semantic tokens only.** A literal hex, an `rgb()` or a raw Untitled UI ramp step
(`bg-blue-600`) in `frontend/src/components/` or in a screen is a defect, caught by lint (section 6.4).

### 2.2 Brand (locked)

From the identity, unchanged.

| Identity name | Hex | Semantic token | Use |
|---|---|---|---|
| Lapis Night | `#0F2147` | `--color-lapis` | Navigation rail, headings, dark surfaces |
| Ishtar Blue | `#1E4FA0` | `--color-action` | Primary actions, links, selected states, focus ring (7.83:1 on Paper, 6.90 on Clay) |
| Glaze Teal | `#1F8A8A` | `--color-teal` | Secondary data series, info accents (graphics only, 4.15 on Paper) |
| Babylon Gold | `#C8A24B` | `--color-gold` | Logo, active-nav marker on the rail. **Never a status colour** |
| Clay | `#F5F0E6` | `--color-page` | App and page background |
| Paper | `#FFFFFF` | `--color-paper` | Cards, tables, data surfaces |
| Ink | `#18202F` | `--color-ink` | Body text (16.32 on Paper, 14.37 on Clay) |
| Stone | `#5B6474` | `--color-stone` | Secondary text (5.97 on Paper, 5.25 on Clay) |
| Sand line | `#DDD6C9` | `--color-line` | Card edges and dividers. **Never a control border** (1.44 on Paper) |
| Risk high | `#B3261E` | `--color-risk-high` | High risk, overdue, true match (text and graphics) |
| Risk medium | `#B26A00` | `--color-risk-medium` | Medium risk, due soon (large text and graphics only) |
| Risk medium fill | `#F2A33A` | `--color-risk-medium-fill` | Fills only, labels in Ink (7.83:1) |
| Risk low | `#2F7D4F` | `--color-risk-low` | Low risk, cleared, reconciled (graphics) |

### 2.3 Screen contrast tokens (locked) and derived tints

The four screen tokens were locked with the identity. The tints, hover and selection surfaces are **Derived**
(measured 2026-10-07 with the WCAG 2.x formula; script in section 2.9).

| Token | Hex | Status | Use | Measured |
|---|---|---|---|---|
| `--color-teal-text` | `#177272` | Locked | Teal text on Paper or Clay | 5.70 Paper, 5.02 Clay, 4.85 on teal tint |
| `--color-risk-medium-text` | `#9C5D00` | Locked | Amber text, also on its tint | 5.28 Paper, 4.65 Clay, 4.53 on tint |
| `--color-risk-low-text` | `#2D794C` | Locked | Green text on Clay and on its tint | 5.32 Paper, 4.68 Clay, 4.62 on tint |
| `--color-control` | `#7F8695` | Locked | Borders of inputs, selects, text areas, checkboxes, toggles | 3.65 Paper, 3.22 Clay, 3.28 on selected row |
| `--color-risk-high-tint` | `#FBE1E1` | Derived (research 31 s.5.3) | Background of a high-risk badge or row marker | Risk high on it 5.28; Stone 4.82 |
| `--color-risk-medium-tint` | `#FDEBD7` | Locked (identity) | Background of a medium badge | Amber text 4.53; Stone 5.13 |
| `--color-risk-low-tint` | `#E6F2EA` | Derived | Background of a low or cleared badge | Green text 4.62; Stone 5.19 |
| `--color-teal-tint` | `#E0F0F0` | Derived | Info notes (draft, help, "written automatically") | Teal text 4.85 |
| `--color-tier-tint` | `#E8ECF4` | Derived | Background of the sensitivity-tier badge (section 7.4) | Ink 13.79; Stone 5.04 |
| `--color-selected` | `#EEF3FB` | Derived | Selected row, selected list item | Ishtar 7.03; Stone 5.35; control border 3.28 |
| `--color-hover` | `#F0EBDF` | Derived | Row and item hover on Clay | Stone 5.02 |
| `--color-hover-paper` | `#FAF8F3` | Derived | Row and item hover on Paper | Stone above 5.5 |
| `--color-on-rail` | `#FFFFFF` | Derived | Rail labels on Lapis | 15.80 |
| `--color-on-rail-muted` | `#B8C3D9` | Derived | Rail group headings, collapsed hints | 8.91 |
| `--color-rail-hover` | `#1A3060` | Derived | Rail item hover and pressed | White 12.84; Gold marker 5.33 |

### 2.4 Status and the limit of colour

The status hues come from the identity. Light mode uses the locked text tokens wherever status is written as text,
because this product writes status as words:

| Role | Text token | Graphic token | Tint | Words it carries (examples) |
|---|---|---|---|---|
| High | `risk-high` | `risk-high` | `risk-high-tint` | High risk; Overdue; True match; Failed |
| Medium | `risk-medium-text` | `risk-medium` (or the fill) | `risk-medium-tint` | Medium risk; Due soon; Potential match; Degraded |
| Low | `risk-low-text` | `risk-low` | `risk-low-tint` | Low risk; Cleared; Match (reconciliation) |
| Info | `teal-text` | `teal` | `teal-tint` | Draft; Written automatically; Awaiting information |
| Neutral | `stone` | `control` | `paper` with a `line` edge | Incorrect match; Claimable; Paused; Not comparable |

**Colour-vision separation fails and cannot be fixed by choosing shades.** Red, amber and green is the
colour-blind-hostile triad; the DBS measured it and every step combination it tried left green and amber below
the separation target for protan and deutan readers. The same holds for these hues. So principle 4 is mandatory:

> Every status indicator pairs its colour with a **word**. A badge, dot or icon without its word is an accessibility
> defect, not a style choice. An icon alone does not discharge this. Neither does a coloured dot. The word does.

### 2.5 Semantic mapping

| State | Treatment |
|---|---|
| Overdue, breached clock, failed check, true match, high risk | High: badge with word; on rows, a word in `risk-high`, never a whole red row |
| Due soon (inside the warning band of its clock) | Medium due-text only ("due in 4 h"), no badge |
| On track | Nothing. Plain text |
| Potential match, degraded figure, medium risk | Medium: badge with word; degraded figures carry the word "degraded" beside the figure (IR-74) |
| Incorrect match, claimable, paused, not comparable | Neutral badge (Stone on Paper, `line` edge) |
| Cleared, reconciled, chain verified | Low, shown only where a positive confirmation is the point (reconciliation table, chain verifier); never on every healthy row |
| Restricted or suspicion tier | **Not a status colour.** Lock icon plus a word on `tier-tint` (section 7.4) |
| Draft, assistant output, help notes | Info: teal text on teal tint, with the word "Draft" |
| System health (platform health only) | Low dot plus the word "Healthy"; never shown to business users |
| Priority class | A word with its code, never a colour (section 7.5) |

### 2.6 Forbidden pairs

Enforced by the token check (section 2.9) and by lint (section 6.4):

1. Babylon Gold as text, border or icon on any light surface (2.41 on Paper, 2.1 on Clay). Gold appears only in the
   logo and as the active marker on the Lapis rail (6.57:1). The utilities `*-gold` are allowed in two files only:
   `components/shell/Rail.tsx` and `components/brand/Logo.tsx`.
2. The amber fill `#F2A33A` as text or as a boundary (2.08 on Paper). It is a fill, with Ink labels.
3. Glaze Teal `#1F8A8A`, Risk medium `#B26A00` and Risk low `#2F7D4F` as normal-size text. Use the `-text` tokens.
   (`text-teal`, `text-risk-medium` and `text-risk-low` fail lint; `text-risk-high` is allowed.)
4. Sand line as the border of any control. Controls use `border-control`.
5. Status colour as the only difference between two things (section 2.4).
6. Neighbouring chart segments that differ only by hue (blue and teal 1.9:1, teal and amber 2.0:1, amber and red
   3.1:1). Segments are separated by a 2 px Paper gap and labelled directly (section 7.15).

### 2.7 Data colours

- **Quantity** is one hue: Ishtar Blue, solid, length carrying the value (the DBS reports rule). No fading by
  position; a faded bar draws equal values at different strengths.
- **Two series** (for example money in against money out, or actual against expected): Ishtar Blue solid and Glaze
  Teal, the second also dashed, so the pair separates without colour.
- **Reference lines** (declared turnover, segment band, threshold): Stone, dashed, labelled at the line end.
- **Seasonal bands** (Ramadan, Eid, Muharram, Arbaeen) on timelines: `tier-tint` blocks with a text label (UX-07,
  UX-22).
- **Risk distributions** (stacked by level): the status graphic colours, 2 px Paper gaps, values printed inside or
  beside each segment, legend in words.
- **Network graph**: nodes Paper with an Ink outline; the subject node Lapis fill with white label; flagged nodes add
  a `risk-high` outline and a word or icon label (the outline alone is not enough); edges Stone, thickness by amount
  with the amount on hover and in the node and link table.

### 2.8 Dark theme (Proposed, not locked)

The identity defines a light theme ("light theme by default"). MODULES.md section 21.7 asks for a dark theme from
the same tokens with WCAG 2.2 AA in both. The values below are a **proposal**: they keep the identity's hues (lapis
surfaces, clay-tinted text, the same status hues lifted for dark) and were measured on 2026-10-07. They ship only
after the owner approves them (DS-O1); until then the theme switch is hidden and every account gets light.

| Token | Proposed dark | Measured |
|---|---|---|
| `--color-page` | `#0A1428` | |
| `--color-paper` (cards) | `#111E38` | |
| raised surface (menus, dialogs) | `#182A4D` | |
| `--color-ink` | `#ECE7DC` | 14.89 page, 13.45 card, 11.53 raised |
| `--color-stone` | `#A8B0BF` | 8.42 page, 7.60 card, 6.52 raised |
| `--color-action` (links, text) | `#8FB0F0` | 7.61 card, 6.52 raised |
| primary button fill | `#2F5BAA` | white on it 6.57 |
| `--color-line` | `#26365A` | decorative only |
| `--color-control` | `#6E7A96` | 3.86 card, 4.27 page |
| `--color-risk-high` | `#FF8A80` on tint `#3A1A1F` | 7.26 card, 6.83 on tint |
| `--color-risk-medium-text` | `#F2A33A` on tint `#36270F` | 7.95 card, 6.93 on tint |
| `--color-risk-low-text` | `#6CC28D` on tint `#10301F` | 7.69 card, 6.65 on tint |
| `--color-teal-text` | `#5CC4C4` | 8.01 card |
| `--color-tier-tint` | `#24365E` | Ink 9.65 |
| `--color-selected` | `#1B3260` | Ink 10.19, action 5.76 |
| rail | `#0F2147` (Lapis Night, unchanged) with a `line` edge | white 15.80, gold 6.57 |

Gold stays a brand accent in dark mode too (6.89 on the dark card, but it is still never status, text or a border
outside the rail and the logo). The dark theme is its own palette, never an inversion (DBS principle).

### 2.9 Token file and contrast check

The token values live in one file, `frontend/src/styles/tokens.json` (name, light, dark, status Locked, Derived or
Proposed). `akr.css` is generated from it, and a CI check (`scripts/check-contrast.mjs`) asserts every required pair:

- text tokens at least 4.5:1 against every surface they are allowed on (Paper, Clay, their tint, selected, hover);
- control border, focus ring and graphic tokens at least 3:1 against Paper and Clay;
- the forbidden pairs of section 2.6 stay below their thresholds, so nobody "fixes" a rule by changing a colour;
- a Locked value differs from `plan/identity.md` → the check fails.

Re-run it whenever a value changes. Do not re-tune by eye (UX-33).

## 3. Typography

### 3.1 Faces

| Role | Face | Notes |
|---|---|---|
| Arabic (and Sorani Kurdish) | **IBM Plex Sans Arabic** 400 / 500 / 600 | Vendored with Fontsource; the subset covers all ten Kurdish letters, Persian kaf and yeh, harakat and tatweel (research 31 s.4.3) |
| Latin and **all digits** | **IBM Plex Sans** 400 / 500 / 600 | `latin` **and `latin-ext`** subsets, so ş, ğ, İ and transliteration marks (ā ī ū ḥ ṣ ḍ ṭ ẓ ʿ ʾ) do not fall back (UX-26) |
| Identifiers, account numbers, amounts | **IBM Plex Mono** 400 / 500 | Tabular; `latin` and `latin-ext` |
| Documents (PDF) | The same three faces | Installed in the Gotenberg image; no other face in product output |

The plan PDF's IBM Plex Serif is a leadership-document face and does not appear in the product.

**Never a runtime font request** (no egress: bank data never leaves the bank and nothing is fetched from outside, README non-negotiable 2; IR-83). All faces are npm-vendored and
bundled. A glyph-coverage test renders every letter listed in research file 22 (Arabic, Kurdish, Persian, Turkish and
transliteration letters) and fails on any fallback (UX-26).

### 3.2 One stack

```css
--font-sans: "IBM Plex Sans", "IBM Plex Sans Arabic", system-ui, sans-serif;
--font-mono: "IBM Plex Mono", "IBM Plex Sans Arabic", ui-monospace, monospace;
```

IBM Plex Sans has no Arabic glyphs, so the browser falls through per code point. As in the DBS (which used Inter
the same way), two consequences follow and both are wanted:

- **Every digit renders in IBM Plex Sans, in both languages.** Plex Sans digits are all 600/1000 em wide, tabular by
  default, so a column of reference numbers stays aligned on an Arabic screen. The Arabic-Indic digits of Plex Sans
  Arabic are proportional (263 to 630 units) and would break columns, which is one reason data never uses them.
- **The space character comes from Plex Sans in Arabic too** (U+0020 is Latin). Known and accepted.

Mono is used, per the identity, for identifiers, account numbers, IBANs, reference numbers and amounts. Counts and
percentages use Plex Sans with `tabular-nums`.

### 3.3 Scale

A 14 px product (dense data). Latin line heights follow the Untitled UI scale; **Arabic line heights are raised**
under `:where([dir="rtl"])` only, because Arabic marks above and below the letter (hamza, shadda, the dots of taa and
yaa, kasra) make the inked line taller than the Latin box (research 31 s.4.4; W3C ALREQ 7.4).

| Token | Size | Latin leading | Arabic leading | Use |
|---|---|---|---|---|
| `text-xs` | 12 px | 18 | 20 | Meta lines, captions, table footnotes (12 px is the floor) |
| `text-sm` | 14 px | 20 | 23 | **Body and table text (default)** |
| `text-md` | 16 px | 24 | 27 | Branch window body; reading pages; dialog text |
| `text-lg` | 18 px | 28 | 30 | Section titles |
| `text-xl` | 20 px | 30 | 32 | Page titles |
| `text-display-xs` | 24 px | 32 | 36 | Case and customer headers |
| `text-display-sm` | 30 px | 38 | 44 | KPI figures |
| `text-display-md` | 36 px | 44 | 52 | Command-centre headline figures |

Starting values; the leading is verified in the RTL spike against marked text (shadda with kasra on the line below)
and adjusted per token, never globally.

### 3.4 Arabic rules

- **No letter-spacing on Arabic, ever.** The bridge zeroes every `--tracking-*` token under `[dir="rtl"]` (the DBS
  enforced this after measuring tiles drawing Arabic labels at 0.3 px).
- **No uppercase micro-labels in Arabic.** The mockups' 11 to 12 px uppercase labels ("DATA COVERAGE") are an
  English-only style under `html[lang="en"]`; in Arabic they become sentence case, weight 600, 12 px or more.
- **No italics on Arabic**, and no fake bold. Emphasis is weight (500 or 600) or colour. Italics are for Latin only.
- **Mixed lines** (a name in both scripts, an amount in a sentence) are checked for baseline and x-height alignment
  per context.
- Legal-form words in entity names (شركة، مؤسسة، المحدودة، Co., Ltd) are set at weight 400 when the rest of the name
  is 500, so the distinguishing words read first.

### 3.5 Numbers, dates, money and plurals

The i18n kit (copied from the DBS, extended) owns these; screens never format by hand.

| Kind | Rule | Kit function |
|---|---|---|
| Digits | Western 0-9 in all data, tables, identifiers, amounts and dates, in both languages. A per-bank and per-user setting may show Arabic-Indic digits **in prose only** | `digits()` |
| Input | Every number and search box accepts Western, Arabic-Indic and Persian digits, normalises them and shows the understood value (IR-50) | `westernDigits()` |
| Date and time | `YYYY-MM-DD HH:MM`, Baghdad local time, isolated as one left-to-right run; no `toLocaleString`, no native date inputs (UX-21) | `formatDateTime()` |
| Month names in prose | `ar-IQ` names (كانون الثاني، شباط ...) in reports and the board pack, switchable per bank | `monthName()` |
| Hijri | Display and entry only, labelled with the calendar variant, never a clock or comparison (UX-22) | `formatHijri(variant)` |
| Money | Figure in Mono plus the ISO code (IQD, USD) as one isolate; decimals from the currency table; in KPI tiles "مليون" and "مليار" with correct plural forms (UX-23) | `formatMoney()`, `formatCompact()` |
| Counted phrases | ICU plural rules with the six Arabic forms (zero, one, two, few, many, other); keys held in state, never concatenated sentences (UX-24) | i18next ICU |
| Ranges | Rendered in one order everywhere (left to right), isolated as a whole (research 31 s.4.2 rule 1; the preference is a usability-test item, DS-O4) | `formatRange()` |
| Numeric expressions | Phone, IBAN, reference, amount with code, date-time: wrapped in LRI and PDI **inside the value**; names in FSI and PDI (UX-19) | `ltr()`, `auto()` |
| Copy | Copy buttons copy the raw value without isolates; searches and equality checks strip them (IR-55) | `raw()` |

Additions to the DBS kit: Kurdish and Persian folds, ZWNJ repair, bidi-control scanning of incoming text (flagged as
data quality, UX-20), six-form plurals, the Hijri formatter with variant labels, the seasonal calendar, and the digit
preference.

## 4. Shape, depth, spacing, layout

### 4.1 Shape and depth

- **Radius**: 8 px for cards, panels, dialogs, menus, inputs and buttons (identity). Badges and chips are fully
  rounded (pill). Tables have square inner cells inside an 8 px card.
- **Depth**: thin `line` borders instead of shadows. No shadow on cards or rows. One small shadow only on floating
  layers (menus, popovers, dialogs, the step-up dialog) so they separate from the page. No gradients, glows or
  blurred glass anywhere.
- **No nested borders** (a bordered thing inside a bordered thing). Use spacing or a background shift (Paper on Clay).

### 4.2 Spacing and density

Tailwind's 4 px scale through Untitled UI's `--spacing`. Three densities (IR-13, UX-15), chosen per user (X4) with a
role default:

| Density | Row height | Control height | Default for |
|---|---|---|---|
| Compact | 32 px | 32 px | Analysts, screening specialists, data stewards |
| Standard | 40 px | 40 px | MLRO, compliance head, configuration, administration |
| Relaxed | 48 px | 44 px | Branch window, operations-task inbox |

Every interactive target is at least 24 by 24 CSS px, including icon buttons in compact rows (WCAG 2.5.8; UX-38).

### 4.3 Layout

- **App shell**: rail on the start side (right in Arabic), 240 px with labels, collapsible to 64 px (icons with
  tooltips and accessible names; the labelled form is the default). Header 56 px. Content on Clay with Paper cards.
- **Breakpoints**: Tailwind defaults. Workbenches are designed at 1440 px and must work at 1280 px. Below 1024 px the
  rail becomes a drawer. Every screen reflows down to 320 CSS px for 400 percent zoom (WCAG 1.4.10): the page never
  scrolls sideways; wide tables scroll inside their own container with a sticky first column.
- **Reading width**: help articles, report narratives and the board view cap at 72 characters per line.
- **DOM order is the reading order.** No CSS `order` tricks to place the rail or the decision panel; mirroring is by
  logical properties only (WCAG 1.3.2, 2.4.3).

## 5. Iconography

- `@untitledui/icons` (as the DBS), 24 viewBox, stroke `currentColor`. Sizes: `size-5` default, `size-4` small,
  `size-3` inline in badges.
- **Directional icons flip** in RTL through `data-rtl-flip`, backed by
  `[data-rtl-flip]:dir(rtl) { transform: scaleX(-1); }`. That rule is the only place `scaleX(-1)` is allowed.
  Symmetric icons never flip; neither do play, download, upload, external-link marks, or the lock (UX-30).
- Progress bars and sliders mirror; **graph geometry does not** (a network is not a sentence); chart time axes follow
  the RTL spike (DS-O3).
- Fixed meanings: lock = sensitivity tier (always with a word); clock = a running clock (always with its kind and
  target); shield-check = verified; paperclip = evidence attached. One icon per meaning across the product.
- **No emoji anywhere in the product.** No figurative illustrations (people, animals, buildings). Empty states use the
  wedge geometry of the mark or a plain icon.

## 6. Tailwind v4 mapping

### 6.1 Files

CSS-first, no `tailwind.config.js`, **one entry**:

```
frontend/src/styles/theme.css      @import "tailwindcss";  @import "./akr.css";     <- the entry
  └── frontend/src/styles/akr.css  @import "../uui/theme.css";  + the AKR bridge (generated from tokens.json)
```

A second stylesheet imported on its own registers its `@theme` tokens and generates **no utilities** (utility
generation needs `@import "tailwindcss"` in that file): the symptom is a token that resolves in DevTools and a class
that does nothing. The DBS hit this; one entry prevents it.

### 6.2 The bridge

`akr.css` does four things, each with its reason written beside it:

1. **Redirects the Untitled UI ramps** to the identity, so every vendored component picks up AKR colours without
   being edited. The ramps are anchored, not invented:

   | Untitled UI ramp | Anchors (step = identity value) | Other steps |
   |---|---|---|
   | `brand` | 50 = `#EEF3FB`, 600 = `#1E4FA0` Ishtar, 950 = `#0F2147` Lapis | Generated in OKLCH at the Ishtar hue, chroma tapering at both ends |
   | `gray` | 50 = `#F5F0E6` Clay, 200 = `#DDD6C9` Sand, 400 = `#7F8695` control, 600 = `#5B6474` Stone, 900 = `#18202F` Ink | Interpolated between anchors in OKLCH |
   | `error` | 50 = `#FBE1E1`, 600 = `#B3261E` | Generated at the hue |
   | `warning` | 50 = `#FDEBD7`, 400 = `#F2A33A`, 600 = `#B26A00`, 700 = `#9C5D00` | Generated at the hue |
   | `success` | 50 = `#E6F2EA`, 600 = `#2F7D4F`, 700 = `#2D794C` | Generated at the hue |

   Untitled UI's semantic aliases resolve through these ramps untouched, except three set explicitly:
   `--color-bg-primary` = Paper, `--color-bg-secondary` = Clay, `--color-border-primary` = `#7F8695` (its control
   border; it ships a lighter grey that fails 3:1).
2. **Adds the AKR semantic tokens** of section 2 under `@theme` (`--color-page`, `--color-paper`, `--color-ink`,
   `--color-stone`, `--color-line`, `--color-control`, `--color-action`, `--color-lapis`, `--color-gold`,
   `--color-teal`, `--color-teal-text`, `--color-teal-tint`, `--color-risk-*`, `--color-tier-tint`,
   `--color-selected`, `--color-hover`, `--color-hover-paper`, `--color-on-rail`, `--color-on-rail-muted`,
   `--color-rail-hover`), so component code reads in product terms (`text-risk-high`, `bg-tier-tint`).
3. **Sets shape**: `--radius-*` so the default control and card radius is 8 px; shadows on cards and inputs set to
   none; the overlay shadow kept for floating layers.
4. **Sets type**: the font stacks of section 3.2, the Arabic leading under `:where([dir="rtl"])`, and every
   `--tracking-*` zeroed under `[dir="rtl"]`.

Sketch (values from tokens.json; the generated file is the authority):

```css
@theme {
  --font-sans: "IBM Plex Sans", "IBM Plex Sans Arabic", system-ui, sans-serif;
  --font-mono: "IBM Plex Mono", "IBM Plex Sans Arabic", ui-monospace, monospace;
  --color-page: #F5F0E6;   --color-paper: #FFFFFF;   --color-ink: #18202F;   --color-stone: #5B6474;
  --color-line: #DDD6C9;   --color-control: #7F8695; --color-action: #1E4FA0; --color-lapis: #0F2147;
  --color-gold: #C8A24B;   --color-teal: #1F8A8A;    --color-teal-text: #177272;
  --color-risk-high: #B3261E;        --color-risk-high-tint: #FBE1E1;
  --color-risk-medium: #B26A00;      --color-risk-medium-text: #9C5D00;
  --color-risk-medium-fill: #F2A33A; --color-risk-medium-tint: #FDEBD7;
  --color-risk-low: #2F7D4F;         --color-risk-low-text: #2D794C;  --color-risk-low-tint: #E6F2EA;
  --radius-md: 8px;
}
:where([dir="rtl"]) { --text-sm--line-height: 23px; /* ... per section 3.3 */ }
[dir="rtl"] { --tracking-tight: 0; --tracking-wide: 0; /* every tracking token */ }
```

### 6.3 Themes

The vendored `theme.css` keys dark mode off `[data-theme="dark"]` (one selector, as in the DBS). The theme
controller stamps `data-theme` on `<html>` before first paint. **Light is the product default**: a stored choice
wins, and an unconfigured account gets light; `prefers-color-scheme` is not consulted. While DS-O1 is open the
choice is not offered.

### 6.4 Lint (CI)

- **Logical properties only**: `ms-*`, `me-*`, `ps-*`, `pe-*`, `start-*`, `end-*`, `text-start`, `text-end`,
  `border-s`, `border-e`, `rounded-s-*`, `rounded-e-*`. `ml-`, `mr-`, `pl-`, `pr-`, `left-`, `right-`,
  `text-left`, `text-right` in app code fail. Exceptions: canvas coordinates, and `left-1/2` with `-translate-x-1/2`
  for centring.
- No literal colours (`#`, `rgb(`, `hsl(`, `oklch(`) and no raw ramp steps in `components/` or `screens/`.
- Forbidden utilities of section 2.6 (`text-gold` and every gold utility outside the two allowed files,
  `text-teal`, `text-risk-medium`, `text-risk-low`, any `*-risk-medium-fill` other than `bg-`, `border-line` on a
  control component).
- No `toLocaleString`, `Intl.DateTimeFormat` outside the kit, `input type="date"`, `type="time"` or
  `type="datetime-local"`.
- No `scaleX(-1)` outside the flip rule; no `letter-spacing` on selectors that can match Arabic.

> The vendored Untitled UI tree ships physical direction utilities and is not RTL-safe as delivered (the DBS counted
> 407). `npx untitledui@latest migrate <path>` fixes it: run it after every upstream re-pull, before reading a single
> component. Its absence is invisible in LTR.

## 7. Components

The layer is `frontend/src/components/`, wrapping `frontend/src/uui/`. Screens use the layer, not the vendored tree
directly, wherever a component below exists.

### 7.1 App shell

- **Rail** (Lapis, start side): the AKR symbol (gold on lapis) at the top; entries generated from the screen
  catalogue (section 8.1) for the user's roles only (IR-01, UX-01); the active entry gets a 3 px Gold marker on its
  start edge plus white text and `aria-current="page"` (the marker is never the only sign); counts on entries are
  neutral and count only items needing action (IR-64).
- **Header**: the bank's name (and logo from settings, section 12) at the start; global search and command palette
  (X2) in the centre; at the end, in this fixed order in both directions: notifications (X3), help (X5, one fixed
  place, WCAG 3.2.6, UX-40), language toggle, the user chip showing name and **active role**; the session countdown
  appears beside the user chip only in the last two minutes.
- **Data-as-of stamp**: every data screen shows "Data as of 2026-10-07 14:32" from the response's `data_as_of`
  (API.md section 1), with the list and configuration versions where relevant (IR-02). If the CBS feed is paused or
  late, the stamp turns into a medium badge with the words "Data delayed" and the lag.
- **Training mark**: inside the training workspace (W12) a permanent band reads "TRAINING - synthetic data" in both
  languages; it cannot be dismissed (UX-41).
- **Live-updates control**: on queues and dashboards fed by server-sent events, a "Pause live updates" toggle;
  new items are announced politely and in aggregate ("3 new items") and appear on request; rows never move under the
  cursor (IR-12, UX-37).

### 7.2 Tables and lists

TanStack Table owns the data; React Aria owns the table (semantics, keyboard, focus, sort affordance, RTL). They meet
at `sortDescriptor`. The wrapper (`components/table/`) enforces:

- **Text sorts through `Intl.Collator`** (`ar` with `numeric: true`; `ckb` for Kurdish lists), never by code point;
  code-point order scatters the alef forms and sends every Kurdish letter to the end (UX-28). Server sorts use the
  matching collation, with a parity test in CI.
- **Sort the number, render the sentence**: "متأخرة بيوم" and "4 hours left" sort by the underlying value.
- **Collections keyed on the locale**, or a language switch leaves rows in the old language (DBS lesson).
- Numbers end-aligned (the logical end), tabular, Mono for amounts; currency code in its own narrow column.
- Sticky header and first column; densities of section 4.2; keyset pagination; `aria-rowcount` and `aria-rowindex`
  when virtualised (IR-14, UX-38).
- No totals where restricted rows can exist (API.md cursor rule); "about" counts are never shown.
- Whole row navigable; two-line cell = title in 500 weight plus a Stone meta line with " · " separators; no per-row
  action buttons except "claim".
- Selection reveals the **batch action bar** with the count; selection clears on sort, filter or page change; bulk
  actions limited to assign, tag, snooze, request information and mark duplicate; a preview list, one reason code, a
  cap (default 25) and one audit event per item; no bulk closure of class P0 or P1 (IR-40 to IR-43, UX-13).
- Saved views per user and shared; filters, sort and period in the address (IR-04, UX-14).

### 7.3 Status badge

`StatusBadge` takes a **status key**, not a colour: the key maps to tone and to the bilingual word from the API's
enum labels. It cannot be rendered without its word (the prop is required and the label comes from the key). Tones:
high, medium, low, info, neutral (section 2.4). Optional leading icon at `size-3`.

### 7.4 Sensitivity-tier badge

A lock icon plus a word ("Restricted customer", "Restricted case"; the suspicion tier shows "Restricted case" too,
so the badge itself does not reveal which tier) on `tier-tint`, in the record header and in the browser tab title
(research 31 s.12.1 item 3). Shown **only to users on the record's access list**; for everyone else the record does
not exist (X6). Never red: the tier is not a verdict.

### 7.5 Priority class

Rendered as code and word, never colour alone: "P0 · Immediate", "P1 · Urgent", "P2 · Standard", "P3 · Routine"
(Arabic labels from the policy pack, default فوري، عاجل، اعتيادي، روتيني). The class is a rule result (research 28
priority classes); a help tooltip gives the legal order behind it. In this document and in code, priority classes are
written "class P0" to avoid confusion with the macro-phases P0-P3.

### 7.6 Name

The shared `Name` component (UX-25), used by customer, screening, case, report and branch screens:

- Arabic original first and larger, chain order, right to left, `lang="ar"` or `ckb`; the Latin transliteration
  beneath, smaller, labelled with its source (card, passport, typed, generated).
- Never truncated in a table: wrap to a second line; where a column is narrow, end-truncate with a popover of the
  full chain; never split into "first" and "last" name.
- In comparisons, chain parts aligned one under another with role labels (given, father, grandfather, tribe or
  family).
- The mother's name beside the birth year in every identification block.
- Titles and kunya as small separate tokens; aliases listed beneath with their class (strong, weak, kunya, former).
- Data-quality marks (truncated, mixed script, hidden control characters, legacy ZWNJ) as a small neutral mark with a
  tooltip.
- Copyable text, never an image; copying gives the raw stored value.

### 7.7 Values

`Amount`, `DateTime`, `Phone`, `Iban`, `Ref` (case, alert, report, list entry numbers), `Identifier` (national card,
residence card, passport, CBS customer number). Each isolates its value as one run, renders in Mono where section 3.2
says so, offers copy of the raw value, and masks according to the field's sensitivity and the user's grant (full
account numbers only where the API returns them). `Iban` groups in fours and validates Iraqi IBANs (23 characters,
mod 97) in inputs (UX-29).

### 7.8 Clock

Shows its **kind** (age, stage, decision to filing, RFI, Committee 48 h, freeze, review due) and its target, never a
bare "late" (IR-60): "Decision to filing · due 2026-10-09 14:00 · 1 d 6 h left". Medium due-text inside the warning
band, high with the word "Overdue" after the target. A paused clock ("Awaiting information") says "paused" and keeps
showing the elapsed age beside it (IR-61).

### 7.9 Reasons and explanations

- `ReasonChip`: reason code label (bilingual, from `ref.reason_codes`) with its fact beside it ("Cash deposits
  IQD 612.4 million in 7 days · 3.2 times expected"). At most three headline reasons on a row or summary strip; the
  rest one click away (UX-08, UX-42).
- `Explanation` renders the four layers L0 to L3 of research 31 section 7.2, disclosed by role; model attributions
  are labelled "model contributions"; thresholds never appear on branch screens.
- Mitigating factors are shown beside reasons, in neutral tone.

### 7.10 Assistant output (phase 2)

A labelled draft with sentence-level sources; unsupported sentences removed or marked; a "not considered" line;
recommendations only after the required evidence is opened; no "accept all"; saving requires an edit or an explicit
accept; a "useful or not" control; provenance one click away (IR-23, UX-45). Info tone with the word "Draft". After
an analyst edits it the label reads "Drafted by assistant · edited by <role>".

### 7.11 Decision panel

The case workspace's sticky panel (UX-06, UX-64, UX-65):

- **Actions** (request information, escalate, create operations task, start watch) and **decisions** (propose
  closure, recommend UTR or STR; for the MLRO: STR, UTR, TFR, no report with reasons, more evidence) are separate
  groups with separate headings (IR-30).
- Decisions need coded reasons plus text; the generic-text check runs as the user types and shows what it refused
  and why ("no suspicion" on its own; capacity wording) (IR-31).
- The CBI seven-criterion escalation checklist sits beside the decision, pre-filled where computable (IR-32).
- Decision buttons stay disabled until the evidence panels the class requires have been opened; the panel lists which
  are opened and which are not (IR-22, UX-09).
- Four-eyes is explained in the case's terms at the point of action ("This closure needs approval by a level-2
  analyst"), never as a blanket line (IR-34).
- The buttons render only when the response's affordances allow them (API.md affordances).

### 7.12 Step-up dialog and session warning

- `StepUpDialog` is one shared component for every signing-grade action (file a report, close without a report at
  MLRO level, clear a hold, approve a change request, export): WebAuthn first, TOTP fallback; one-time-code fields
  allow paste and autofill (`autocomplete="one-time-code"`); the dialog keeps screen state and drafts; focus is
  trapped and returned; the success result is the single-use assertion the API needs (`X-StepUp-Assertion`, API.md
  section 1) (IR-33, UX-36).
- `SessionWarning`: at the idle limit minus 60 s (default) a dialog counts down with an "Extend session" action;
  drafts survive a time-out (IR-62, UX-35).

### 7.13 Buttons, forms, feedback

- **Buttons**: one primary (Ishtar fill, white text) per view region; secondary for the rest; destructive actions are
  outline in `risk-high`, filled only inside the final confirmation dialog, which names the consequence and the
  target in bold (IR-35). Buttons disable and show inline progress during their own mutation.
- **Forms**: the schema-driven renderer (react-hook-form and Zod, MODULES.md 21.4) over Untitled UI `Input`, `Label`,
  `HintText`; control borders in `border-control`; errors under the field with icon and text, translated by rule,
  never toast-only (IR-53). Date entry is a text field with the expected order shown; typed dates are accepted in the
  shown order only, and unreadable values stay in the field with the fault (IR-51). Free text is scanned for card
  numbers and secrets before saving (IR-56).
- **Feedback**: skeletons for lists and evidence panels; optimistic UI only for mark-read and claim; decisions,
  signatures and approvals always round-trip. Errors show what happened, what to do and the correlation ID (IR-71).
  Long jobs (packs, backtests, exports) show progress, continue in the background and post a notification; beyond
  about 10 s a percent-done indicator (IR-73).
- **Empty states**: one sentence and one action for every list (IR-72). A refusal never looks like an empty list;
  for suspicion-tier resources it looks exactly like "not found" (IR-70; API.md existence masking).
- **Concurrent edits**: an `If-Match` conflict shows both versions and asks; it never overwrites (IR-26).

### 7.14 Timeline and notes

- `Timeline`: one vertical timeline per party or case (alerts, windows, observations, RFIs, authority requests,
  rating changes, decisions, tasks, QA events), filterable by type, with seasonal bands; dates in the product format
  (UX-07).
- Notes are posted, not edited; "Amend" creates a new note that points at the old one; drafts autosave and are marked
  "not evidence" (IR-24).

### 7.15 Charts

- No chart library until the RTL spike decides (tech-stack.md lists Recharts by default after the spike). Sparklines,
  share bars, range bars and simple bar lists are small in-house SVG components (`components/charts/`), as in the DBS.
- Every chart carries a sentence saying what it shows and a data table for screen readers; legends in words; direct
  labels on stacked bars; 2 px Paper gaps between segments; axis units on every axis (UX-34, UX-38).
- Time runs right to left in Arabic: positions are mirrored, the drawing is not (DBS rule, confirmed or changed by the
  spike, DS-O3).
- Every count and segment is a door to the filtered list behind it; zero is plain text (IR-03).

### 7.16 Network graph

`@xyflow/react` for ego graphs (tech-stack.md). Automatic grouping above about 15 accounts with one-click expansion
and group summaries; the subject is never grouped away; a text summary ("9 accounts, 214 senders, 96 percent to two
exchange companies"); a node and link table as the equal alternative; keyboard traversal; dragging has buttons and
keyboard moves; masked account numbers with the Arabic customer name (toggle) (UX-10, UX-61; IR-82).

### 7.17 Change and diff views

For change requests, scenario studio and mapping versions: old values struck through in Stone, new values in Ink with
an Ishtar start-edge bar; the computed change class and volume effect from the change request (DSL.md Part D) as
badges with words ("Material", "Reduces alerts"); the impact summary (alerts before and after, below-the-line sample
result) beside the diff; the reason code is required and capacity wording is flagged (UX-60).

### 7.18 Shortcuts

A shortcut layer on physical key codes (`KeyboardEvent.code`), labelled with the Latin keycap so Arabic-layout
typists get the same keys; never active inside text fields; turn-off and remap-with-Ctrl-or-Alt settings in X4; a
cheat sheet on `?` (IR-91, IR-92, UX-12).

### 7.19 Help

One help entry in the header; contextual "?" popovers next to fields and reason codes, drawn from the bilingual help
centre (offline, owner and review date per article); a "What changed" notice after a policy or release change
(UX-40). No policy chatbot.

## 8. Screens

### 8.1 Screen catalogue

The catalogue below is the source for `frontend/src/app/screens.ts` (ID, route, roles, layout pattern, phase), which
generates the routes, the role-aware rail, the access-matrix report and the route-to-policy coverage test (every
route maps to a Cedar action; none answers without a decision) (UX-53; SP-25, SP-71). Research file 31 section 10
holds each screen's purpose and key elements; this table adds the route and the layout pattern. Screens E1 and E2
are added by this document for the two early pieces of P0; G10 is added by phase 08 (DR-74).

Layout patterns:

| Pattern | Shape |
|---|---|
| **LIST** | Title, saved-view tabs with counts, filter bar, table, batch bar; detail opens as a page |
| **WORK** | Header strip, scrolling evidence column, sticky decision column at the end side (start side in LTR mirrors) |
| **COMPARE** | Two records side by side with the deciding identifiers between, decision below |
| **STUDIO** | Editor (schema-driven) at the start, test and impact at the end, change-request panel docked below |
| **DASH** | Attention row, period comparison, detail; tiles open their lists |
| **REGISTER** | Read-mostly table with filters and an export (SU job) |
| **STEPPER** | Numbered steps with validation per step, preview, sign |
| **INBOX** | Minimal single list of neutral items with one action each |
| **FORM** | Single short form with a neutral confirmation |

| ID | Screen | Route | Pattern | Phase |
|---|---|---|---|---|
| X1 | Sign-in and step-up | `/sign-in` (dialog everywhere) | FORM | P1 (P0 for staff) |
| X2 | Global search and command palette | header | overlay | P1 |
| X3 | Notifications and task inbox | `/inbox` | LIST | P1 (task half P0, phase 08) |
| X4 | Profile and preferences | `/me` | FORM | P1 (security section P0, phase 08) |
| X5 | Help centre, glossary, what changed | `/help` | reading | P1 |
| X6 | Not found and no access | any | message | P0 |
| E1 | goAML checker (with the profile pages at `/goaml-check/profiles`) | `/goaml-check` | STEPPER | P0 |
| E2 | Discovery and CBI coverage gap report | `/discovery` | DASH | P0 |
| W1 | My work | `/work` | DASH | P1 |
| W2 | Alert and case queue | `/queues/:id` | LIST | P1 |
| W3 | Case workspace | `/cases/:id` | WORK | P1 |
| W4 | Party page (customer 360) | `/parties/:id` | WORK (tabs) | P1 |
| W5 | Transaction explorer | `/parties/:id/transactions` | LIST | P1 |
| W6 | Network view and network case | `/parties/:id/network` | WORK | P1 |
| W7 | Decision dialogs | dialog in W3 | FORM | P1 |
| W8 | RFI composer and RFI desk | `/rfi` | LIST + FORM | P1 (correspondents P2) |
| W9 | Operations tasks and verification | `/ops-tasks` | INBOX (OPS) / LIST (M, A2) | P1 |
| W10 | Watches and hibernations | `/watches` | REGISTER | P1 |
| W11 | Internal reports and authority requests | `/requests` | REGISTER | P1 |
| W12 | Seeded cases and training workspace | `/training/practice` | WORK | P2 |
| S1 | Screening review | `/screening/hits/:id` | COMPARE | P1 |
| S2 | Screening queue | `/screening` | LIST | P1 |
| S3 | List status and known-answer tests | `/screening/lists` | REGISTER | P1 |
| S4 | Pair-memory and exemption register | `/screening/memory` | REGISTER | P1 |
| S5 | Freeze board | `/freezes` | LIST | P1 |
| K1 | Customer risk profile | `/parties/:id/risk` | WORK | P1 |
| K2 | KYC review queue and workspace | `/kyc` | LIST + WORK | P1 |
| K3 | Ownership and evidence vault | `/parties/:id/ownership` | WORK | P1 |
| K4 | PEP register and relatives | `/pep` | REGISTER | P1 |
| D1 | Scenario library and coverage matrix | `/scenarios` | REGISTER | P1 |
| D2 | Scenario studio | `/scenarios/:id/studio` | STUDIO | P1 |
| D3 | Policy studio | `/policy` | STUDIO | P1 |
| D4 | Model registry and AI governance | `/models` | REGISTER | P2 |
| R1 | Report builder (UTR, STR, TFR) | `/reports/:id` | STEPPER | P1 |
| R2 | Report list and FIU feedback | `/reports` | LIST | P1 |
| R3 | Regulatory and periodic reports | `/reports/periodic` | REGISTER | P1 |
| R4 | Board pack and evidence pack builder | `/packs` | STEPPER | P1 |
| R5 | Liaison report review | `/liaison` | LIST | P1 |
| T1 | Data connection | `/data/connection` | DASH | P1 (basic P0) |
| T2 | Reconciliation and coverage ledger | `/data/recon` | REGISTER | P1 |
| T3 | Mapping workbench | `/data/mapping` | STUDIO | P1 (draft P0) |
| T4 | Data-quality and quarantine queue | `/data/quality` | LIST | P1 |
| G1 | Command centre | `/command` | DASH | P1 |
| G2 | Capacity workbench | `/capacity` | STUDIO | P2 |
| G3 | QA workspace | `/qa` | LIST + WORK | P1 |
| G4 | Registers | `/registers` | REGISTER | P1 |
| G5 | Access and insider controls | `/access` | REGISTER | P1 |
| G6 | Audit viewer and chain verifier | `/audit` | REGISTER | P1 |
| G7 | Retention and legal holds | `/retention` | REGISTER | P1 |
| G8 | Assessor workspace | `/assessor` | DASH | P1 |
| G9 | Training register | `/training` | REGISTER | P1 |
| G10 | Change requests | `/changes` | LIST + detail | P0 (phase 08) |
| I1 | Sharia board aggregates | `/islamic/sharia-board` | DASH | P1 (phase 26, optional pack) |
| B1 | Branch home | `/branch` | INBOX | P1 |
| B2 | Branch review | `/branch/items/:id` | FORM | P1 |
| B3 | Observation form | `/branch/observe` | FORM | P1 |
| B4 | Confidential report | `/branch/report` | FORM | P1 |
| B5 | Monthly liaison report | `/branch/monthly` | STEPPER | P1 |
| B6 | Branch transactions and documents check | `/branch/check` | LIST | P1 |
| A1 | Users, roles and leave | `/admin/users` | LIST | P0 |
| A2 | Settings | `/admin/settings` | FORM | P0 |
| A3 | Platform health | `/admin/health` | DASH | P0 |
| A4 | Release and updates | `/admin/releases` | STEPPER | P1 |

**Count:** 63 screens (59 from research file 31 plus E1, E2, G10, the change-request screen behind the rail's
"Governance (change requests)" entry, added by phase 08, DR-74, and I1, the Sharia board view of phase 26, added by
gap pass 1), 8 with approved mockups (01 G1, 02 W3, 03 S1,
04 K1, 05 D2, 06 W6, 07 R1, 08 T1).

### 8.2 Rail by role

Role codes are MODULES.md section 0. The rail shows only what the role may open (IR-01).

| Role | Rail entries |
|---|---|
| L | Branch window only (separate bundle): home, items, observation form, report to the MLRO, monthly report, documents check |
| A1, A2 | My work; Alerts and cases (one entry, saved views "Alerts" and "Cases"); Customers; Screening (if granted); Reports (read); Help |
| SS | My work; Screening; Customers; Cases (linked); Help |
| M | Command centre; Decisions; Alerts and cases; Screening; Customers; Reports; Governance; Help |
| KYC | My work; KYC reviews; Customers; PEP register; Help |
| RFI | RFI desk; Help |
| QA | QA workspace; sampled cases; Help |
| OPS | One "Operations tasks" inbox, nothing else |
| CFG, CHK | Scenarios; Policy studio; Governance (change requests); Help |
| DS | Data; Help |
| CH | Command centre; Reports; Governance (read); board pack |
| MRO | Models; Scenarios (read); Governance (change requests); Help |
| IA, AS | Assessor workspace; Governance (read); Reports (read) |
| ADM | Administration; Platform health |
| SEC | Administration (security); Access and insider controls; Audit |
| VS | Platform health (no business data) |

A user with several roles sees the union, grouped by role, with the active role shown in the user chip.

### 8.3 Specifications of the key screens

The mockups are English, left-to-right pictures for bank leadership. The build designs each screen right to left
first and mirrors it for English. Every screen applies the common changes of research 31 s.12.1 (UX-55): mirroring,
role-aware rail, tier badge, contrast tokens, the product date format, English-only micro-labels, names as text,
header controls, focus states.

**G1 Command centre (mockup 01; UX-47, UX-48, UX-56).** Order: what needs attention now (decisions waiting for me,
overdue and at-risk clocks, verification failures, data and coverage incidents), then the period against the one
before (arrivals against throughput, capacity ratio, QA status, tipping-off summary), then the detail (backlog by
elapsed age, top scenarios with cases, UTRs and STRs and a defined "productive", regulatory deadlines with words,
branch liaison reports submitted of total). Every tile has a definition, a formula in its help, the data-as-of time,
and opens its list. Open work reads "1,284 alerts in 911 cases". Stacked bars gapped and labelled. From phase 29,
the Assessment tile (phase 29 section 5.3): modes per rail, list freshness, open blind periods and open release
exceptions, counts only; visible to M, CH and IA; it opens the Assessment tab of D3 for M and CH and the
release-without-decision list for IA.

**W2 Alert and case queue (UX-05, UX-13, UX-14; IR-10 to IR-16).** LIST pattern. Columns: priority class (code and
word), customer (Name, Arabic first), top reason (ReasonChip), age, next due (Clock), merged-alert count, owner; no
more. Sort by class, then score, then age, explained in a tooltip. Saved-view tabs "All", "Alerts", "Cases", "Mine",
"Unassigned" with counts. Queue-run mode: "Next" opens the next item of the view; a key opens the oldest unassigned
item of the user's pool. Pause live updates.

**W3 Case workspace (mockup 02; UX-06, UX-57, UX-64).** WORK pattern. Header strip: case reference, priority class,
clocks (age and next due), tier badge, merged alerts, current stage, data-as-of. Evidence column: why raised
(ReasonChips with CBI scenario rows), money in and out against declared turnover with the seasonal band, entity
timeline, counterparties, evidence panels expanding in place (transactions, documents, RFIs and answers, earlier
cases, screening, adverse media), network link, assistant block (P2). Decision column, sticky (7.11). Activity is a
filterable timeline. "Add to evidence" captures the window, filters or graph definition as well as an image (IR-27).

**S1 Screening review (mockup 03; UX-43, UX-44, UX-58).** COMPARE pattern. Top: a verdict sentence with the evidence
level and the applied threshold (not a bare score ring). Records side by side (customer and list entry), Arabic
first with transliteration. Between them the deciding identifiers, each marked agree, differ or not comparable:
mother's name and maternal grandfather, birth date and its precision, document numbers, place of birth, family
number. Per-part table with Arabic parts first, folds applied and unmatched tokens. Frequency sentence; list-entry
facts (aliases, listing basis, link to the source entry). Decision tiles in the Office's words: True match,
Potential match, Incorrect match (Arabic: التطابق الحقيقي، التطابق المحتمل، التطابق غير الصحيح; the CBI's terms as
aliases in help), each with its consequence line (freeze clock and 48-hour Committee report; verification due;
reason recorded). "Remember this decision for this pair" appears only on Incorrect match, unticked, with its expiry.
Four-eyes notice on freeze lists; coded reason required.

**S2 Screening queue, tab Holds (phase 29 section 5.1).** A tab beside the alert queue lists payment holds
(`GET /api/v1/assess/holds`) with the rail, the mode label ("held at the gateway", "advisory: not held at the
gateway", "shadow: no gateway effect"), reason codes, age, the countdown to expiry and the proposal state. A
suspicion-tier subject the caller may not see is not listed. Claim, propose release or reject, and approval by a
second person follow the alert rules of S2, each with step-up class `decision`.

**K1 Customer risk profile (mockup 04; UX-59).** The number, the level word and the 12-month trend (no decorative
gauge). Drivers as sorted bars with points, reasons, refresh dates and the missing-input share; floors and triggers
that applied, in words; override history; rating-change reasons; EDD checklist with dates; open cases and watches
strip (shown only within the user's access).

**D2 Scenario studio (mockup 05; UX-60).** STUDIO pattern over the scenario DSL (DSL.md Part A): parameters by
segment with hard bounds and approved band shown; plain-language description "written automatically"; test on
history and below-the-line sample; the change-request panel with computed class, volume effect and route (7.17);
version history; effective date. The example reason in the mockup is replaced by a risk reason, and capacity wording
is flagged.

**D3 Policy studio, tab Assessment (phase 29 section 5.2).** Three panels: Fail policy (the matrix as a table and
the bands per currency), Rail modes (one row per rail with its mode, effective-from instant and readiness criteria
with observed and required values) and Prohibited rules (read-only list with "Propose a rule"). Each panel says
that a change is material and needs MLRO and CHK, and CH for a decrease.

**W6 Network view (mockup 06; UX-10, UX-61).** 7.16, plus "Open one case for the cluster" and "Add graph to evidence"
with the snapshot note.

**R1 Report builder (mockup 07; UX-62).** STEPPER: report type (UTR, STR, TFR); subject and accounts; origin of
suspicion and earlier reports; indicators (chips); narrative (assistant draft badge in P2, tipping-off phrase check,
sentence-level sources); validation panel with links to fix (goAML checker rules, the same engine as E1); review and
sign (step-up); 7 "Filed and acknowledged" with the goAML reference, time and acknowledgement, "Download goAML XML"
and "Record filing reference". The decision-to-filing clock is in the header; amendments show a version comparison;
an FIU feedback strip appears after filing.

**T1 Data connection (mockup 08; UX-63).** Read-only tile with the time of the last grant check and the words
"Read-only verified"; replica delay; reconciliation table with "Match" words; data-quality list with severity words;
feed volume chart with units and a table alternative; bank kill-switch state; quarantine count and age; dark units;
coverage certificate link.

**E1 goAML checker (P0).** STEPPER in a reduced shell: upload or paste a goAML XML report the bank produced
elsewhere; validation against the goAML schema and the Office's rules with each fault linked to its field and a
bilingual explanation; a result page that can be printed. Nothing is filed from here.

**E2 Discovery and CBI coverage gap report (P0).** DASH in a reduced shell: source profiles and field availability
(M5), then the 173 CBI items against the bank's data with a readiness grade each (M7), then "Generate gap report"
as a PDF pack (M16).

**B1 to B6 Branch window (UX-02, UX-66, UX-67).** A separate bundle of about 300 KB compressed script, no chart or
graph library, relaxed density, `text-md` body. Items carry neutral statuses only ("Awaiting your input", "Received",
"Closed") with due times; no reason codes with thresholds, no outcomes, no AML vocabulary (section 11). A prominent
"Report a concern" button on every branch screen. The monthly report is auto-drafted and signed with step-up by the
10th. An item that moves into the suspicion tier disappears without explanation, by design.

**W9 Operations-task inbox (UX-69).** INBOX for OPS: each item shows the neutral instruction and reference only
("Apply restriction to account ... · reference T-2026-0412"), the due time, and "Done" with the CBS reference field.
No case, customer risk, reason or name beyond what the instruction needs.

**X1 Sign-in and step-up.** The bank's name and logo above the AKR-TMS lockup; WebAuthn first; paste allowed in code
fields; a calm failure message with the correlation ID; lockout text without revealing which factor failed.

**X6 Not found.** One page, byte-identical for a missing resource and a masked one (API.md existence masking). A
distinct "no access" page only where existence is not secret.

**G8 Assessor workspace (UX-50).** The evidence matrix for the CBI assessment guide's D2 test; random sample draw; end-to-end trace from a CBS row to the
alert, case, decision and report; chain verifier; pack download; a banner naming the read-only, time-boxed account
and its expiry.

## 9. Accessibility (WCAG 2.2 AA)

Target: WCAG 2.2 level AA in every theme that ships, on every route, in both languages (UX-32). The criteria that
bite hardest here, and how the design meets them:

| Criterion | How |
|---|---|
| 1.4.1 Use of colour; 1.4.11 Non-text contrast | Words on every status (2.4); control border 3:1 (2.3); gapped, labelled segments (2.6) |
| 1.4.3 Contrast | Token check in CI (2.9); measured against Clay, the worst surface a row or badge sits on, not only white |
| 1.4.4, 1.4.10 Resize and reflow | Reflow to 320 CSS px; tables scroll in their own container (4.3) |
| 1.3.2, 2.4.3 Order | DOM order = Arabic reading order; logical properties only (4.3, 6.4) |
| 2.1.1, 2.1.4 Keyboard and character shortcuts | Everything by keyboard; shortcut layer can be turned off and remapped (7.18) |
| 2.2.1 Timing | Session warning with extend; drafts survive (7.12) |
| 2.2.2 Pause | Pause live updates; no reordering under the cursor (7.1) |
| 2.4.7, 2.4.11 Focus visible and not obscured | React Aria focus ring, 2 px Ishtar with 2 px offset (white on the rail); `scroll-padding` under sticky headers and the decision panel |
| 2.5.7, 2.5.8 Dragging and target size | Button and keyboard alternatives to every drag; 24 px targets (4.2, 7.16) |
| 3.1.2 Language of parts | `lang` on every Arabic, Kurdish and English span (Name, values) |
| 3.2.6 Consistent help | One help entry, fixed place (7.1) |
| 3.3.7 Redundant entry | RFI, report and decision forms pre-filled from the case |
| 3.3.8 Accessible authentication | Paste and autofill of codes, WebAuthn first (7.12) |
| 4.1.2 Name, role, value | React Aria primitives; graph and timeline have roles, keyboard models and table alternatives |

React Aria's `I18nProvider` is wired at the app root (`src/i18n/AriaLocale.tsx`, from the DBS); it is not a duplicate
of `<html dir>`: that attribute drives CSS, the provider drives keyboard behaviour. Without it the layout mirrors and
the arrow keys do not. `prefers-reduced-motion` switches off the few transitions the product has.

**Testing** (UX-32, UX-39): axe-core on every route, in both languages, in every shipped theme, in CI; a manual
keyboard-only walkthrough of the six core tasks per release (open a case, record a disposition, request information,
escalate, sign a report, answer an RFI); 400 percent zoom in Arabic; screen-reader passes with NVDA and VoiceOver
including mixed-script names (Arabic screen-reader behaviour is a named test item, DS-O5); a conformance statement
per release.

## 10. Performance budgets

From IR-80 to IR-83 (UX-16), tested in CI against a synthetic data set:

- Queue and case navigation: first content within 1 s on the bank LAN at 50 concurrent analysts; evidence panels
  within 300 ms from cache or a skeleton; type-ahead search p95 within 300 ms; instant interactions within 100 ms.
- Branch window: about 300 KB compressed script; first content within 5 s at 2 Mbps and 300 ms latency.
- Graphs: 100 nodes within 2 s; grouping above about 15 accounts.
- Fonts subset per script and preloaded for the active language only; icons tree-shaken.

## 11. Wording

Plain words, written in Arabic first and translated to English, not the other way round. Vendor and reviewer
evidence (research 31 s.9) is that technical vocabulary is a top complaint; AKR-TMS's vocabulary is the bank's.

- **A glossary owned by compliance staff** (X5) is the source of terms in both languages; the UI uses its terms only.
- **Never print a machine ID at a reader**: every enum, step, reason code and status has a bilingual label from the
  API (IR-54).
- **Match states use the Office's words** (true, potential, incorrect match), with the CBI's words as aliases.
- **Neutral surfaces** (branch window, operations inbox, notifications, mail, the browser tab title for non-members):
  no "suspicion", "suspicious", "STR", "UTR", "money laundering", "terrorist", "freeze reason", "sanction", no
  thresholds, no amounts in notifications, no customer names in notifications. Notifications read "You have a task"
  with a link (IR-63). Neutral text codes (`ops_tasks.neutral_text_code`) carry the instruction wording; the
  forbidden-word list is part of the policy pack, owned by compliance, and the RFI and branch forms scan against it
  before release (UX-68).
- **Counted phrases** go through plural rules; **sentences are never concatenated** from fragments.
- **Errors** say what happened and what to do, calmly, without blame and without technical detail; the correlation
  ID is shown for support.

## 12. Assets

| Asset | Where | Notes |
|---|---|---|
| Symbol, gold on lapis | `plan/assets/logo-symbol.svg` → `frontend/public/akr-symbol.svg` | Rail top, favicon (32, 64, 180, 512 px PNGs generated from the SVG) |
| Symbol, one-colour lapis | `plan/assets/logo-symbol-mono.svg` → `frontend/public/akr-symbol-mono.svg` | Light backgrounds, sign-in, PDF footers |
| Wordmark | Text in IBM Plex Sans ("AKR" SemiBold, "-TMS" Regular, tight tracking) | Built in markup, not an image; the Arabic lockup keeps the Latin name with the descriptor "نظام مراقبة المعاملات ومخاطر العملاء" in Plex Sans Arabic |
| Clear space | One wedge height on every side | Identity |
| Bank logo | Setting `brand.bank_logo` (M20 settings, uploaded by ADM, changed by change request) | Sized by **height** with `w-auto` everywhere (a square box squashes a wide logo, DBS lesson); embedded as a data URI in PDF templates, because Gotenberg must not fetch anything remotely (no egress) |
| Fonts | `@fontsource/ibm-plex-sans-arabic`, `@fontsource/ibm-plex-sans`, `@fontsource/ibm-plex-mono` with `latin-ext` | Vendored; same files in the Gotenberg image |
| Icons | `@untitledui/icons` | Tree-shaken |
| Tokens | `frontend/src/styles/tokens.json` → `akr.css` | Section 2.9 |
| Screen catalogue | `frontend/src/app/screens.ts` | Section 8.1 |
| Mockups | `plan/` (eight English images) | Layout reference; the tokens here win over sampled image colours |

`alt` text on the bank's logo is the bank's name; on the AKR symbol it is "AKR-TMS". In PDF templates the logo's
`alt` is empty, so a text extractor does not put a word into the document that nobody wrote (DBS lesson).

## 13. Interaction-rule checklist

Every pull request that adds or changes a screen carries the checklist of research 31 section 11 (IR-01 to IR-93)
(UX-54). Automated where possible: logical properties, native date inputs, `toLocaleString`, literal colours and
forbidden utilities (6.4), contrast tokens (2.9), the shortcut layer on key codes, axe on the route, the
route-to-policy coverage test (8.1), the glyph-coverage test (3.1), collation parity (7.2).

## 14. Open items

| ID | Item | Decided by | When |
|---|---|---|---|
| DS-O1 | Approve, change or drop the proposed dark palette (2.8). Until approved, light only | Owner | Before P1 build of the shell |
| DS-O2 | Confirm the Derived tokens (2.3) and the generated ramp steps (6.2) | Owner | Phase 08 design-token task |
| DS-O3 | Chart time-axis direction in RTL and the chart library (Recharts or in-house only) | RTL spike (tech-stack.md open item) | P0 |
| DS-O4 | Numeric ranges: reading order or left to right | First usability round with Iraqi staff (UX-03) | P0 / early P1 |
| DS-O5 | Arabic screen-reader behaviour: mixed names, Western digits in Arabic speech | Accessibility test round | P1 |
| DS-O6 | Branch link quality and branch hardware (budgets in section 10 assume 2 Mbps, 300 ms) | First bank's IT | P0 discovery |
| DS-O7 | Arabic leading values (3.3) against marked text | RTL spike | P0 |
