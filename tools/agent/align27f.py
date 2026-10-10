p='docs/MODULES.md'
s=open(p).read()
def rep(a,b):
    global s
    assert s.count(a)==1,(s.count(a),a[:80])
    s=s.replace(a,b)
rep("G5. Not carried: models, scores and the assistant (P2, phases 27 and 28).\n",
"G5. Not carried: models, scores and the assistant (P2, phases 27 and 28).\n"
"Phase file: docs/plan/phase-27.md (DR-257 to DR-273): a model family and each version are inventory items of kind\n"
"`model`, and only an `approved_for_use` version scores (shadow included); import in two steps (manifest hash and\n"
"Ed25519 signature in the API, artifact checks in the worker; a bank-trained artifact enters the store only by\n"
"`akr-ctl model artifact add`); `ort` with a `tract` fallback in the worker only; the score is a separate row in\n"
"`alert.model_scores` that ranks inside the priority class and never sets the class, hides, closes or creates an alert;\n"
"label gates 50, 200 and 500 from QA-confirmed outcomes over every tier; frozen shadow criteria; validation through the\n"
"phase 24 pack; PSI and override monitoring with red points as phase 24 findings (source `model_monitor`, five closure\n"
"codes); kill switch immediate by an MRO, release by change request `cr_material`; pins, one live family; TreeSHAP\n"
"reason codes (three shown). The kill switch is pressed by an MRO only (scope `model.risk`), not by a family owner who\n"
"is not an MRO (Open doubts 1). Not carried: GV-02, GV-06..08, GV-10..13, GV-17, GV-23, GV-26..28, GV-31, GV-33,\n"
"GV-42..53, T-48..51, M-70..73, TS-17 (the assistant, phase 28, and other phases).\n")
open(p,'w').write(s)
print('ok')
