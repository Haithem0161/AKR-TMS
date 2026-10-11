# Phase 08: Frontend Foundation

**Goal:** The web application the P0 product is used through: the design tokens of the locked identity generated
into one Tailwind entry with a contrast check in CI; Untitled UI vendored and migrated to right-to-left; the i18n kit
(direction, isolates, dates, digits, folds, six-form plurals, Hijri display); the vendored IBM Plex fonts with a
glyph-coverage test; the shared component layer that makes the design rules unavoidable; the machine-readable screen
catalogue that generates the routes and the role-aware rail and drives a route-to-policy test; the app shell in its
reduced P0 form; and the P0 screens on the APIs of phases 01 to 07: sign-in, step-up and session warning (X1), not
found (X6), the security section of the profile (X4), users and roles (A1), settings (A2), platform health (A3), the
task inbox with the task panel (X3, task half), change requests (G10), the basic data connection (T1), and the two
early pieces, the CBI coverage gap report (E2) and the goAML checker (E1). Two spikes (RTL charts, network graph)
settle the open chart and leading questions before P1.

**Dependencies:** Phase 01 (sessions, step-up, `/me`) and Phase 04 (inbox, tasks, change requests) for stages 1 to
4; Phases 03 (users, settings, health), 05 (sources, discovery), 06 (coverage) and 07 (goAML) for the screens of stage
5, which lands after them. Phases 06 and 07 list this phase as their screen dependency; the shell and the component
layer (stages 1 to 3) land before them, the E screens after.
**Macro-phase:** P0
**Complexity:** L (no new routes; the component layer, the i18n kit and eleven screens carry the weight)
**Binaries touched:** the web bundle (`frontend/`, served by Caddy); `akr-ctl` (`policy matrix --json`); akr-api
(OpenAPI extensions only: `x-akr-stepup`, `x-akr-idempotency`, the `ProblemCode` and `ActionClass` schemas).
akr-worker,
akr-ingestd and akr-assess unchanged
**Requirements carried:** UX-01 (the role-aware rail from the screen catalogue), UX-14 (the "My inbox" views of the
task half; saved views arrive with the queues of phase 17), UX-15 (three densities, sticky header and first column),
UX-18 (Western digits in data, prose-digit preference, every numeric input accepts three digit sets), UX-19 (isolates
inside values, raw copy), UX-20 (the kit's scan for bidirectional and zero-width characters; flagging as data
quality arrives with ingestion in phase 11), UX-21 (the product date format, no native date inputs, no
`toLocaleString`), UX-22 (the Hijri formatter with variant labels, display only), UX-23 (money with its code and the
compact Arabic forms), UX-24 (ICU plurals with six Arabic forms, keys in state), UX-25 (the shared `Name` component),
UX-26 (vendored fonts with `latin-ext` and the glyph-coverage test), UX-27 (Arabic typography rules), UX-28
(collation-aware sorting with a browser and server parity test), UX-30 (mirroring rules), UX-32 (axe on every route in
both languages in CI), UX-33 (on-tint text tokens, control border and the contrast check over the token file), UX-34
(status never by colour alone), UX-35 (the session warning; server half in phase 01), UX-36 (the step-up dialog;
server half in phase 01), UX-38 (row counts when virtualised, table alternatives, focus not obscured, 24 px targets),
UX-53 (the machine-readable catalogue, the rail and the route-to-policy test; the `x-akr-action` half in phase 03),
UX-54 (the interaction-rule checklist with its automated checks), UX-55 (the changes common to all mockups, applied to
the P0 screens), UX-63 (the basic T1 with words on every severity and the kill-switch state; feed volume and
reconciliation in phases 11 and 12)
- Gap pass 1 (coverage), partly implemented: TS-35 (the frontend stack, the table, the bidi helpers and the chart and graph spikes); UX-71 (the role-based rail and plain wording).
- Gap pass 1 (coverage), implemented here: UX-03 (usability round 1 with Iraqi staff settles the range order, section 7+), UX-39 (the per-release accessibility gate: keyboard walkthroughs, 400 percent zoom in Arabic, screen-reader passes, section 7+); VA-18 (Arabic and right-to-left first on every screen, English second).
- Gap pass 1 (coverage), not carried: UX-52 (vendor usability benchmark, research only, DR-400).

Frontend phase. Sections 1 and 2 hold the few backend additions the client contract needs; section 3 is the frontend
architecture and the screen specifications; section 5 orders the work in stages.

## 1. SQLx Migration Changes

None. This phase adds no table, column, setting or flag. The preferences it writes (`density`,
`prose_digits`) are phase 01 `PreferencesV1` fields through `PATCH /me/preferences`; it writes neither `shortcuts` (the shortcut layer is P1) nor `theme` (hidden while DS-O1 is open). `locale` is not a `PreferencesV1` field: it is the `admin.users.locale` column of phase 01, written by the same `PATCH /me/preferences`.

## 2. DDD Implementation (Rust)

No domain or repository changes, and no new routes; two read scopes widen in phases 05 and 06 (section 4). Four
additions make the API contract complete enough that the browser never
copies a string the server chose (DR-76):

- **`x-akr-stepup: <action_class>`** on every operation that calls `StepUpService::consume`, set by the same
  `StepUpAction` constant the handler passes (phase 01 declares the constants; the `authz_route!` macro of phase 03
  writes the extension). The phase 01 test "every SU operation names a declared action" now reads the extension.
- **`x-akr-idempotency: required | exempt`** on every mutating operation, from the phase 00 exemption list (one
  source in `common::idempotency::EXEMPT`); a test asserts that the list and the extension agree.
- **`ProblemCode`** (`ToSchema` enum of every `E_*` code of API.md section 6) as the type of `ProblemDetails.code`,
  and **`ActionClass`** (`ToSchema` enum of `common::stepup::ActionClass`) as the type of the `action_class` member of
  the `E_STEPUP_REQUIRED` problem and of `StepUpChallenge`, so both appear in the generated models.
- **Scopes in the document**: each operation's `security` requirement lists its scopes (the cookie and bearer
  schemes declare the scope names), and the document-level extension `x-akr-role-scopes` carries the role-to-scope
  table of `common::scopes::scopes_for_roles` (phase 01), so the browser reads both instead of parsing descriptions.
- **`akr-ctl policy matrix --bundle <dir> --json`** (phase 03 `AccessMatrixBuilder` over a bundle on disk, no
  database): `{bundle_hash, roles: [{role, actions}]}` (the `RoleActions` list of phase 03 section 7.16) from the permits' `@role` annotations evaluated against
  the schema's actions. `make generate-api` writes it to `frontend/src/app/policy-matrix.json`.

`make generate-api` becomes: start the API against the dev database, run Orval, run `scripts/gen-contract.mjs`
(reads the OpenAPI document and writes `frontend/src/api/contract.gen.ts`: per operation `{operationId, method,
path, action, stepup, idempotency, access, scopes}`), run `akr-ctl policy matrix`. The CI drift check (phase 00)
covers all three outputs: a second run gives zero diff.

Route table: none (this phase adds no routes).

## 3. Business Logic

### 3.1 Project layout and dependencies

```
frontend/
├── index.html                      # <html lang="ar" dir="rtl"> before first paint; no external URL anywhere
├── vite.config.ts                  # tanstackRouter() before react(), tailwindcss(); /api proxied in dev
├── orval.config.ts                 # tags-split, react-query, axios mutator, headers: true (X-StepUp-Assertion)
├── eslint.config.js                # typescript-eslint, react-hooks recommended-latest, the AKR restricted syntax
├── playwright.config.ts            # browser checks (section 6; DR-77)
├── scripts/
│   ├── gen-tokens.mjs              # tokens.json -> src/styles/akr.css (generated, committed, drift-checked)
│   ├── check-contrast.mjs          # every required pair and every forbidden pair (DESIGN-SYSTEM 2.9)
│   ├── lint-design.mjs             # class-string rules of DESIGN-SYSTEM 6.4 over src/components and src/screens
│   ├── gen-contract.mjs            # OpenAPI extensions -> src/api/contract.gen.ts (section 2)
│   └── glyphs.txt                  # every letter of research file 22 lists (Arabic, Kurdish, Persian, Turkish, marks)
├── public/                         # akr-symbol.svg, akr-symbol-mono.svg, favicons (DESIGN-SYSTEM 12)
└── src/
    ├── main.tsx                    # providers: QueryClient, I18nextProvider, AriaLocale, RouterProvider
    ├── api/                        # axios-instance.ts (mutator), problem.ts, etag.ts, stepup.ts,
    │                               # generated/ and models/ (Orval, never edited), contract.gen.ts
    ├── app/                        # screens.ts, policy-matrix.json (generated), rail.ts, guard.ts
    ├── i18n/                       # the kit (section 3.3) and locales/{ar,en}/*.json
    ├── styles/                     # tokens.json, akr.css (generated), theme.css (the one entry)
    ├── uui/                        # Untitled UI MIT tier, vendored, RTL-migrated, imports rewritten to @uui/
    ├── components/                 # the shared layer (section 3.7)
    ├── screens/                    # one folder per screen ID: x1/, x3/, x4/, x6/, a1/, a2/, a3/, g10/, t1/, e1/, e2/
    ├── routes/                     # TanStack file routes, thin: each renders one screen component
    └── test/                       # setup.ts (jsdom shims React Aria needs), fixtures, msw handlers
```

