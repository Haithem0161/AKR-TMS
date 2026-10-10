p='docs/MODULES.md'
s=open(p).read()
a="GV-42..53, T-48..51, M-70..73, TS-17 (the assistant, phase 28, and other phases).\n"
assert s.count(a)==1
b=a+("Phase file: docs/plan/phase-28.md (DR-274 to DR-291): an optional pack (flag `pack.assistant`, default off, 404 on\n"
"every route) with two tasks only, case summary and report narrative, both drafts; the llama.cpp server with one GGUF\n"
"model on premises in zone `z2_model` without egress, called by the worker only, and a second server `llama-eval` for\n"
"verification and evaluation; generation is a job, the browser polls; figures, dates and names are placeholders filled by\n"
"code and the `GroundingVerifier` removes any sentence it cannot match; phase 09 tipping-off scan; nothing is saved\n"
"without one person accepting one draft, recorded in `casework.decisions.ai_involvement`; golden evaluation on\n"
"QA-confirmed closed cases (200, 50 per language) with a human factual review; go-live of the model and template set\n"
"by change request `cr_material_validated`; monitoring counts suppressed; QA sample of assisted decisions never below\n"
"10 per cent; no agent, tools or function calling. The kill switch is engaged by one MRO and released by two people\n"
"through `cr_material`; research 28 asks for two people on both, so the bank decides (Open doubts 8). Not carried:\n"
"disposition recommendation (needs GV-42), the other candidate tasks of research 21, the GV-51 security tests and exit\n"
"plan (procurement records), model selection and GPU hardware.\n")
s=s.replace(a,b)
open(p,'w').write(s)
print('ok')
