p='docs/API.md'
s=open(p).read()
old_note="added; DR-257 to DR-273). Cedar permits the GET rows to MRO, M, IA and CH; every POST is MRO. The `/assistant` rows below are phase 28's.\n"
assert s.count(old_note)==1
old_rows="""| POST | /assistant/drafts | queue.work | Ask the on-premises assistant for a draft within the permitted uses [P2] |
| POST | /assistant/drafts/{id}/accept | queue.work | A person accepts or edits the draft; logged as AI involvement [P2] |
| GET | /assistant/evaluations | model.risk | Evaluation gate results (at least 200 QA-confirmed cases, Arabic and English) [P2] |
"""
assert s.count(old_rows)==1
new_rows="""| POST | /assistant/drafts | queue.work | Request a draft of one task (`case_summary`, `report_narrative`) for one case; 202 `requested`, job `assistant_generate`; no text in the body; `X-Access-Purpose`; **IDEM** (phase 28) |
| GET | /assistant/drafts/{id} | queue.work | The draft: state, sentences with source labels, removed and marked counts, `not_considered` line, scan count; text only to the case team after the cited panels are opened; `X-Access-Purpose` (phase 28, added) |
| POST | /assistant/drafts/{id}/accept | queue.work | Accept one draft as is or edited: writes a case note or the phase 21 narrative; recorded in `ai_involvement`; no accept-all; `If-Match`; **IDEM** (phase 28) |
| POST | /assistant/drafts/{id}/discard | queue.work | Discard a draft with the useful-or-not answer; `If-Match`; **IDEM** (phase 28, added) |
| POST | /assistant/evaluations | model.risk | Run the golden evaluation of one (task, template, model) triple; 202, job `assistant_evaluate`; **IDEM** (phase 28, added) |
| GET | /assistant/evaluations | model.risk | Gate results (at least 200 QA-confirmed cases, 50 per language): counts, rates, verdicts, golden-set hash; no ids, no text; cursor (phase 28) |
| GET | /assistant/tasks | model.risk, audit.read, dashboard.read | Task registry: tier, autonomy, switch and kill state, live template and model, gate status, next drill; no text (phase 28, added) |
| POST | /assistant/versions | model.risk | Import a model (202, job `assistant_version_verify`) or a prompt template (201); bad signature or manifest 422 `E_VALIDATION`; **IDEM** (phase 28, added) |
| POST | /assistant/versions/{id}/promote | model.risk | Open change request on route `cr_material_validated` for one model and one template per enabled task; nothing changed 409 `nothing_to_promote`; `If-Match`; **IDEM** (phase 28, added) |
| POST | /assistant/kill-switch | model.risk | Engage the global switch or one task's switch; immediate; **SU** `approval`; `If-Match` on the scope row; **IDEM** (phase 28, added) |
| POST | /assistant/kill-switch/release-requests | model.risk | Open change request on route `cr_material` that releases a switch; **IDEM** (phase 28, added) |
| GET | /assistant/monitoring | model.risk, audit.read, dashboard.read | Counts per task over 30 days on live drafts, every tier, suppressed below `report.small_number_threshold`, no split (phase 28, added) |

Phase 28 owns the `/assistant` rows (docs/plan/phase-28.md s.2; 12 rows, 9 added; DR-274 to DR-291). Every route is 404
`E_NOT_FOUND` while flag `pack.assistant` is off. Cedar permits the `/assistant/drafts` rows to the case team (A1, A2, M;
the narrative accept also needs phase 21 `report.edit`), the GET registry and monitoring rows to MRO, M, IA and CH, and
every other row to MRO.
"""
s=s.replace(old_rows,new_rows)
open(p,'w').write(s); print('ok')