Dependencies (pinned exactly in `package.json`, `pnpm` with a committed lock file; versions are the DBS's proven set
at the time of writing, rechecked at implementation and recorded in tech-stack.md): `react` and `react-dom` 19.2,
`@tanstack/react-router` 1.170, `@tanstack/react-query` 5.101, `@tanstack/react-table` 9.1 (the v9 API: `useTable`,
`tableFeatures`), `@tanstack/react-virtual`, `react-aria-components` 1.20, `react-aria` 3.51, `react-stately` 3.49,
`@internationalized/date` 3.12, `axios` 1.18, `i18next` 26.3, `react-i18next` 17.0, `react-hook-form` 7.83,
`@hookform/resolvers` 5.4, `zod` 4.4, `tailwind-merge` 3.6, `input-otp` 1.5, `@untitledui/icons` 0.0.22,
`@fontsource/ibm-plex-sans-arabic`, `@fontsource/ibm-plex-sans`, `@fontsource/ibm-plex-mono` (5.x, with
`latin-ext`). Dev: `vite` 7.3, `@vitejs/plugin-react`, `@tanstack/router-plugin`, `@tailwindcss/vite` 4.3,
`tailwindcss` 4.3, `tailwindcss-react-aria-components`, `typescript` 5.9, `typescript-eslint`, `eslint` 10,
`eslint-plugin-react-hooks` 7, `vitest` 4.1, `jsdom`, `@testing-library/react`, `@testing-library/user-event`, `msw`,
`orval` 8.22, `prettier`, `@playwright/test`, `@axe-core/playwright`. Not added in this phase: a chart library (the
spike decides, section 3.9), `@xyflow/react` (spike only, not in the bundle), any rich-text editor, any toast library
(errors are never toast-only, IR-53). `pnpm licenses list` in CI refuses anything outside the tech-stack.md licence
list.

### 3.2 Tokens, styles and the design lint (DESIGN-SYSTEM 2, 3, 6; UX-27, UX-33, UX-34)

1. `src/styles/tokens.json` holds every token of DESIGN-SYSTEM 2.2, 2.3 and 2.8 as `{name, light, dark, status:
   Locked | Derived | Proposed, allowed_on: [surfaces]}`, the type scale of 3.3 with Latin and Arabic leading, the
   radius and shadow values of 4.1 and the density table of 4.2.
2. `scripts/gen-tokens.mjs` writes `src/styles/akr.css`: the Untitled UI ramp redirection of DESIGN-SYSTEM 6.2 (ramp
   steps between anchors generated in OKLCH and written as hex), the AKR semantic tokens under `@theme`, the three
   explicit Untitled UI aliases, radius and shadow, the font stacks, the Arabic leading under `:where([dir="rtl"])`,
   every `--tracking-*` zeroed under `[dir="rtl"]`, and `[data-theme="dark"]` values only for tokens whose dark
   status is approved (none while DS-O1 is open, so the file has no dark block). `theme.css` is the single entry:
   `@import "tailwindcss"; @import "./akr.css";`.
3. `scripts/check-contrast.mjs` (WCAG 2.x formula) fails CI when: a text token is under 4.5:1 on any surface in its
   `allowed_on`; a control, focus or graphic token is under 3:1 on Paper or Clay; a forbidden pair of DESIGN-SYSTEM
   2.6 rises above its threshold; a `Locked` value differs from `plan/identity.md` (the script parses the identity's
   hex table).
4. `scripts/lint-design.mjs` scans string literals in `className`, `cva()` and `cx()` calls under `src/components`
   and `src/screens` and fails on: physical direction utilities (`ml-`, `mr-`, `pl-`, `pr-`, `left-`, `right-`,
   `text-left`, `text-right`, `rounded-l-`, `rounded-r-`, `border-l`, `border-r`; the centring exception
   `left-1/2` with `-translate-x-1/2` is allowed); raw ramp steps (`-gray-500`, `-brand-600` ...); literal colours;
   `text-teal`, `text-risk-medium`, `text-risk-low`; any `gold` utility outside `components/shell/Rail.tsx` and
   `components/brand/Logo.tsx`; `*-risk-medium-fill` other than `bg-`; `border-line` in `components/form/`;
   `scale-x-[-1]` (the flip rule lives in `theme.css` only).
5. ESLint `no-restricted-syntax` and `no-restricted-properties`: `toLocaleString`, `toLocaleDateString`,
   `toLocaleTimeString`, `Intl.DateTimeFormat` and `Intl.NumberFormat` outside `src/i18n/`; JSX `input` with `type`
   `date`, `time` or `datetime-local`; `dangerouslySetInnerHTML`; `localStorage`, `sessionStorage` and `indexedDB`
   outside `src/app/uiPrefs.ts` (DR-75); `window.open` and `<a target="_blank">` without `rel="noopener"`; imports from
   `@uui/` inside `src/screens/` where a `components/` wrapper exists (a list kept in the rule).
6. Untitled UI: the MIT tree copied to `src/uui/` without demos and stories, bare `@/` imports rewritten to `@uui/`,
   `npx untitledui@latest migrate src/uui` run before any component is read (DESIGN-SYSTEM 6.4 note), the single
   dark selector changed to `[data-theme="dark"]`. `src/uui/README.md` records the upstream commit, the migrate run
   and the edits, so a re-pull is a copy, the rewrite and the migrate.
7. Theme controller (`src/app/theme.ts`): stamps `data-theme="light"` before first paint; the stored choice wins only
   when the dark palette is approved (DS-O1); `prefers-color-scheme` is not read.

### 3.3 The i18n kit (`src/i18n/`; DESIGN-SYSTEM 3.5; UX-18 to UX-24, UX-28)

Copied from the DBS (`direction.ts`, `bidi.ts`, `datetime.ts`, `fold.ts`, `AriaLocale.tsx`, `lang.ts`) and extended.
Screens never format by hand; every function has unit tests in both languages.

| Module | Functions | Rules |
|-|-|-|
| `direction.ts` | `dirOf(lang)`, `applyDocumentLang(lang)` | Sets `<html lang dir>`; Arabic `rtl`, English `ltr`; Kurdish (`ckb`) reserved |
| `bidi.ts` | `ltr(s)` (LRI ... PDI), `auto(s)` (FSI ... PDI), `raw(s)` (strips isolates and marks), `scanControls(s)` -> `{bidi: [offsets], zero_width: [offsets]}` | Isolates go inside the value (DBS decision 70); copy and comparison use `raw` |
| `digits.ts` | `westernDigits(s)` (Arabic-Indic U+0660-0669 and Persian U+06F0-06F9 to 0-9), `proseDigits(s, pref)` | Data always Western; prose follows the user's `prose_digits` |
| `datetime.ts` | `formatDateTime(iso)`, `formatDate(iso)`, `formatDateTimeRaw`, `normalizeDate(input)`, `normalizeTime(input)`, `monthName(n, lang)` | `YYYY-MM-DD HH:MM` in `Asia/Baghdad` via `@internationalized/date`, wrapped in LRI and PDI; unreadable -> empty string; input normalised on blur, unreadable input returned unchanged |
| `hijri.ts` | `formatHijri(iso, variant)` with the variant label (`umm_al_qura`, `civil`, `tabular`) | Display only; never compared or used for a clock (UX-22) |
| `money.ts` | `formatMoney(amount, ccy)`, `formatCompact(amount, ccy, lang)` | Decimal strings in, never floats; figure in Mono plus the ISO code as one isolate; "مليون" and "مليار" with six-form plurals in tiles |
| `number.ts` | `formatCount(n)`, `formatPercent(x, dp)`, `formatRange(a, b)` | Tabular Western digits; ranges left to right as one isolate (DS-O4 may change it) |
| `fold.ts` | `fold(s, lang)`: alef forms, taa marbuta, alef maqsura, tatweel, harakat; Kurdish and Persian letters (ک ی ە ڕ ڵ ۆ ێ پ چ ژ گ); ZWNJ repair | Same table as the server's `text::fold` (phase 06), with a parity test over a shared fixture file |
| `collate.ts` | `collator(lang)` (`Intl.Collator('ar', {numeric: true})`, `ckb` for Kurdish lists) | Used by every text column; a parity test sorts a fixture in the browser and in PostgreSQL (the collation of phase 00) and compares |
| `plurals.ts` | i18next ICU with the six Arabic categories | A test checks each counted key in `ar` has `zero`, `one`, `two`, `few`, `many`, `other` |
| `AriaLocale.tsx` | `I18nProvider` locale from i18next | Drives React Aria keyboard direction (DESIGN-SYSTEM 9) |

