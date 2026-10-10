#!/usr/bin/env python3
"""Apply gap pass 1, run 1 (targets 00, 01, 02) after the lead's checks.

Source: tools/agent/gap1-run1.md (Haiku draft); decisions: tools/agent/gap1-run1-verdict.md.
Apply-and-assert: every edit is checked; files are written only at the end.
"""
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
PLAN = ROOT / 'docs' / 'plan'
draft = (ROOT / 'tools' / 'agent' / 'gap1-run1.md').read_text()

# --- extract the fenced 7.x blocks by finding id --------------------------------------------
blocks = {}
for m in re.finditer(r'^### (G1-0[012]-\d\d) .*?(?=^### G1-|^## )', draft, re.S | re.M):
    fid, body = m.group(1), m.group(0)
    f = re.search(r'```markdown\n(.*?)```', body, re.S)
    if f:
        blocks[fid] = f.group(1).rstrip('\n')
assert len(blocks) == 30, len(blocks)  # 32 findings, two (01-08, 02-10) had no text


def rep(text, a, b, count=1):
    assert text.count(a) == count, (a[:60], text.count(a))
    return text.replace(a, b)


B = blocks

# --- phase 00 -------------------------------------------------------------------------------
B['G1-00-03'] = """### 7.3 Timer kinds registered by later phases
The timer kind registry is a list of kind and owner kind pairs. The sweep enqueues the job `timer_<kind>`; a kind that
is not in the registry is never claimed (the job rule of this section). Kinds registered by later phases (each in its
own section 4.2):

| Timer kind | Owner kind | Registered by |
|-|-|-|
| `cr_activate`, `cr_emergency_review`, `cr_post_review` | `change_request` | phase 04 |
| `task_timeout` | `task` | phase 04 |
| `screening_alert_clock` | `screening_alert` | phase 14 |
| `kyc_review_clock` | `kyc_review` | phase 15 |
| `alert_touch`, `alert_age` | `alert` | phase 17 |
| `work_item_due` | `work_item` | phase 18 |
| `rfi_clock` | `rfi` | phase 19 |
| `breach_stage` | `alert`, `work_item` | phase 19 |
| `watch_expiry` | `watch` | phase 19 |
| `hibernation_expiry` | `hibernation` | phase 19 |
| `qa_grant_expiry` | `qa_sample` | phase 19 |
| `intake_clock` | `internal_report`, `authority_request` | phase 19 |
| `freeze_due`, `freeze_limit`, `committee_report_due`, `second_report_due` | `freeze` | phase 20 |
| `refrain_due` | `refrain` | phase 20 |
| `report_filing_target` | `report` | phase 21 |
| `report_rework_due` | `report_task` | phase 21 |
| `liaison_due` | `liaison_monthly_report` | phase 22 |"""

B['G1-00-04'] = """### 7.4 Monitoring and retention roles
Migration 0001 is unchanged. The role list of section 1 is completed with the two roles that later phases create
(DR-22: login facts are provisioning):
- `akr_monitor`: a LOGIN role created by `akr-ctl db provision` (password in OpenBao `db/akr_monitor`), member of
  `pg_monitor` only, with no privilege on any AKR schema (phase 03 section 4.4; phase 01 7.2).
- `akr_retention`: NOLOGIN, created by phase 23 migration 0072 with its table grants; its LOGIN role
  `akr_retention_login` (member of `akr_retention` and of `akr_audit_writer`, password `db/akr_retention_login`) is
  created by `akr-ctl db provision` (phase 23 sections 1.2 and 4.4; phase 01 7.2)."""

