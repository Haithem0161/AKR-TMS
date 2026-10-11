---
paths:
  - "backend/api/src/auth/**"
  - "backend/api/src/middleware/**"
  - "backend/crates/authz/**"
  - "backend/policies/**"
  - "deploy/compose/kratos/**"
  - "deploy/compose/hydra/**"
---

# Authentication and Authorisation Rules (Ory Kratos sessions, BFF, Cedar)

Ported from the digital-banking-system repository (`.claude/rules/auth.md`, read 2026-10-11) and adapted to AKR-TMS.
Differences from the DBS rule are marked "AKR". The lessons marked "DBS" were paid for there with real defects.

`docs/plan/phase-01.md` (identity) and `docs/plan/phase-03.md` (authorisation) are the specifications; `docs/API.md`
section 1 states the contract. This file is what must stay true while they are built and afterwards.

**AKR-TMS stores no password and no second factor.** Ory Kratos owns credentials; the API resolves sessions. There
is no self-issued login token and no signing secret for one.

## Until phase 01 (AKR)

Phase 00 has an auth SLOT, not auth. Outside production, and only with `AKR_DEV_PRINCIPAL=true`, the principal is
the UUID in the `X-Dev-Principal` header. A production binary refuses to start with that switch on. Phase 01 deletes
the stub; nothing may come to depend on it.

## Three credentials, one per request

| Credential | Carried as | Interactive |
|-----------|------------|-------------|
| Personal access token | `Authorization: Bearer akrpat_<id>_<secret>` | never |
| Hydra client-credentials JWT | `Authorization: Bearer <jwt>` | never |
| Kratos session | the `akr_session` cookie | yes |

- A bearer header that is PRESENT BUT UNUSABLE is an error, never a silent fall-through to the cookie. Otherwise an
  attacker holding a victim's cookie could attach a junk bearer and act as the victim while looking like a machine.
- AKR: a bearer together with a session cookie is 400 `E_BAD_REQUEST`.
- AKR: the payment gateway reaches `akr-assess` with mutual TLS and holds the single scope `assess.call`. It has no
  way into `/api/v1`.
- A token is stored as a hash and shown once. The `akrpat_` prefix exists so that secret scanning and the log
  redaction layer can find a leaked one.

## The order of the auth layer (AKR, phase 01 section 3)

1. **Origin and content type** on unsafe methods: a request that carries a cookie must carry `Origin` equal to
   `AKR_PUBLIC_ORIGIN` (403 `E_CSRF_INVALID`); a body is `application/json`, never a form post.
2. **Credential**: at most one, resolved as in the table.
3. **Anonymous** requests reach only the public routes (`POST /api/v1/auth/login`, `POST /api/v1/auth/mfa`).
   Everything else is 401 `E_UNAUTHENTICATED`. A test walks every operation of the OpenAPI document without
   credentials and expects 401 everywhere else.
4. **CSRF** on cookie-authenticated unsafe requests.
5. **Confinement**.
6. **Session-only routes** reached with a token: 403 `E_SCOPE`.
7. **Rate limits**.

Adding a public route, a confinement exception or an idempotency exemption is a deliberate, tested act with its
reason written in the phase file. Each list is exact-match on (method, path), never a prefix.

## The BFF shape (DBS)

The browser NEVER addresses Kratos. The API proxies Kratos's **browser** flows, which is why cookies travel in both
directions through it. Browser flows (not native API flows) are required: native flows cannot do WebAuthn.

Two rules Kratos enforces with a bare 403:

1. **A flow submit must echo the flow's anti-CSRF cookie.** Init sets it; the submit sends it back, merged with the
   caller's cookies. A correct `csrf_token` in the body is still a 403 without the cookie.
2. **A flow's secrets belong to that flow.** A settings flow's TOTP secret lives in that flow's context, so the
   confirmation goes back to the SAME flow: the enrol response returns the flow id and the confirm echoes it.

## Session CSRF

Double-submit, **HMAC-bound to the session id**. Plain double-submit is weak: an attacker who can set a cookie on the
victim's browser can also choose the matching header. Binding the token to the session under a server-side key
makes it valid only inside the session it was minted for.

- Delivered as the `akr_csrf` cookie (`Secure`, `SameSite=Strict`, readable by the page) and required as
  `X-CSRF-Token` on every cookie-authenticated mutation; the two must match AND the HMAC must verify, compared in
  constant time.
- Bearer requests are exempt: a browser never attaches `Authorization` cross-site.
- **Mint it wherever a usable session is handed out**, not only after the second factor. Every route a confined
  session may reach is a mutation, so a session without its CSRF token cannot do the one thing it exists to do.