Locales: `src/i18n/locales/{ar,en}/{common,enums,problems,screens,...}.json`. Arabic is the source language: keys are
added in Arabic first and translated; CI fails when a key exists in one language only. `enums.json` holds a label for
every value of every enum in the generated models, keyed `<EnumName>.<value>`; `problems.json` one message and one
"what to do" line per `ProblemCode` and validation `rule` (DR-76). Labels the API carries as data (reason codes,
purpose codes, workflow step labels, goAML rule messages, scenario and CBI item titles) are rendered from the API in
the user's language and never duplicated in the catalogue. Translation keys are held in component state, never
translated strings (DBS rule).

### 3.4 The API client and the query rules (`src/api/`; DR-75, DR-76)

**Mutator** (`axios-instance.ts`, the Orval `customInstance`): base URL empty (the generated paths carry `/api/v1`;
a test pins it); `withCredentials: true`; `Accept-Language` from i18next; a UUIDv7 `X-Correlation-Id` per request,
kept on the error; on POST, PUT, PATCH and DELETE the `X-CSRF-Token` read fresh from the `akr_csrf` cookie and, when
`contract.gen.ts` marks the operation `idempotency: required`, an `Idempotency-Key` (a UUIDv7 created once per user
intent and kept across retries of that intent, so a retry after a time-out or a step-up replays rather than repeats).

**ETags** (`etag.ts`): responses with `ETag` are remembered per resource URL in memory; mutations whose operation
requires `If-Match` send it; 412 `E_VERSION_CONFLICT` opens the conflict view (both versions, the user chooses; never
an overwrite, IR-26). 428 `E_PRECONDITION_REQUIRED` is a client defect and is reported as such.

**Step-up** (`stepup.ts`): before sending an operation with `stepup` set in the contract, the client opens the
`StepUpDialog` for that action class (and resource where the route binds one), obtains an assertion and sends it in
`X-StepUp-Assertion`; a 401 `E_STEPUP_REQUIRED` on any request except `POST /auth/step-up/verify` (an assertion spent or expired meanwhile) reopens the
dialog with the class from the problem body and retries once with the same `Idempotency-Key`. A refused proof on `POST /auth/step-up/verify` is not resent: the dialog asks for a new proof (a new TOTP code, or a fresh security-key touch). Assertions are held only
in the closure of the one request.

**Problems** (`problem.ts`): every error becomes `{status, code: ProblemCode, title_key, action_key, errors[] (JSON
pointer, rule, bound), correlation_id}`; field errors map to form fields by pointer and to messages by `rule`
(`problems.json`), with the server prose only as the fallback for an unknown rule (DBS decision 63 pattern).

**Session and refusal routing:**
- 401 `E_UNAUTHENTICATED` outside the public prefixes (`/sign-in`, `/_dev/`): clear the query
  cache and the ETag map, keep drafts (section 3.6), navigate to `/sign-in?next=<path>`. On a public prefix nothing
  happens: the shell's own `/me` request is allowed to fail there (DBS decision 74 lesson).
- 404 `E_NOT_FOUND`: the screen renders X6, byte-identical for a missing and a masked resource; the router's
  not-found renders the same component.
- 403 `E_SCOPE` or `E_FORBIDDEN` on a screen's primary query: the Forbidden view with the correlation id, never an
  empty state (IR-70). On a suspicion-tier resource the API answers 404 and the client cannot tell the difference,
  by design.
- 403 `E_CSRF_INVALID`: one silent `GET /auth/session` (which re-mints the cookie) and one retry; a second refusal
  routes to sign-in. 403 `E_CREDENTIAL_CHANGE_REQUIRED`: the first-login stepper (section 3.6). 422
  `E_PURPOSE_REQUIRED`: the purpose dialog (`GET /access/purpose-codes`, phase 03), then one retry with the code
  in `X-Access-Purpose`.
- 429: the message with the `Retry-After` time; 503: "The service is not available" with the correlation id.

**Query defaults** (`main.tsx`): `staleTime` 30 s, `gcTime` 5 min, `retry` 2 on network errors and 502/503/504 only,
never on 4xx; `refetchOnWindowFocus` true; no query persister of any kind. Queries of sensitive operations
(`access` set in the contract, or the operation tagged `goaml`, `coverage` or `change-requests` detail) use `gcTime`
0 and are removed when their screen unmounts (DR-75). Polling only where a screen says so, and only while the tab
is visible (`refetchIntervalInBackground: false`). No server-sent events in P0.

### 3.5 The screen catalogue, routes, rail and route-to-policy test (`src/app/`; UX-01, UX-53; DR-73)

`screens.ts` is the one machine-readable copy of DESIGN-SYSTEM 8.1 (UX-53). Each entry:

```ts
export interface ScreenDef {
  id: ScreenId;                      // 'E1', 'A1', 'G10', ...
  route: string;                     // '/goaml-check'
  pattern: 'LIST' | 'WORK' | 'COMPARE' | 'STUDIO' | 'DASH' | 'REGISTER' | 'STEPPER' | 'INBOX' | 'FORM' | 'message';
  shell: 'reduced' | 'full' | 'bare';  // bare: X1 and X6 render without the rail
  available: 'P0' | 'P1' | 'P2';     // built in this release or not yet
  rail: { group: RailGroup; order: number; label_key: string; icon: IconName } | null;
  roles: Role[];                     // who may open it (DESIGN-SYSTEM 8.2)
  primary: OperationId[];            // the screen renders when the user may call at least one; sections whose
                                     // operations the user cannot call are not rendered
  operations: OperationId[];         // every operation the screen calls (Orval operation ids)
  count?: OperationId;               // the rail badge source, items needing action only (IR-64)
}
```

The entries for screens not yet built (`available` later than the running release) stay in the file with `rail:
null` behaviour at runtime, so the access-matrix report and the tests already know them; their routes are not
registered.

**Rail** (`rail.ts`): from `GET /me` (roles, delegated roles, scopes): a screen appears when it is available, the user
holds one of its `roles` (own or delegated), and the user's scopes include a scope of one of its `primary`
operations (from `contract.gen.ts`). Groups with no visible entry are hidden with their heading. A user with several
roles sees the
union, grouped by role, with the active role in the user chip (DESIGN-SYSTEM 8.2).

**Route guard** (`guard.ts`, TanStack `beforeLoad`): an unauthenticated visitor goes to `/sign-in?next=`; a pending
first-login step goes to the stepper; a route the user's roles do not list renders X6 (never a "no access" page for a
masked kind; a distinct Forbidden view only where existence is not secret, DESIGN-SYSTEM X6). The server decides in
every case; the guard only avoids offering what cannot work.