B['G1-00-05'] = """### 7.5 KV mount rule, AppRole policy additions and KV paths
- **One KV v2 mount, relative paths (DR-356).** OpenBao has one KV v2 mount, `akr/` (section 3). Every KV path in every
  phase is relative to it and is written without a mount prefix: `db/<role>`, `s3/<name>`, `keys/<name>`,
  `sources/<id>`, `ory/<x>`, `polis/<x>`, `release/trust`, `kratos/oidc/<provider>`, `assistant/<x>`. The spellings
  `kv/...` (phases 25 and 31) and `secret/...` (phase 28) mean the same paths under `akr/`; those phases' own 7.x
  record the correction.
- AppRole policies (section 3), in addition to the current lines:
  - `akr-api`: KV read `db/akr_readonly` (read-only pool, phase 24), `s3/stager` (stager credential, phase 02) and
    `polis/directory` (webhook secret, phase 31).
  - `akr-worker`: KV read `db/akr_readonly` (verifier pool, phase 02), `db/akr_retention_login` (phase 23),
    `assistant/llama` and `assistant/golden-hmac` (phase 28) and `release/trust` (phase 25); read on
    `sys/seal-status` and `sys/health` (phase 03); sign on the signing keys of 7.6.
  - `akr-ingestd`: unchanged (`db/akr_ingest`, `sources/*`).
  - `akr-assess`: transit encrypt and decrypt on `akr-kek`; KV read `db/akr_assess`, `keys/assess-decision/*`,
    `keys/assess-extract/*` (phase 29 section 4.4).
  - New AppRoles: `akr-monitor` (KV read `db/akr_monitor`, for the postgres exporter, phase 03); `akr-migrate` (KV
    read `db/akr_migrate` only, issued per release, phase 30); `akr-polis` (KV read `ory/polis` and `polis/keys`
    only, phase 31).
- KV items added: `s3/stager` (phase 02), `keys/model-artifact/v<N>.pub` (phase 27, verify-only), `db/akr_monitor`,
  `db/akr_retention_login`, `db/akr_migrate`, `db/akr_readonly`, `polis/directory`, `polis/keys`, `ory/polis`,
  `release/trust`, `kratos/oidc/<provider>`, `assistant/llama`, `assistant/golden-hmac`."""

B['G1-00-06'] = """### 7.6 Signing keys: inventory (DR-357)
The transit key list of section 3 (OpenBao layout) becomes:
- `akr-kek` (aes256-gcm96, wraps DEKs);
- ed25519 signing keys used by `akr-worker`: `akr-sign-anchor` (audit anchors, phase 02), `akr-sign-package`
  (as section 3 names it), `akr-sign-certificate` (coverage certificates, phases 05 and 12) and `akr-sign-deletion`
  (deletion certificates and batch manifests, phase 23; a separate key with its own rotation and custody, DR-194;
  phase 23 wrote it `akr-sign-cert`, corrected in its 7.1);
- `akr-sign-bundle` (ed25519; support bundles, phase 25): signed by `akr-ctl support-bundle create` on the bank host
  under an operator policy issued for that act, never by a service AppRole;
- `akr-sign-decision`: reserved and unused. Decision signing uses a seed wrapped under `akr-kek`, unwrapped once per
  node at start-up (DR-294, phase 29 section 3.4).
In the `akr-worker` AppRole line, "sign on the four signing keys" becomes "sign on `akr-sign-anchor`,
`akr-sign-package`, `akr-sign-certificate` and `akr-sign-deletion`"."""

B['G1-00-13'] = """### 7.13 Schema list: train
Migration 0001 is unchanged. Phase 31 creates the schema `train` in its own migration (`CREATE SCHEMA IF NOT EXISTS
train AUTHORIZATION akr_migrate`, with its grants; DR-338). The schema list of section 1 names it as a later addition,
as it does for the schemas of later phases."""

B['G1-00-15'] = """### 7.15 Keyed-hash purposes
Every call of `keyed_hash(purpose, data)` names a purpose, declared as a constant of `common::crypto::Purpose` by the
phase that uses it; a test asserts that the names are unique. Purposes declared so far: `csrf`, `token` and
`login_identifier` (phase 01), and `islamic.document_ref` (phase 26; phase 26 wrote "phase 02 purpose", corrected in
its 7.1). The phase 10 mapping transform `KeyedHash(purpose)` computes `_hk` values with the match key
`keys/match-key` and declares its purposes the same way."""