## MFA and confinement

- AKR: MFA is mandatory for every user. There is no role and no setting that turns it off.
- A session that still owes something (a second factor to enrol, a forced password change, a policy to acknowledge)
  is **confined**: it reaches only the setup routes, and anything else is 403 `E_CREDENTIAL_CHANGE_REQUIRED`.
- DBS: never treat the presence of a credential KEY as enrolment, and never read credentials from a whoami or login
  response (the field is admin-only and structurally empty there).

## Step-up

An action marked **SU** needs a fresh second factor, proven by a single-use assertion:

- `POST /auth/step-up/verify` verifies the factor and returns an assertion bound to ONE action class, valid for
  `stepup.ttl_s`.
- The action presents it as `X-StepUp-Assertion`.
- **The header is the carrier; the PostgreSQL row is the control.** The assertion is consumed by a conditional
  `UPDATE` inside the protected action's own transaction, so a replay loses the race. Verifying only the carrier
  would leave it reusable until it expires.
- Every failure (absent, invalid, expired, replayed, minted for another session or another action class) is ONE
  answer: 401 `E_STEPUP_REQUIRED`. Telling them apart would help an attacker probe the ledger.
- AKR: opening a suspicion-tier record, every decision, every approval and every export needs one.

## Authorisation (Cedar, from phase 03)

**Two gates, in order, on every route that is not public:**

1. **The route's coarse SCOPE**, checked first. The scope belongs to the CREDENTIAL.
2. **Cedar**, on (principal, action, resource, context), deny by default. The roles belong to the USER.

Both are load-bearing. With Cedar alone, a token minted with one narrow scope would act with its owner's full
authority. Roles on the request path are therefore bounded by the credential's scopes.

- A handler's service authorises before it reads, and names both the action and the resource.
- **Every decision is audited**, allow and deny, with the policy version.
- A route that answers 2xx without a decision is a defect: a test walks the served OpenAPI document against the
  route-to-action table in both directions.
- **Segregation of duties is a `forbid` rule, not a UI rule** (AKR): a maker never checks, a proposer never
  approves, an analyst never reviews their own case. `E_SELF_APPROVAL` is the answer, and no role outranks it.
- Policies live in `backend/policies/` and change through a change request, never by editing a row. A bundle is
  validated STRICTLY against its schema at load: a misspelt action fails the load. A broken bundle never displaces a
  working one.
- **Affordances** are computed by the same engine and returned with the resource. The browser shows what the server
  says is allowed and never works permissions out for itself.

## Existence masking (AKR: the tipping-off control)

A suspicion-tier resource the caller may not see answers **404 `E_NOT_FOUND`**, byte-identical to a resource that
does not exist, in the same timing class. A 403 admits the id is real, and for a suspicion that admission IS the
disclosure.

Both halves are needed:

- **SQL.** The access filter is in the query, so a masked row is never selected. It cannot leak through a count, a
  cursor, a sort order or a timing difference.
- **The service.** It answers `NotFound`, not `Forbidden`, when the access list is the reason.

And everything derived follows: lists, searches, counts, exports, SSE events, notifications, error texts and job
payloads never reveal a masked resource. Every phase file's tipping-off review says what a person outside the case's
access list can learn from each new artefact; the answer must be "nothing".

An insufficient ROLE on a resource that is not suspicion-tier may still be told 403.

## Security schemes in the document

| Name | Kind | Carries |
|------|------|---------|
| `session_cookie` | apiKey, cookie | the session |
| `bearer_token` | http bearer | a personal access token or a client-credentials JWT |
| `stepup_assertion` | apiKey, header `X-StepUp-Assertion` | a single-use step-up assertion |

## Non-negotiables

- NEVER log a session cookie, a token, a step-up assertion, a password or a TOTP secret. The request layer marks the
  headers sensitive and the telemetry layer redacts the field names and the `akrpat_` pattern. A cache key is a hash
  of the cookie, never the cookie.
- NEVER reintroduce a self-issued login token. Sessions are Kratos's.
- NEVER answer differently for "wrong password", "no such account" and "account disabled": all three are one 401.
  (DBS: mapping a Kratos rejection to 503 was a status oracle that announced disabled accounts.)
- NEVER trust a client address from `X-Forwarded-For` unless the peer is in `AKR_TRUSTED_PROXIES` (the edge).
- NEVER put an authorisation decision in the frontend only. The frontend hides; the server refuses.
- A mocked identity provider proves the code is self-consistent, NOT that Kratos accepts it. Anything that touches
  flows, cookies or credential shapes is proven only against the real service (DBS: five of six identity defects
  passed the entire mocked suite).