**Route-to-policy test** (vitest, CI; SP-25, SP-71 front half):
1. Every route in `routeTree.gen.ts` belongs to exactly one `ScreenDef`, and every available `ScreenDef` has a route.
2. Every `operations` entry exists in `contract.gen.ts` with an `action`.
3. For every screen and every role it lists, the role holds a scope of at least one `primary` operation (the operation's
   `security` scopes and the document's `x-akr-role-scopes`, both in `contract.gen.ts`) and `policy-matrix.json`
   grants the role that operation's action. A listed role that the shipped bundle would refuse fails the build.
4. For every role in `policy-matrix.json` and every available screen it may open by rule 3, the screen lists that role
   (no screen hidden from a role that may use it).
5. Every `stepup` operation a screen calls is reachable from a component that renders the `StepUpDialog` (checked by
   a registry the dialog writes in test mode).
A bank that changes its Cedar bundle by change request (phase 04) can make the shipped catalogue stricter than its
policy; the server still refuses, and the screen shows the Forbidden view. The release's matrix is regenerated against
the bank's active bundle by `akr-ctl policy matrix` during the phase 25 release check.

### 3.6 Shell, session and sign-in (`components/shell/`, `screens/x1/`; DESIGN-SYSTEM 7.1, 7.12; UX-35, UX-36; DR-74)

**One shell, two forms.** `AppShell` renders the rail, the header and the content area. P0 ships the **reduced**
form: the rail with the P0 screens the user may open (section 3.5); the header with the bank name and logo at the
start, and at the end, in fixed order, the inbox counter (X3 task half), help (a link to the offline glossary page,
X5 arrives in P1), the language toggle and the user chip with the active role; no global search, no command palette
and no notification bell (X2 and the notification half of X3 are P1). The **full** form adds them in P1 without a
change to any screen. X1 and X6 render **bare** (logo and content only).

- Rail: Lapis, start side, 240 px with labels, collapsible to 64 px (the choice kept in `uiPrefs`), a drawer below
  1024 px; the active entry has the 3 px gold marker, white text and `aria-current="page"`; counts are neutral and
  come from each screen's `count` operation (the inbox count only in P0).
- The data-as-of stamp component reads `data_as_of` from the screen's primary response and is placed by each data
  screen's header; "Data delayed" with the lag when the response says the feed is late (phase 11 fills it).
- Density: from `me.preferences.density`, else the role default of DESIGN-SYSTEM 4.2, as `data-density` on the content
  area; components read it through CSS variables.
- Language: the toggle sends `PATCH /me/preferences` (`If-Match`) and switches i18next, `<html lang dir>` and the
  React Aria locale at once; collections keyed on the locale re-render (DBS trap 7.3).
- Training mark: not in P0 (W12 is P2); the band component exists and is tested.

**Sign-in (X1).** `/sign-in`, bare, FORM: the bank's name and logo above the AKR-TMS lockup; email and password ->
`POST /auth/login`; on `mfa_required` the second factor with WebAuthn first (one button; the browser prompt) and the
TOTP field below (`input-otp`, `autocomplete="one-time-code"`, paste allowed, digits normalised) -> `POST /auth/mfa`;
on `setup_required` or pending steps the **first-login stepper**: change password (`POST /me/credentials/password`,
the policy hints from the problem rules), enrol a factor (WebAuthn or TOTP: QR, the secret as text for typing, confirm
code), acknowledge the undertaking (`POST /me/policy-acknowledgement`, the text in both languages); then `next` or the
user's first rail entry. Failure messages are calm, carry the correlation id and never say which factor failed;
lockout reads "Too many attempts. Try again after HH:MM" from the problem's `retry_after`.

**StepUpDialog** (`components/stepup/`): props `{action_class, resource_ref?, onAssertion}`; `POST /auth/step-up`
then WebAuthn (`navigator.credentials.get` with the returned options) or the TOTP field, `POST /auth/step-up/verify`;
returns the assertion to the caller and closes. Focus is trapped and returned to the invoking control; the screen
under it keeps its state and form values; the dialog names the action in words ("Approve this change", "Download the
report"), from `enums.json` `ActionClass` plus the caller's label. Cancel leaves the mutation unsent.

**SessionWarning** (`components/session/`): reads `idle_expires_at` and `warning_s` from `GET /auth/session` (on
mount and after each heartbeat); at `idle_expires_at - warning_s` a dialog counts down with "Extend session" (sends
`POST /auth/session/heartbeat`) and "Sign out". Real input (key, pointer, wheel; not focus or timers) triggers a
heartbeat at most once per `session.heartbeat_min_interval_s` from the session view, so an unattended screen times out
(phase 01). At expiry the shell routes to sign-in with `next`.

**Drafts** (`components/form/useDraft.ts`): forms that take longer than a minute (the change-request reason, the
declaration drawer, the goAML profile amendment) keep their unsent values in memory across a step-up and across a
time-out within the same tab (held in a module-level map keyed by user id and form id, cleared on sign-out by another
user); nothing is written to browser storage (DR-75). Drafts that must survive a closed tab arrive with the case
drafts of phase 18 (server-side).

### 3.7 The shared component layer (`src/components/`; DESIGN-SYSTEM 7; UX-15, UX-19, UX-25, UX-28, UX-34, UX-38)

Each component exists because a rule must hold on every screen; the rule is enforced inside it, and a test proves it.
Screens use these, not the vendored tree, wherever one exists (lint rule, section 3.2).

| Component | Wraps | Rule it enforces | Test |
|-|-|-|-|
| `AppShell`, `Rail`, `Header`, `UserChip`, `Logo` | `uui` sidebar and header | Section 3.6; gold only here; DOM order is reading order | Rail renders only permitted screens for each fixture role |
| `DataTable`, `columnsFor()` | `uui` table + TanStack Table v9 + TanStack Virtual | Text columns sort through `collator()` (a header sort reorders the rows loaded so far; the list routes take no sort parameter and each keeps the server order its phase states); collection keyed on locale; numbers end-aligned and tabular, Mono for amounts, currency in its own column; sticky header and first column; densities; keyset pagination only ("Load more", never totals where masked rows can exist); `aria-rowcount` and `aria-rowindex` when virtualised; whole row navigable; two-line cells | Arabic fixture sorts alef forms together; switching language re-renders rows; virtualised table exposes counts |
| `StatusBadge` | `uui` badge | Takes a status key, not a colour; the word comes from `enums.json`; tones high, medium, low, info, neutral | A key without a label fails the enum completeness test; no tone without word renders |
| `TierBadge` | | Lock icon plus "Restricted case" or "Restricted customer" on `tier-tint`; never red | Snapshot in both languages |
| `Name` | | Arabic first, larger, `lang="ar"`, chain order; transliteration beneath with source label; never split; wraps rather than truncates; data-quality marks from `scanControls()` | Mixed-script fixture keeps order in RTL and LTR (bounding-box probe in the browser check) |
| `Amount`, `DateTime`, `Ref`, `Identifier`, `Iban`, `Phone` | | One isolate per value; Mono where DESIGN-SYSTEM 3.2 says; copy gives `raw()`; masking per the API's value; `Iban` groups in fours and validates Iraqi IBANs (23 characters, mod 97) on input | Copy returns raw; `2026-10-09 14:32` reads date first in Arabic |
| `DataAsOf` | | "Data as of" plus the time, or "Data delayed" with the lag (medium badge with words) | Both states |
| `Clock` | | Kind and target always; medium due-text in the warning band, "Overdue" after; "paused" with elapsed age | Each state |
| `DiffView` | | Old value struck through in Stone, new value in Ink with an Ishtar start-edge bar; class and volume-effect badges with words | RTL and LTR snapshots |
| `StepUpDialog`, `SessionWarning`, `PurposeDialog`, `ConflictDialog` | `uui` modal | Section 3.4 and 3.6; focus trapped and returned | Keyboard-only flows |
| `Form`, `Field`, `TextField`, `NumberField`, `DateField`, `Select`, `Checkbox`, `Textarea`, `FileDrop` | `uui` inputs + react-hook-form + Zod | Control borders `border-control`; errors under the field with icon and text, translated by rule; numbers accept three digit sets and show the understood value; dates typed in the shown order (`normalizeDate`), unreadable values kept with the fault; free text scanned for card numbers (Luhn) and secret-looking strings before submit (IR-56); `FileDrop` checks type and size before upload and never reads file content | Each rule; no `type="date"` anywhere |
| `SchemaForm` | `Form` | Renders a JSON Schema served by the API (`GET /dsl/{kind}/schema`, profile and setting schemas) with the DBS renderer pattern | Renders the change-request schema and a setting schema |
| `Button`, `ConfirmDialog` | `uui` buttons and modal | One primary per region; destructive actions outline in `risk-high`, filled only in the confirmation that names consequence and target; inline progress during their own mutation | |
| `EmptyState`, `Forbidden`, `NotFound` (X6), `ErrorPanel` | | One sentence and one action; a refusal never looks empty; `NotFound` byte-identical for missing and masked; errors show what happened, what to do and the correlation id | X6 output identical for both causes |
| `Skeleton`, `Progress` | `uui` loading | Skeletons for lists; a percent indicator for jobs beyond about 10 s (IR-73) | |
| `Stepper` | | Numbered steps, validation per step, back without losing values (the STEPPER pattern) | |
| `PrintPage` | | A4 print stylesheet: product fonts, no rail or header, page header with the screen title, run id and time, page numbers | Print emulation in the browser check |
| `Help` (`?` popover) | `uui` tooltip | Text from the offline glossary JSON shipped with the bundle | |
| `HealthDot` | | Platform health only, word "Healthy" with the dot; never on business screens | |

Charts: none in P0 screens except share bars and range bars from `components/charts/` (in-house SVG, DESIGN-SYSTEM
7.15), each with a sentence, a data table alternative and words on every segment. The shortcut layer (7.18) is P1;
P0 registers no single-key shortcuts.

### 3.8 The P0 screens (`src/screens/`; DESIGN-SYSTEM 8)

Every screen: designed right to left first and mirrored; every string in both languages; the data-as-of stamp on data
screens; loading skeletons; an empty state with one sentence and one action; refusals routed as in section 3.4; the
interaction-rule checklist (UX-54) in its pull request. Roles are the catalogue's (section 3.5); "hooks" are the
generated ones of phases 01 to 07.

| ID | Route | Shell, pattern | Roles | Primary | Phase of the API |
|-|-|-|-|-|-|
| X1 | `/sign-in` | bare, FORM | everyone | `postAuthLogin` | 01 |
| X6 | any unknown or masked | bare, message | everyone | none | 00 |
| X4 | `/me` | reduced, FORM | every human role | `getMe` | 01 |
| X3 | `/inbox` | reduced, LIST (task half) | every internal role (not L, VS) | `getInbox` | 04 |
| G10 | `/changes`, `/changes/$id` | reduced, LIST + detail | CFG, DS, MRO, CHK, M, CH, ADM, SEC, IA | `getChangeRequests` | 04 |
| A1 | `/admin/users`, `/admin/users/$id` | reduced, LIST + detail | ADM, SEC | `getAdminUsers`, `getAdminCredentialResets` | 03, 04 |
| A2 | `/admin/settings` | reduced, FORM | ADM | `getAdminSettings` | 03, 04 |
| A3 | `/admin/health` | reduced, DASH | ADM | `getAdminOps` | 03 |
| T1 | `/data/connection`, `/data/connection/$id` | reduced, DASH + detail | DS, SEC (list, pause, resume and documents) | `getSources` | 05 |
| E2 | `/discovery` | reduced, DASH | DS, M, CH, IA, CFG | `getCoverageCbiGapReport`, `getCoverageReadiness` | 05, 06 |
| E1 | `/goaml-check`, `/goaml-check/runs/$id` | reduced, STEPPER | A2, M | `getGoamlCheckRuns` | 07 |
| E1p | `/goaml-check/profiles`, `/goaml-check/profiles/$id` | reduced, REGISTER + detail | A2, M, CFG, CHK | `getGoamlProfiles` | 07 |

E1p is the goAML profile page of phase 07 section 5, a sub-screen of E1 in the catalogue (same ID family, own entry so
CFG and CHK reach it without the checker).

**X6 Not found.** One component for the router's not-found, every 404 and every masked refusal: the same title,
sentence ("This page does not exist or is no longer available"), a link to the user's first rail entry and the
correlation id; byte-identical output for a missing and a masked resource (a test renders both and compares). The
Forbidden view (section 3.4) is separate and used only for scope refusals where existence is not secret.

**X4 Profile, security section.** Language, density and prose digits (`PATCH /me/preferences`, `If-Match`); the
theme choice is hidden while DS-O1 is open. Sessions (`getMeSessions`; this device marked; revoke). Second factors
(`getMeCredentials`; add WebAuthn or TOTP with the enrolment flow of X1; remove with the `StepUpDialog`,
`credential_change`; the 409 `E_MIN_FACTORS` explained). Personal access tokens (`getMeTokens`; create with a scope
subset of the user's own scopes and an expiry, step-up `token_create`, the secret shown once in a copy field with the
words "You will not see this again"; revoke). Password change. The undertaking version acknowledged and its date.

**X3 Inbox (task half).** Views as tabs: "Mine", "My pools", "Waiting on others", "History" (`getInbox` with
`view`); counts from `getInboxCounts` (polled every 60 s while visible; the rail badge reads the same query). Rows:
the step label in the user's language, the subject summary the API gives (never more), due time with `Clock`
(overdue in words), escalated flag, claimed by me or open. Opening a row shows the **task panel** (`getTasksId`) at
the end side: subject summary, step and form fields, the timeline slice, and only the actions whose affordance is true
(claim, release, approve, reject with comment, sign, submit form, review), each disabled with its reason word when
the affordance says why not. Deciding actions open the `StepUpDialog` with the class from the contract; 409
`E_CLAIMED` and `E_STALE_ASSIGNEE` are explained in words, and 403 `E_SELF_APPROVAL` is explained in words ("Someone else claimed this task", "You
proposed this change, so another approver must decide it"). For a change-request task the panel links to G10 detail for a role that G10 lists (the screen table above) and shows the reference of the request as text for any other role (an A2 maker of a corpus case, section 7.11 item 3).
Rows the API masks never appear; the counts are the API's (post-filter).

**G10 Change requests.** List (`getChangeRequests`, keyset; filters kind, class, status, mine, awaiting me,
emergency, as phase 04 offers) with columns reference, kind word, object, class and volume-effect badges with words,
maker, status, age. Detail (`getChangeRequestsId`): the `DiffView` of before and after; class, volume effect and the
route (the steps and who decides each, by role); reason code with label and text; evidence list; the impact block
when the target provides one (the goAML corpus result, the readiness change); tasks and timeline. Actions by role and
state: submit and withdraw for the maker (`If-Match`), approve and reject on the caller's own task (step-up
`approval`), never shown to the maker (four-eyes explained in the request's terms). Creating a request happens where
the object lives (A1 grant, A2 setting, T1 budget and access, E1p profile); G10 has no generic "new" button in P0
because `generic_create` kinds have no P0 editor except settings and grants.

**A1 Users, roles and leave.** For ADM: the user list (`getAdminUsers`, filters status, kind, role, branch, locked,
text); create (step-up `identity_admin`; the one-time password panel shown once); detail with profile, grants (each
with its source change request), delegations, token register, lock and reset state; actions suspend, re-enable,
offboard and reset credentials (step-up each), unlock; **propose a role grant** (`postAdminUsersIdGrants`: role,
branch scope, expiry, reason code and text; answers 202 with the change request, shown with a link to G10); revoke a
grant (`deleteAdminUsersIdGrantsGrantId`). Branches and pools as two tabs (`getOrgBranches`, `getOrgPools`, create and
edit). For SEC: only the pending-reset list (`getAdminCredentialResets`) with confirm (step-up `identity_admin`,
four-eyes: the initiator cannot confirm; the temporary password shown once). Delegations (leave) for any user are in
X4 and A1 detail (`getOrgDelegations`, create with step-up when the phase 03 rule says so).

**A2 Settings.** Settings grouped by key prefix with label, value, unit, floor and its basis ("CBI minimum"), bounds,
the governed mark; non-governed keys edit in place (`putAdminSettingsKey`, `If-Match`, 200); governed keys show
"Propose a change" (the same route with reason code, text, optional effective time and evidence; 202 with the change
request); a value under the floor is refused in the field with the floor named (`E_BELOW_FLOOR`). Flags as a second
tab (`getAdminFlags`, `putAdminFlagsKey`), each with its label and the effect sentence the API returns.

**A3 Platform health.** The cockpit tiles of `getAdminOps` (jobs, outbox, timers, backups and drills, anchors and
verification, partitions, evidence scanning, disk, certificates, OpenBao seal, Valkey, bundle, the phase 05 sources
line, the phase 07 goAML watcher line), each a word state with `HealthDot` and the measured value and its time; a
tile in trouble opens its list. Jobs (`getAdminJobs`, filters kind and status; never the payload) with retry on dead
jobs (`postAdminJobsIdRetry`). Refreshed every 60 s while visible. VS reaches platform health with support sessions in
a later phase; in P0 the screen is ADM's.

**T1 Data connection (basic).** Sources list (`getSources`): name, tier, state word, proved read-only level as words
("Read-only verified at 2026-10-09 14:32"), last contact, open pause. Detail (`getSourcesId`): the grant check with
findings by privilege name (`getSourcesIdGrants`; "Run check now" -> `postSourcesIdProbe`, 202, polled), the budget in
force and its history with "Propose a budget" (`putSourcesIdLoadBudget`, a document reference to the DBA-signed
file, 202 change request), the approved object list with "Propose access" (`putSourcesIdAccess`), documents (upload
through `FileDrop` -> `postSourcesIdDocuments`; read), and the kill switch: "Pause now" in `risk-high` outline with a
confirmation naming the source and the consequence, step-up, then "Resume" (only after a fresh passing check; the
API answers 202 with the check id and the source stays paused until that check passes, and the screen says why). Discovery runs per source (`getDiscoveryRuns`, start a catalogue or
profile run, cancel). Feed volume, replica delay, reconciliation and quarantine (DESIGN-SYSTEM T1) arrive with phases
11 and 12; their places show "Available when production ingestion starts". SEC sees the list and the detail
header, the kill switch and the documents (phase 05 scopes, widened by this phase so SEC can reach the source it
pauses).

**E2 Discovery and CBI coverage gap report.** As phase 06 section 5 specifies, on the DASH pattern: the sources band
(phase 05 hooks; DS only), the coverage grid of the 173 items (`getCbiCatalogue` joined with `getCoverageReadiness`;
grade chip with its word, linked scenarios with restricted ones as "Restricted scenario", missing fields; filters by
grade, kind and declaration status; for M, CH and IA the grid reads the same rows), the declaration drawer (M only;
`putCoverageReadinessIdDeclaration` with `If-Match` and step-up `readiness_declare`; owner picker of active humans;
review date bounded by the setting), the field candidates panel (DS; "Use as hint" -> `patchSourcesId`), and the
report bar ("Generate gap report" for DS and M -> `postCoverageCbiGapReportRuns`, polled every 3 s; the stale banner;
the four pack downloads, each with step-up `pack_download`, and "Integration profile PDF" per source for DS and M).
The roles see only the parts their operations allow (section 3.5, sections not rendered).

**E1 goAML checker and E1p profiles.** As phase 07 section 5 specifies: step 1 files (`FileDrop`, XML or ZIP, the
limits from the run settings shown; the paste box; the optional Office message enabled with exactly one file; the
profile picker with "Not yet in force" candidates for M only, since CFG has no checker), step 2 progress
(`getGoamlCheckRunsId` every 2 s), step 3 result per file with the outcome word, the estimated grade always with the
word "estimated" and the note that the Office grades its own way, and findings grouped by layer
(`getGoamlCheckRunsIdFindings`) with path, line, column, explanation and fix from the API in the user's language, the
other language one control away; `PrintPage` gives one A4 page per file. Earlier runs list (no file names in the
list). E1p: versions, version detail (patches, rules with severity and parameters, limits, emit flags, open
questions, the diff from base as `DiffView`, the corpus result), "Propose change" for CFG (`putGoamlProfilesId` with
`op` promote or amend, built with `SchemaForm` over the profile schema; 202 with the change request), the watcher runs
and "Check now" for M and CFG, hidden with the reason shown while the watcher is off. The panel **Bank cases** of E1p is in section 7.11.

### 3.9 Spikes (time-boxed; results recorded as decisions before P1 builds on them)

1. **RTL charts** (DS-O3, DS-O7; tech-stack.md open item "chart and graph libraries in RTL"): one day each for
   Recharts and the in-house SVG components on three charts (a time series with a reference line and Ramadan band, a
   stacked bar with direct labels and 2 px gaps, a share bar) in Arabic and English, measured for: time-axis direction
   in RTL, label isolation and digit rendering, the table alternative, bundle size, keyboard access. The result picks
   the chart library for P1 (or none) and the time-axis rule, and fixes the Arabic leading values of DESIGN-SYSTEM 3.3
   against marked text (shadda with kasra on the line below), each recorded as a DR with screenshots in
   `docs/spikes/`.
2. **Network graph** (UX-10, DESIGN-SYSTEM 7.16): `@xyflow/react` with 100 nodes and grouping above 15 accounts, in
   RTL (geometry not mirrored, labels isolated), keyboard traversal and the node and link table; target 100 nodes in
   2 s on the reference workstation. Outside the bundle (a spike route under `/_dev/`, development builds only).
3. **Fonts by measurement** (DBS 12.5 verification 4): `document.fonts` reports the three faces loaded, and a
   `getBoundingClientRect()` probe of a Latin, an Arabic and a mixed string matches the expected face per script; any
   fallback fails.

### 3.10 Errors, audit events, Cedar actions, tipping-off

**Error mapping.** No new codes. Every `ProblemCode` has an entry in `problems.json` (a test enumerates the generated
enum).

**Audit events emitted:** none new. Every mutation the screens make is an existing route whose audit event its phase
names (phases 01, 03, 04, 05, 06, 07). The browser writes nothing to the audit trail of its own; client errors are
not shipped anywhere (no telemetry endpoint; no external error service; IR-83).

**Cedar actions touched:** none new. The catalogue's roles are tested against the shipped bundle (section 3.5).

**Tipping-off review.**
- The rail, the counts and the inbox show only what the API returns after its visibility filter; a masked task,
  change request or check run never appears, and the counts are the API's (no client-side totals).
- X6 is byte-identical for a missing and a masked resource; no "no access" page exists for a masked kind.
- The browser tab title is the screen name only, never a customer, case or file name.
- No API data is kept after sign-out or in browser storage; sensitive queries are dropped when their screen unmounts;
  the print page of E1 carries no file content (the API has none) and is produced by the user's own browser.
- Neutral wording rules (DESIGN-SYSTEM 11) apply from the first screen: no screen of this phase is seen by branch or
  operations staff (L and OPS have no P0 rail entry besides the inbox, whose items carry the step label and subject
  summary the API composes for them).
The answer for a person outside an object's access list: nothing.

**Settings and configuration used:** none new. Read: `session.idle_timeout_s`, `session.warning_s` and
`session.heartbeat_min_interval_s` (through `GET /auth/session`), `stepup.ttl_s` (through the challenge), the goAML
check limits (through the profile and run responses), the declaration review bound (phase 06); `me.preferences`.
Install-time: the bank's name in Arabic and English, which X1 needs before sign-in, comes from `/branding.json`, a
static file Caddy serves from the installation's `deploy/branding.json` (written at installation, no API, no
secret; phase 00 section 7.38 built the route and fixed the shape: `{ "bank_name": { "ar": "...", "en": "..." } }`,
with `AKR_BRANDING_FILE` naming the file that is mounted); the bank's logo arrives with the `brand.bank_logo` setting in P1 (DESIGN-SYSTEM 12), and until then the
header shows the name only.

## 4. Infrastructure Updates

- **Jobs:** no new jobs, outbox events or timers.
- **Docker compose and the edge:**
  - The frontend build (`pnpm build`) produces static assets copied into the Caddy image (phase 00 serves `/` from
    them); no Node runtime in production.
  - Caddy adds the security headers for `/`: `Content-Security-Policy: default-src 'self'; script-src 'self';
    style-src 'self'; img-src 'self' data:; font-src 'self'; connect-src 'self'; frame-ancestors 'none'; base-uri
    'none'; form-action 'self'; object-src 'none'` (no inline script: the theme stamp is a tiny external
    `theme-boot.js` loaded first), `Strict-Transport-Security`, `X-Content-Type-Options: nosniff`, `Referrer-Policy:
    no-referrer`, `Permissions-Policy` denying camera, microphone, geolocation, payment and USB, and
    `Cache-Control: no-store` on `index.html` (hashed assets are immutable for a year). `/branding.json` is served
    with `no-cache`.
  - Development: `vite` on the host proxies `/api` to akr-api; no new compose service.
- **CI (web pipeline of tech-stack.md, extended):** `pnpm install --frozen-lockfile`; `pnpm licenses list` check;
  `tsc -b` (zero errors, vendored tree included); `eslint`; `scripts/lint-design.mjs`; `scripts/gen-tokens.mjs` and
  `scripts/check-contrast.mjs` (drift and contrast); the Orval, contract and policy-matrix drift check; `vitest run`;
  `vite build` with the bundle budget (section 6); Playwright with axe against the dev stack (section 6).
- **AppState additions:** none in the binaries.
- **OpenAPI registration:** no new operations; the extensions and schemas of section 2.
- **Network zones:** none. The browser talks to the edge only; fonts, icons and help are in the bundle (no external
  request of any kind, enforced by the CSP and by a Playwright check that records every request a session makes).
- **Existing entity, schema or route updates:**
  - Phase 00: Caddy headers and `/branding.json`; the web pipeline steps; `.claude/rules/frontend.md` ported from the
    DBS (AKR: the screen catalogue, the design lint, the refusal routing of section 3.4, no browser storage) and
    `.claude/rules/form-templates.md` ported for the bank's own letter and report templates used from phase 21 (AKR:
    rendered through Gotenberg).
  - Phase 01: `x-akr-stepup` on SU operations; `ActionClass` and `ProblemCode` schemas; `x-akr-role-scopes`; the
    `security` scopes on every operation.
  - Phase 00 and 01: `x-akr-idempotency` from the exemption list.
  - Phase 03: `akr-ctl policy matrix --json`.
  - Phase 06: `GET /coverage/readiness` scopes `config.author, dashboard.read` and Cedar `readiness.read` for M, CH and
    IA (found while specifying E2: the MLRO declares on rows it could not read); API.md 3.7 aligned.
  - Phase 05: `GET /sources` and `GET /sources/{id}` scopes `data.steward, security.ops`, Cedar `source.list` and
    `source.read` for SEC (found while specifying T1: SEC could pause a source it could not find); API.md 3.5 aligned.
  - Phase 07: the candidate profile picker is M's (CFG has no checker); section 5 aligned.
  - Planning docs aligned in the same change: DESIGN-SYSTEM 8.1 (G10 added, X3 route `/inbox` with the task half in
    P0, E1 sub-screen, count 62), tech-stack.md (Playwright and axe in the web pipeline), roadmap row 08.

## 5. Frontend Impact

This phase is the frontend. The work lands in five stages; each ends green on its own checks before the next starts.

1. **Foundation:** the project skeleton of section 3.1 over the phase 00 scaffold; Untitled UI vendored, imports
   rewritten, `migrate` run, dark selector changed; tokens, generator, contrast check and design lint; fonts and the
   glyph test; the i18n kit with its tests; `AriaLocale`; the mutator, ETag, problem and step-up modules; the
   contract generator and the policy matrix. Gate: `tsc`, lint, contrast, kit tests, glyph test.
2. **Component layer:** section 3.7, each component with its test and both-language snapshots. Gate: component tests,
   axe on a component gallery route (`/_dev/gallery`, development builds only).
3. **Shell:** reduced, full and bare forms; rail from the catalogue; route guard; sign-in, the first-login stepper,
   `StepUpDialog`, `SessionWarning`; X6; the route-to-policy test. Gate: sign-in and step-up flows in the browser in
   both languages; route-to-policy test.
4. **Screens on phases 01, 03 and 04:** X4, X3, G10, A1, A2, A3. Gate: section 6 items for these screens.
5. **Screens on phases 05, 06 and 07:** T1, E2, E1 and E1p; the spikes of section 3.9. Lands after those phases.
   Gate: the whole of section 6.

Generated-client surface consumed: the hooks listed in section 5 of phases 01, 03, 04, 05, 06 and 07 (those used by
the screens of section 3.8; the workflow-definition and inventory hooks are not used in P0).

## 6. Verification

1. `make migrate`: no migration in this phase; the chain 0001-0025 still applies on an empty database.
2. `cargo check`, `cargo clippy --all-targets -- -D warnings`: the OpenAPI additions compile; tests: every SU
   operation has `x-akr-stepup` naming a declared class; every mutating operation has `x-akr-idempotency` and the
   values equal `common::idempotency::EXEMPT`; `ProblemDetails.code` is the `ProblemCode` enum; `akr-ctl policy
   matrix --bundle backend/policies/bundle --json` lists each role's actions and its `bundle_hash` equals `policy
   validate`'s.
3. `make generate-api` twice: zero diff over `src/api/generated`, `src/api/models`, `contract.gen.ts` and
   `policy-matrix.json`.
4. Frontend checks: `pnpm install --frozen-lockfile`; `tsc -b` zero errors; `eslint` clean; `lint-design` clean (a
   seeded `ml-2` in a screen fails it; `text-gold` outside the two files fails it); `check-contrast` passes (a Locked
   value changed in `tokens.json` fails it; the amber fill used as text fails it); `vite build` with the main bundle
   at most 450 KB compressed script for the reduced shell and the P0 screens (route-split; the E and A screens load on
   demand).
5. `vitest run`, including: the kit (dates, isolates, digits in three scripts, folds with the server parity fixture,
   collation parity with PostgreSQL, six-form plurals, Hijri labels, money); the mutator (base URL not doubled, CSRF
   header, `Idempotency-Key` on required operations only and kept across one retry, `If-Match` sent, step-up retry
   once); the enum and problem catalogue completeness (every generated enum value and every `ProblemCode` in both
   languages); the route-to-policy test of section 3.5 (and a fixture catalogue listing CFG on A1 fails it); X6
   byte-identity; component tests of section 3.7; no `localStorage` outside `uiPrefs.ts` (lint and a runtime spy).
6. Browser checks (Playwright against the dev stack with the phase 01 dev users, both languages, light theme;
   DR-77):
   - axe on every route of the catalogue that the user may open, for one user per role: zero violations at WCAG 2.2 A
     and AA.
   - Fonts by measurement (section 3.9 item 3); the glyph page renders every letter of `glyphs.txt` with no
     fallback.
   - Sign-in as `m@bank.test` with password and TOTP pasted into the code field; the first-login stepper for a new
     user created in A1 (change password, enrol TOTP, acknowledge); the session warning appears at idle minus 60 s
     with the test clock and "Extend session" keeps the session; at expiry the user lands on `/sign-in?next=`.
   - Rail by role: A2 sees E1 and the inbox; DS sees E2, T1, G10 and the inbox; ADM sees A1, A2, A3, G10 and the
     inbox; SEC sees A1 (resets only), T1 (pause and documents) and G10; CFG sees E2, E1p and G10; no user sees an
     entry whose primary operations it cannot call.
   - X6: `/changes/<random uuid>` and `/goaml-check/runs/<another preparer's run>` (as A2b) render byte-identical
     pages (DOM text and structure compared).
   - G10 and X3: ADM proposes a governed setting change in A2; the decider named by the route opens the inbox, opens
     the task and approves with the `StepUpDialog` (TOTP); the maker sees no approve button on their own
     request; a 412 from a stale `If-Match` opens the conflict dialog with both versions.
   - A1: ADM proposes a role grant -> the change request link; SEC confirms a reset initiated by ADM; ADM cannot
     confirm their own reset (no button; the API's 403 if forced).
   - E1: A2 uploads the sample STR and a file with an apostrophe in a first name: progress, then "Accepted, estimated
     grade: ..." with the word "estimated" and "Would be rejected" with `XSD-04` at its line; the print emulation gives
     one A4 page per file; no file content appears anywhere in the DOM beyond the names the requester typed.
   - E2: DS generates the gap report and downloads the Arabic report PDF with step-up; M opens the D row and records
     a declaration with step-up; CFG sees the grid without the report bar.
   - T1: DS runs a grant check and sees "Read-only verified" with the time; pauses the source with the confirmation
     and step-up; resume answers 202 and the source stays paused until a fresh check passes, with the reason in words.
   - Mirroring spot checks with `Range.getBoundingClientRect()`: a date-time reads date first in Arabic; an amount
     with its code keeps figure and code together; a mixed Arabic and Latin name keeps chain order.
   - Network: the recorded requests of a full session go only to the app's origin.
   - Keyboard-only pass through sign-in, step-up, the inbox decision and the E1 stepper; focus visible and never
     hidden under the sticky header.
7. Audit: after the browser run, `audit_log` holds the events of the routes the screens called (phase 01 sign-in and
   step-up events, `change_request.*`, `user.*`, `setting.*`, `source.*`, `goaml_check.*`, `readiness.declared`,
   `pack.downloaded`) and no event the API did not cause; `akr-ctl audit verify` passes.
8. No CBS connection is made by this phase beyond what the phase 05 routes do when T1 calls them (the grant check
   through akr-ingestd); the conformance suite is unchanged.
9. Phase 00 to 07 tests still pass; the phase 06 curl suite passes with the added M read.

## 7+. Gap Additions

Gap pass 1 (2026-10-10), run 1: corrections found while checking phases 00 to 02 (tools/agent/gap1-run1-verdict.md); lead-verified.

### 7.1 Step-up names in the coverage screens
The declaration drawer's step-up is the action `readiness.declare` (class `approval`) and each pack download's
step-up is the action `pack.download` (class `export`), as phase 06 7.1 records; the classes `readiness_declare` and
`pack_download` named in section 3 do not exist.

Gap pass 1 (2026-10-10), run 3a, from tools/agent/gap1-run3a.md; lead-verified (tools/agent/gap1-run3a-verdict.md).

### 7.2 RailGroup values (DR-369)
`RailGroup` is a closed union, one value per rail group of DESIGN-SYSTEM 8.2 (two entries may share a group, section 7.8): `my_work`, `command`, `decisions`,
`alerts_cases`, `screening`, `customers`, `kyc_reviews`, `pep`, `rfi`, `qa`, `ops_tasks`, `scenarios`, `policy`,
`governance`, `data`, `reports`, `models`, `assessor`, `administration`, `access_insider`, `audit`,
`platform_health`, `help`. The branch window is a separate bundle and has no group. `screens.ts` lists every screen of
DESIGN-SYSTEM 8.1, including D3 (phase 09, group `policy`) and T3 (phase 10, group `data`), with `available` later
than the running release until their phase.

### 7.3 The Name component (UX-25, DR-373)
The section 3.7 table gains the row `Name`: an original name and its transliteration, each in its own direction
(isolated with `dir`), and the quality flags in words (never colour alone). This phase builds it, since it carries
UX-25. Phase 14 extends it with the chain alignment (father and grandfather), the mother's name and the screening
flags for S1 and the customer screens.

Gap pass 1 (2026-10-10), run 7a (coverage of UX and W IDs, gap item (ab)), from tools/agent/gap1-run7a.md; lead-verified (tools/agent/gap1-run7a-verdict.md).

### 7.4 Usability round 1 (UX-03; ROADMAP track A; DS-O4)
Usability round 1 runs with three to five Iraqi compliance staff per group (MLRO team, analysts, branch liaison), Arabic
first, on paper and clickable prototypes of the P0 screens. Its dated record in docs/plan/research.md settles DS-O4
(the order of numeric ranges); until then `formatRange()` keeps the left-to-right order. No code, route or setting is
added.

### 7.5 Accessibility release gate (UX-39; ROADMAP P1 accessibility pack; DS-O5)
The axe run of section 6 stays in CI. Each release adds a signed checklist, stored with the conformance statement:
keyboard-only walkthroughs of the six core tasks (open a case, record a disposition, request information, escalate,
sign a report, answer an RFI); 400 percent zoom (320 CSS px reflow) in Arabic on those screens; NVDA and VoiceOver
passes on the same tasks with mixed-script names. A failed item blocks the release.

Verification pass (2026-10-10), from tools/agent/verify/; lead-verified (tools/agent/verify-verdict.md).

### 7.6 Risk or capacity tag on the change-request screens (V-04-06-a, LOW; DR-413)
Screen G10 shows the tag of the reason (`ReasonTag`, phase 04; `ref.reason_codes.tag`: `risk` or `capacity`) beside
the reason code on the detail and beside the class badge on the list (the list has no reason-code column), as a badge
with words (UX-60). A capacity tag carries the
capacity-wording flag of DESIGN-SYSTEM section 7.17.

### 7.7 The code points of scanControls (V-08-08-a, LOW; DR-413)
`scanControls(s)` in the section 3.3 `bidi.ts` row returns offsets only and changes no text. `bidi` flags U+202A to U+202E, U+2066 to U+2069, U+200E, U+200F and U+061C. `zero_width` flags U+200C, U+200D and U+FEFF. The sets are those of research/31-analyst-ux.md section 4.2 rule 4 and of research/22-arabic-name-matching.md section 6.2 (row "Invisible format characters"). Removal for matching is phase 14 N3 (section 3.1).

### 7.8 The board pack rail entry (V-08-10-a, LOW; DR-413)
The CH rail entry `board pack` of DESIGN-SYSTEM 8.2 is the R4 screen (`/packs`, the board pack and evidence pack builder of DESIGN-SYSTEM 8.1). It takes `RailGroup` `reports`, the group of the R series; the closed union of section 7.2 gains no value.

### 7.9 The transliteration under a name (V-08-11-a, MEDIUM; DR-413)
The label rule of this block is replaced by section 7.10 (owner decision of 2026-10-10: the four labels are kept and the bank's mapping sets the source of each name); what follows stands. The `Name` component shows what the source gives. The original is the party's `canon.party_name` row in Arabic or Kurdish script; the transliteration beneath it is the row of the same party and `name_type` with `script = 'lat'`, when the source supplied one. A party with no Latin row shows no second line. The product generates no Latin form and reads no document.

Owner decisions (2026-10-10): the product owner overturned the lead decision of section 7.9; lead-written, checked in the owner-decision round (tools/agent/owner-verdict.md).

### 7.10 The four source labels of a Latin name (owner decision 9; DR-420)
The owner's decision: the labels `card`, `passport`, `typed` and `generated` of DESIGN-SYSTEM 7.6 are all used. This block replaces the label rule of section 7.9; the rest of section 7.9 stands (which row is the original, which is the transliteration, and no second line when the party has no Latin row).

1. **Data.** Each name row that the API returns carries `capture_source` (phase 10 section 7.26): `card`, `passport`, `typed` or `generated`. The bank's mapping sets it; a name with no stated source is `typed`.
2. **Component.** `Name` takes `latin?: { text: string; source: 'card' | 'passport' | 'typed' | 'generated' }`. The source label is shown after the Latin line, smaller, as DESIGN-SYSTEM 7.6 says, and in the secondary text colour (`--color-stone`), as a word and never as an icon or a colour alone. The label is shown for the Latin line only; the original line has none.
3. **Words** (`src/i18n/locales/{ar,en}/enums.json`, keys `name_source.<value>`):

| Key | Arabic | English |
|-|-|-|
| `name_source.card` | من البطاقة الوطنية | from the national card |
| `name_source.passport` | من جواز السفر | from the passport |
| `name_source.typed` | أُدخل يدوياً | typed |
| `name_source.generated` | مولَّد آلياً | generated |

4. **What does not change.** The product still generates no Latin form and reads no document: `generated` is shown only when the bank's mapping says that a program of the bank produced the name. `Name` shows only a Latin row that the source gave; a back-transliteration made for matching (phase 14) is never passed to it.
5. **Tests** (added to the `Name` row of section 3.7): the component renders each of the four labels in both languages from a fixture; a fixture with no `latin` renders one line; the mixed-script order probe of section 3.7 passes with a label present.

### 7.11 The panel Bank cases on E1p (owner decision 2; DR-415)
Phase 07 section 7.6 lets the bank add its own cases to the goAML regression corpus. E1p shows them and takes the proposals. This block adds one panel to a screen this phase already builds; it adds no screen, no route and no rail entry.

1. **Where.** E1p (`/goaml-check/profiles`) gains the panel **Bank cases** under the version list, read with `getGoamlCorpusCases` for the profile code of the version list (`IQ` unless the bank keeps a second profile code). A2, M, CFG and CHK see it (the roles of E1p). The actions of items 3 and 4 are rendered for A2 and M only; CFG and CHK read.
2. **List.** A `DataTable`: the key, the expected result in words (`corpus_case.expect.would_reject` with its rule codes, or `corpus_case.expect.graded`), the state word, the last intake result in words, and the date added; a state filter. There is no file column and no download: the API returns no file.
3. **Add a case** (A2, M). A form: `FileDrop` limited to one XML file (the client checks the type only; no response open to A2 or M gives the size limit, so a file that is too large is the server's 422 `file_too_large`, shown at the file field); the key (it starts with a lower-case letter, then lower-case letters, digits or hyphens; 3 to 61 characters in all; under the field the hint "A short code for the rule. Never a name." in the user's language, phase 07 section 7.6 item 13); the expected outcome (two choices, with the words of `corpus_case.expect.would_reject` and `corpus_case.expect.graded`); the expected block codes, a multi-select over the rule codes of the active version, enabled only with the first choice; an optional Office message code, a select over the active version's rejection patterns; and the reason code and reason text of the propose form. Above the submit button stands the notice "The file is masked before it is kept: names, numbers and text are replaced. The original is deleted." in the user's language. Submit sends `postGoamlCorpusCases` (multipart) with a generated `Idempotency-Key` and with `profile_code` = the profile code of item 1. On 202 the page shows the reference of the change request: as a link to G10 for M, whom G10 lists, and as text for A2, whom G10 does not list and who follows the request in the inbox tab "Waiting on others" (X3). An A2 maker therefore has no withdraw screen in this version; an unwanted proposal is closed by the approver's reject (phase 07 section 7.6 item 7). A 422 shows the field message for `file_too_large`, `type_not_allowed`, `case_key_taken`, `expectation_invalid`, `rule_unknown` or `too_many_cases`.
4. **Retire** (A2, M), on an `active` row: a confirm dialog with the reason code and text, then `postGoamlCorpusCasesIdRetire`; on 202 the reference of the change request, shown as in item 3.
5. **Words** (`src/i18n/locales/{ar,en}/enums.json`):

| Key | Arabic | English |
|-|-|-|
| `corpus_case.state.proposed` | مقترحة | Proposed |
| `corpus_case.state.active` | فعّالة | Active |
| `corpus_case.state.retired` | مُنهاة | Retired |
| `corpus_case.state.rejected` | مرفوضة | Rejected |
| `corpus_case.expect.would_reject` | سيُرفض | Would be rejected |
| `corpus_case.expect.graded` | مقبول | Accepted |
| `corpus_case.intake.pending` | بانتظار الفحص | Waiting for the check |
| `corpus_case.intake.met` | تطابق النتيجة المتوقعة | Meets the expectation |
| `corpus_case.intake.not_met` | لا تطابق النتيجة المتوقعة | Does not meet the expectation |
| `corpus_case.intake.unsafe_file` | تعذّر الفحص: ملف غير آمن | Could not be checked: unsafe file |
| `corpus_case.intake.not_maskable` | تعذّر الفحص: لا يمكن حجب بيانات الملف | Could not be checked: the file cannot be masked |
| `corpus_case.intake.file_gone` | تعذّر الفحص: الملف لم يعد موجوداً | Could not be checked: the file is gone |
| `corpus_case.intake.profile_unavailable` | تعذّر الفحص: لا يوجد ملف تعريف فعّال | Could not be checked: no active profile |
| `corpus_case.intake.validator_failed` | تعذّر الفحص: فشل المدقق | Could not be checked: the validator failed |

6. **Corpus result.** In the version detail, the corpus result lists the bank's cases after the shipped ones, each marked with the word "Bank" (`corpus_case.origin.bank`: من المصرف, Bank), and shows the count from `bank_cases`.
7. **Tests** (added to the E1 rows of section 6): as A2 the panel shows "Add a case" and a proposed case appears with "Waiting for the check"; as CFG the panel lists the cases and renders neither "Add a case" nor "Retire"; the form with the choice `corpus_case.expect.graded` ("Accepted") disables the block-code select; the panel passes axe in both directions and the keyboard-only pass reaches every control.

## What This Phase Does NOT Touch

- No new tables, migrations, settings, flags, jobs, audit events or Cedar actions.
- No P1 screens (W, S, K, D, R, B, G1 to G9, T2 to T4, X2, X5, A4) and no P1 shell parts (global search, command
  palette, notification bell, shortcut layer, live updates over server-sent events, saved views).
- No dark theme until DS-O1 is approved; no chart library until the spike decides; no network graph in the bundle.
- No workflow-definition editor or inventory screens: ADM authors definitions with `akr-ctl` and Swagger UI until
  phases 26 and 27 bring the designer.
- No branch window bundle (phase 22) and no operations-task inbox for OPS (phase 20).
- No offline help centre content beyond the glossary file (X5 is P1).
- No change to any backend behaviour except the OpenAPI extensions, the two schemas, the matrix command, the phase 05
  source read scopes and the phase 06 readiness read scope.