# --- phase 01 -------------------------------------------------------------------------------
B['G1-01-01'] = rep(B['G1-01-01'],
    """- Step 1 (dormancy) reads `admin.users.enabled_at` (added by phase 03 section 1). The exact expression is UNCLEAR in
  phase 03 (it states only that the rule reads `enabled_at`); the lead must write it before implementation.""",
    """- Step 1 (dormancy) measures from `greatest(last_authenticated_at, enabled_at, created_at)` instead of
  `coalesce(last_authenticated_at, created_at)`, so a re-enabled account gets a full `auth.dormancy_days` to sign in
  (`enabled_at` is added by phase 03 section 1; the expression is phase 03 section 3.7).""")

B['G1-01-02'] = rep(B['G1-01-02'],
    """(a) grants LOGIN to `akr_monitor` with the password in OpenBao `db/akr_monitor`; the role is a member of `pg_monitor`
    only and has no privilege on any AKR schema (phase 03 section 4.4; role created in phase 00 section 1, see 7.4 there);
(b) grants LOGIN to `akr_retention_login` with the password in OpenBao `db/akr_retention_login`; it is a member of
    `akr_retention` and of `akr_audit_writer` (phase 23 sections 1.2 and 4.4);""",
    """(a) creates the LOGIN role `akr_monitor` with the password in OpenBao `db/akr_monitor`; it is a member of
    `pg_monitor` only and has no privilege on any AKR schema (phase 03 section 4.4; phase 00 7.4);
(b) creates the LOGIN role `akr_retention_login` with the password in OpenBao `db/akr_retention_login`; it is a member
    of `akr_retention` (NOLOGIN, phase 23 migration 0072) and of `akr_audit_writer` (phase 23 sections 1.2 and 4.4);""")

# --- phase 02 -------------------------------------------------------------------------------
b = B['G1-02-01']
b = rep(b, """Each row is registered by the phase named; the retention class is the one that phase states, or the lead assigns it where
the owning phase does not state one.""",
    """Each row is registered by the phase named. The retention column shows the class where the owning phase states one;
a dash means the owning phase states none (the verification pass checks each).""")
b = rep(b, '| Owner kind | Owning phase | Retention |', '| Owner kind | Owning phase | Retention (as stated) |')
b = b.replace('| not stated |', '| - |')
b = rep(b, '| the owner of the case uploads (phase 18 section 3.4) |', '| - |')
b = rep(b, "\n| `coverage_certificate` | 12 | R6 |", '')
b = rep(b, "| `fx_rate` |", "| `coverage_certificate` | 12 | R6 |\n| `fx_rate` |")
# order rows by phase: 15 before 16
b = rep(b, "| `replay_sample`, `cbi_import` | 16 | - |\n| `kyc_review`, `pep_register`, `ownership_input`, `override`, `rating_feed_export` | 15 | - |",
    "| `kyc_review`, `pep_register`, `ownership_input`, `override`, `rating_feed_export` | 15 | - |\n| `replay_sample`, `cbi_import` | 16 | - |")
assert 'not stated' not in b
B['G1-02-01'] = b

B['G1-02-04'] = rep(B['G1-02-04'], '(SQLSTATE 23514; phase 13 line 910, as phase 10 does)', '(SQLSTATE 23514; phases 10 and 13)')

B['G1-02-05'] = rep(B['G1-02-05'], 'detail keys (the sections listed in the Sources of this finding).',
    'detail keys (the audit section of each phase 03 to 31; that section is normative for its kinds).')
B['G1-02-05'] = rep(B['G1-02-05'], 'The kinds themselves are copied from those sections by the lead; they are not repeated here.',
    'The kinds are not copied here: `CATALOGUE` is the union of the phases\' audit tables, and a test fails on a kind\nemitted by code that is in no phase\'s table.')

