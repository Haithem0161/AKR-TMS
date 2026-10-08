# AKR-TMS Implementation — Status

Living tracker for the 32-phase build (phases 00-31, macro-phases P0-P2; P3 phases written later). Updated as phases
move Planned, Spec ready, In Progress, Done. Counts are plan-time targets until a phase is implemented.

Planning state on 2026-10-07: the eight system docs are written (docs/README.md); the phase table, dependency graph,
entities, services and migration chain are in roadmap.md. Phase files: 00 and 01 written 2026-10-07. The owner chose (2026-10-07) that the
lead writes P0 (00-08) inline, one by one, then asks again before 09-31.

## Phase Status

| # | Phase | Status | Started | Completed | Tables + | Tables ~ | Routes + | Services + |
|-|-|-|-|-|-|-|-|-|
| 00 | Platform Foundation | Spec ready | | | 8 | 0 | 0 (infra only) | 5 + crypto, store and S3 guard ports |
| 01 | Identity & Sessions | Spec ready | | | 7 | 3 (idempotency_keys, settings, feature_flags FKs) | 22 | 4 + RateLimiter, PasswordPolicy, ClientCredentials |
| 02 | Audit Trail & Evidence Core | Planned | | | 5 | 0 | | 4 |
| 03 | Authorisation, Users & Roles | Planned | | | 2 | | | 4 |
| 04 | Change Control & Approval Workflows | Planned | | | 7 | | | 5 |
| 05 | Sources, Read-only Connection & Discovery | Planned | | | 8 | | | 4 |
| 06 | CBI Catalogue, Readiness Grades & Gap Report | Planned | | | 4 | | | 3 |
| 07 | goAML Checker | Planned | | | 4 | | | 4 |
| 08 | Frontend Foundation | Planned | | | 0 | 0 | 0 | frontend |
| 09 | Reference Data, Calendars & Policy Pack | Planned | | | 10 | | | 4 |
| 10 | Mapping & Canonical Model | Planned | | | 8 + canon | | | 5 |
| 11 | Production Ingestion & Freshness | Planned | | | raw + 1 | | | 4 |
| 12 | Reconciliation, Coverage & Data Quality | Planned | | | 6 | | | 3 |
| 13 | Features & Segments | Planned | | | 6 | | | 3 |
| 14 | Screening Engine & Lists | Planned | | | 10 | | | 6 |
| 15 | Risk Rating & KYC | Planned | | | 9 | | | 4 |
| 16 | Scenario Engine, Packs & Studio | Planned | | | 7 | | | 5 |
| 17 | Alerts, Triage & Queues | Planned | | | 6 | | | 4 |
| 18 | Cases, Decisions & Investigation | Planned | | | 6 | | | 4 |
| 19 | RFIs, Watches, Network & QA | Planned | | | 11 | | | 4 |
| 20 | Operations Tasks & Freezes | Planned | | | 3 | | | 3 |
| 21 | Reporting to the Office | Planned | | | 9 | | | 5 |
| 22 | Branch Window & Notifications | Planned | | | 7 | | | 4 |
| 23 | Retention & Legal Holds | Planned | | | 3 | | | 2 |
| 24 | Governance, Registers, Command Centre & Packs | Planned | | | 3 | | | 4 |
| 25 | Platform Growth & Releases | Planned | | | 0 | | | 5 |
| 26 | Islamic Pack (optional) | Planned | | | 3 | | | 1 |
| 27 | Models & Model Governance | Planned | | | 3 | | | 3 |
| 28 | Assistant | Planned | | | 1 | | | 2 |
| 29 | Assessment API | Planned | | | 3 | | | 3 |
| 30 | Profile M/L & High Availability | Planned | | | 0 | 0 | 0 | infra |
| 31 | Training, Capacity & Correspondent Packs | Planned | | | tbd | | | 3 |

Status values: Planned | Spec ready | In Progress | Done | Blocked.

> "Tables ~" = tables a phase modifies (ALTER, new columns, new indexes) that it does not create. Route counts are
> filled in from each phase's route table, which is normative.

## Cumulative Totals

| Metric | Before | Current | Target |
|-|-|-|-|
| Domain tables (specified) | 0 | 15 | about 180 (DATA-MODEL.md) |
| Routes | 0 | 22 | about 366 (API.md) |
| Phases specified | 0 | 2 | 32 |
| Screens | 0 | 0 | 61 (DESIGN-SYSTEM.md 8.1) |
| Research requirement IDs carried by a phase | 0 | 48 (some partial) | 1,208 (coverage baseline) |

## Jobs by Phase

_Filled in from each phase's Section 4._

| Job kind | Phase | Schedule / trigger |
|-|-|-|
| `maintenance_sweep` | 00 | Daily, `schedule.maintenance_sweep` (02:30 bank time) |
| `backup_check` | 00 | Daily, `schedule.backup_check` (06:00 bank time) |
| `identity_sweep` | 01 | Daily, `schedule.identity_sweep` (03:00 bank time): dormancy and account expiry |

## Gap Analysis Summary

_No passes run yet._ See roadmap.md "Gap Analysis Additions" for the authoring delta of 2026-10-07.

## Blockers & Notes

- External items and owners: docs/ROADMAP.md "External dependencies". None blocks writing the phase files.
- The two early pieces (phases 06 and 07) are the P0 priority after the foundation they need; phase 07 can ship
  before discovery (phase 05) is finished.
- Dark palette (DS-O1) awaits the owner; light ships regardless.
- Agents for phase files and gap passes: ask the user first (project rule).
