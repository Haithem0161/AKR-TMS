p='docs/API.md'
s=open(p).read()
def rep(a,b):
    global s
    assert s.count(a)==1,(s.count(a),a[:80])
    s=s.replace(a,b)
G="model.risk, audit.read, dashboard.read"
rep("| GET | /models | model.risk | Model registry [P2] |\n"
"| POST | /models | model.risk | Register a signed ONNX artifact with its card [P2] |\n"
"| GET | /models/{id} | model.risk | Card, validation, drift, label gates [P2] |\n"
"| POST | /models/{id}/validation | model.risk | Record validation results [P2] |\n"
"| POST | /models/{id}/shadow | model.risk | Start shadow or challenger [P2] |\n"
"| POST | /models/kill-switch | model.risk | Fall back to rules only; **SU** [P2] |\n",
f"| GET | /models | {G} | Families with live version, starter flag, next drill and pending version notices, cursor (phase 27) |\n"
"| POST | /models | model.risk | Create a family and its inventory item (kind `model`); 201; **IDEM** (phase 27; meaning narrowed, DR-271) |\n"
f"| GET | /models/kill-switch | {G} | State of the global switch and of each family's switch (phase 27, added) |\n"
f"| GET | /models/ladder | {G} | Model ladder per family: enabled tier, label counts and next gate, suppressed counts only (phase 27, added) |\n"
f"| GET | /models/{{id}} | {G} | Family with live, shadow, challenger and pinned versions, gates, next drill (phase 27) |\n"
"| POST | /models/{id}/versions | model.risk | Import a version (manifest, Ed25519 signature, card); 202 `importing`, worker job `model_import_verify`; bad signature or manifest 422 `E_VALIDATION`; **IDEM** (phase 27, added) |\n"
f"| GET | /models/{{id}}/versions/{{vid}} | {G} | Version, card (ten sections), refusal code; `?view=operating_point` adds thresholds and top-band cut, MRO and M only (phase 27, added) |\n"
"| POST | /models/{id}/validation | model.risk | Record a benchmark, out-of-time, calibration or sensitivity evaluation of a version; the pack is `POST /inventory/{id}/validation-packs`; **IDEM** (phase 27; meaning changed, DR-271) |\n"
"| POST | /models/{id}/shadow | model.risk | Start a shadow or challenger run with frozen success criteria (version `registered` or `retired`, item `approved_for_use`, shadow gate met); **IDEM** (phase 27) |\n"
"| POST | /models/{id}/versions/{vid}/promote | model.risk | Open change request on route `cr_material_validated` for the version; `If-Match`; **IDEM** (phase 27, added) |\n"
"| POST | /models/{id}/pin | model.risk | Pin or unpin the live version; **SU** `approval`; `If-Match`; **IDEM** (phase 27, added) |\n"
"| POST | /models/kill-switch | model.risk | Engage the global switch or one family's switch (scope); immediate, rules-only order; **SU** `approval`; `If-Match` on the scope row; **IDEM** (phase 27) |\n"
"| POST | /models/kill-switch/release-requests | model.risk | Open change request on route `cr_material` that releases a switch; **IDEM** (phase 27, added) |\n"
"| POST | /models/{id}/kill-drills | model.risk | Record a quarterly kill-switch drill and move `next_kill_drill_due`; **IDEM** (phase 27, added) |\n"
f"| GET | /models/{{id}}/monitoring | {G} | Stored monitoring points of the family's versions: bands, values, suppressed window counts, no ids (phase 27, added) |\n")
rep("| POST | /assistant/drafts | queue.work | Ask the on-premises assistant",
"\nPhase 27 owns the `/models` rows (docs/plan/phase-27.md s.2; 16 rows with `GET /alerts/{id}/model-score` in 3.12, 10\n"
"added; DR-257 to DR-273). Cedar permits the GET rows to MRO, M, IA and CH; every POST is MRO. The `/assistant` rows below are phase 28's.\n\n"
"| Method | Path | Scope | Description |\n|---|---|---|---|\n"
"| POST | /assistant/drafts | queue.work | Ask the on-premises assistant")
rep("| GET | /alerts/{id}/snapshot | queue.work | The sealed evidence snapshot (access event) (phase 17, added) |\n",
"| GET | /alerts/{id}/snapshot | queue.work | The sealed evidence snapshot (access event) (phase 17, added) |\n"
"| GET | /alerts/{id}/model-score | queue.work | Live score, top-band flag and up to three reason codes of one alert; never shadow or challenger; the alert's own visibility, purpose and step-up rules (phase 27, added) |\n")
open(p,'w').write(s)
print('ok')