B['G1-02-08'] = rep(B['G1-02-08'], """The ALTER statements of phases 06 and 24 stay in those phases. Phase 24 section 4.5 item 1 says "five kinds" are new; only
`certification` is new in the CHECK (lead to correct the wording in phase 24).""",
    """The ALTER statements of phases 06 and 24 stay in those phases.""")

B['G1-02-09'] = """### 7.9 access_event.action holds the access verb
The `action` column of `audit.access_event` holds the access verb (`view`, `search_hit`, `export`, `print`,
`download`, `unmask`), never a Cedar action name; the CHECK is unchanged (DR-358). The profile read of phase 13
(`GET /parties/{id}/profiles`, Cedar action `profile.read`) writes `action = 'view'`, `resource_type = 'customer'`,
`tier = 'restricted'`, with the purpose code and the party; phase 13 wrote the Cedar name as the access value,
corrected in its 7.1."""

ORDER = {
    '00': ['G1-00-%02d' % i for i in range(1, 16)],
    '01': ['G1-01-%02d' % i for i in range(1, 8)],
    '02': ['G1-02-%02d' % i for i in range(1, 10)],
}

HDR = ('Gap pass 1 (2026-10-10), run 1, from tools/agent/gap1-run1.md; lead-verified '
       '(tools/agent/gap1-run1-verdict.md).')
CHDR = ('Gap pass 1 (2026-10-10), run 1: corrections found while checking phases 00 to 02 '
        '(tools/agent/gap1-run1-verdict.md); lead-verified.')

# --- source-phase corrections ---------------------------------------------------------------
CORR = {
    '06': """### 7.1 Step-up classes of the declaration and the pack download (DR-359)
Phase 01's closed list of action classes is not extended. This phase declares two step-up actions with existing
classes: `StepUpAction { name: "readiness.declare", class: approval }` (section 3.4 step 1) and
`StepUpAction { name: "pack.download", class: export }` (section 3.6; DR-230 of phase 24). Every mention of a class
`readiness_declare` or `pack_download` (sections 3.4, 3.6 and 4) reads as these action names with those classes, and
section 4.5's "Phase 01: two step-up action classes" is void: phase 01 needs no change.""",
    '08': """### 7.1 Step-up names in the coverage screens
The declaration drawer's step-up is the action `readiness.declare` (class `approval`) and each pack download's
step-up is the action `pack.download` (class `export`), as phase 06 7.1 records; the classes `readiness_declare` and
`pack_download` named in section 3 do not exist.""",
    '13': """### 7.1 The profile read's access event
The profile read writes the access event `view` with `resource_type = 'customer'`, tier `restricted`, the purpose
code and the party; `profile.read` is the Cedar action, not the access value (phase 02 7.9, DR-358). Section 3.10's
"access event (`profile.read`, ...)" reads accordingly, and section 4.5's "the access layer gains the `profile.read`
action" is void: phase 02's access layer needs no change.""",
    '23': """### 7.1 Deletion-certificate key name
The transit key `akr-sign-cert` of this phase is named `akr-sign-deletion` (phase 00 7.6, DR-357), so that it is not
confused with the coverage-certificate key `akr-sign-certificate`. Every mention (sections 2, 3 and 4.4, DR-194) and
the `akr-worker` policy paths (`transit/sign/akr-sign-deletion`, `transit/verify/akr-sign-deletion`, read on
`transit/keys/akr-sign-deletion`) use the new name. Nothing else changes: it stays a separate key from the anchor key.""",
    '25': """### 7.1 KV paths and the support-bundle key
- KV paths are relative to the one mount `akr/` (phase 00 7.5, DR-356): `kv/release/trust` is `release/trust` and
  `kv/kratos/oidc/<provider>` is `kratos/oidc/<provider>`.
- The transit key `akr-sign-bundle` is entered in the key inventory (phase 00 7.6, DR-357): signed by `akr-ctl` on the
  bank host under an operator policy. The key part of Open doubts 9 is closed; the disclosure-register part stays open.""",
    '26': """### 7.1 Home of the keyed-hash purpose
The purpose `islamic.document_ref` is declared in the phase 00 crypto port (`Crypto::keyed_hash`, phase 00 7.15), not
in a phase 02 list. Section 3's "(phase 02 `KeyedHash`)" reads "(phase 00 `Crypto::keyed_hash`)". Open doubts 15 is
closed.""",
    '28': """### 7.1 KV paths
KV paths are relative to the one mount `akr/` (phase 00 7.5, DR-356): `secret/assistant/llama` is `assistant/llama`
and `secret/assistant/golden-hmac` is `assistant/golden-hmac`.""",
    '31': """### 7.1 KV paths
KV paths are relative to the one mount `akr/` (phase 00 7.5, DR-356): `kv/polis/directory` is `polis/directory`,
`kv/polis/keys` is `polis/keys` and `kv/kratos/oidc/generic` is `kratos/oidc/generic`; `ory/polis` is unchanged.""",
}

NONE_RE = re.compile(r'(## 7\+\. Gap Additions\n\n)(_None yet\._|None yet\.)\n')

out = {}
for pp, ids in ORDER.items():
    p = PLAN / f'phase-{pp}.md'
    s = p.read_text()
    assert len(NONE_RE.findall(s)) == 1, pp
    parts = []
    for i, fid in enumerate(ids, 1):
        blk = B[fid]
        blk = re.sub(r'^### 7\.\d+ ', f'### 7.{i} ', blk, count=1)
        assert blk.startswith(f'### 7.{i} '), fid
        parts.append(blk)
    body = HDR + '\n\n' + '\n\n'.join(parts) + '\n'
    out[p] = NONE_RE.sub(lambda m: m.group(1) + body, s)

for pp, blk in CORR.items():
    p = PLAN / f'phase-{pp}.md'
    s = p.read_text()
    assert len(NONE_RE.findall(s)) == 1, pp
    out[p] = NONE_RE.sub(lambda m: m.group(1) + CHDR + '\n\n' + blk + '\n', s)

# leftovers that must not survive in the applied text
for p, s in out.items():
    tail = s[s.index('## 7+. Gap Additions'):]
    for w in ('UNCLEAR', 'Not proposed here', 'the lead must', 'lead to correct'):
        assert w not in tail, (p.name, w)

# --- research.md DRs ------------------------------------------------------------------------
r = PLAN / 'research.md'
rs = r.read_text()
anchor = [l for l in rs.split('\n') if l.startswith('| DR-355 |')]
assert len(anchor) == 1
DRS = [
    "| DR-356 | 2026-10-10 | One OpenBao KV v2 mount `akr/`; every KV path in every phase is relative to it, written with no mount prefix (`kv/...` and `secret/...` spellings corrected) | Phase 00 section 3 already mounts `akr/`; one rule keeps AppRole policies checkable (gap pass 1, phase 00 7.5) |",
    "| DR-357 | 2026-10-10 | Signing key inventory: `akr-sign-anchor`, `akr-sign-package`, `akr-sign-certificate` (coverage), `akr-sign-deletion` (phase 23, renamed from `akr-sign-cert`), `akr-sign-bundle` (operator, `akr-ctl`), `akr-sign-decision` reserved | DR-194 keeps the deletion key separate; a distinct name avoids two near-identical names (gap pass 1, phase 00 7.6) |",
    "| DR-358 | 2026-10-10 | `audit.access_event.action` holds the access verb, never a Cedar action name; the profile read writes `view` on `customer` | The CHECK lists verbs; Cedar names belong to the authorization layer (gap pass 1, phase 02 7.9) |",
    "| DR-359 | 2026-10-10 | No new step-up class for phase 06: `readiness.declare` uses `approval`, `pack.download` uses `export` | Phase 01's list is closed in code; DR-230 already chose `export` for pack downloads (gap pass 1, phase 06 7.1) |",
]
rs = rep(rs, anchor[0] + '\n', anchor[0] + '\n' + '\n'.join(DRS) + '\n')
out[r] = rs

for p, s in out.items():
    p.write_text(s)
    print('wrote', p.relative_to(ROOT))
