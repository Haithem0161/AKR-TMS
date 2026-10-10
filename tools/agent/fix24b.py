p='docs/plan/phase-24.md'
s=open(p).read()
def block(start_marker):
    a=s.index(start_marker); a=s.index("```cedar\n",a)+len("```cedar\n"); b=s.index("```",a); return a,b
def per_role(pid, roles, head, cond_extra=None):
    out=[]
    for r in roles:
        c=f'principal.roles.contains("{r}")'
        if cond_extra: c=f'{c} && {cond_extra}'
        out.append(f'@id("{pid}.{r.lower()}") @role("{r}")\n{head}\nwhen {{ {c} }};')
    return "\n\n".join(out)
audit_ = "\n\n".join([
"// 60_audit.cedar (phase 24). Audit viewer, chain status, verify runs, access history and the control report.\n// One role per policy (conventions); the handler answers the masked 404 where a forbid fires on a suspicion record.",
per_role("audit.viewer",["IA","SEC"],'permit (principal, action in [Akr::Action::"audit.read_events", Akr::Action::"audit.chain_status",\n                              Akr::Action::"audit.verify"], resource is Akr::System)'),
per_role("audit.history",["M","SEC","IA"],'permit (principal, action == Akr::Action::"audit.access_history", resource is Akr::AuditTarget)'),
'// Suspicion-tier records: the access history is for the MLRO only (DR-219). The handler answers 404 (masked).\n@id("audit.history.suspicion")\nforbid (principal, action == Akr::Action::"audit.access_history", resource is Akr::AuditTarget)\nwhen { resource.tier == "suspicion" && !principal.roles.contains("M") };',
per_role("control.report",["M","SEC"],'permit (principal, action == Akr::Action::"access.control_report", resource is Akr::System)'),
])+"\n"
pk=lambda k: f'((resource is Akr::Pack && resource.kind == "{k}") || (context has pack_kind && context.pack_kind == "{k}"))'
CRD='[Akr::Action::"pack.create", Akr::Action::"pack.read", Akr::Action::"pack.download"]'
RD='[Akr::Action::"pack.read", Akr::Action::"pack.download"]'
gov = "\n\n".join([
"// C0_governance.cedar (phase 24). Registers, fairness, validation, command centre, clocks, packs of this phase,\n// assessor and training. Phase 06's C0_packs.cedar keeps cbi_gap, vendor_request and integration_profile.\n// One role per policy (conventions).",
per_role("gov.read",["M","CH","IA"],'permit (principal, action in [Akr::Action::"register.read", Akr::Action::"fairness.read",\n                              Akr::Action::"command.read", Akr::Action::"clock.read",\n                              Akr::Action::"training.read"], resource)'),
per_role("register.write.findings",["M","CH","MRO"],'permit (principal, action == Akr::Action::"register.write", resource is Akr::Register)','resource.kind == "finding"'),
'@id("register.close.m") @role("M")\npermit (principal, action == Akr::Action::"register.close", resource is Akr::Register)\nwhen { principal.roles.contains("M") };',
'@id("register.close.owner")\npermit (principal, action == Akr::Action::"register.close", resource is Akr::Register)\nwhen { resource has owner && resource.owner == principal };',
per_role("fairness.review",["M","MRO"],'permit (principal, action == Akr::Action::"fairness.review", resource)'),
per_role("validation.issue",["MRO"],'permit (principal, action == Akr::Action::"validation.issue", resource is Akr::ValidationPack)'),
'// GV-32: the validator is never the owner of the item.\n@id("validation.no_owner")\nforbid (principal, action == Akr::Action::"validation.issue", resource is Akr::ValidationPack)\nwhen { resource.owner == principal };',
per_role("validation.read",["MRO","IA","M"],'permit (principal, action == Akr::Action::"validation.read", resource is Akr::ValidationPack)'),
per_role("pack.board.create",["M"],'permit (principal, action == Akr::Action::"pack.create", resource)','context has pack_kind && context.pack_kind == "board"'),
per_role("pack.board.read",["M","CH","IA"],f'permit (principal, action in {RD}, resource is Akr::Pack)','resource.kind == "board"'),
per_role("pack.case",["M"],f'permit (principal, action in {CRD}, resource)',pk("case")),
per_role("pack.report",["M"],f'permit (principal, action in {CRD}, resource)',pk("report")),
per_role("pack.assessor",["AS"],f'permit (principal, action in {CRD}, resource)',pk("assessor")),
per_role("pack.assessor.read",["IA"],f'permit (principal, action in {RD}, resource is Akr::Pack)','resource.kind == "assessor"'),
per_role("pack.certification.read",["M","CH","IA"],f'permit (principal, action in {RD}, resource is Akr::Pack)','resource.kind == "certification"'),
per_role("pack.certification.create",["M"],'permit (principal, action == Akr::Action::"pack.create", resource)','context has pack_kind && context.pack_kind == "certification"'),
'@id("pack.verify")\npermit (principal, action == Akr::Action::"pack.verify", resource is Akr::System)\nwhen { principal.status == "active" };',
per_role("assessor.d2",["AS","IA"],'permit (principal, action == Akr::Action::"assessor.d2", resource is Akr::System)'),
per_role("assessor.sample",["AS"],'permit (principal, action == Akr::Action::"assessor.sample", resource is Akr::Sample)'),
per_role("assessor.trace",["AS"],'permit (principal, action == Akr::Action::"assessor.trace", resource is Akr::AuditTarget)','(resource.tier != "suspicion" || resource.in_own_sample)'),
per_role("assessor.trace",["IA"],'permit (principal, action == Akr::Action::"assessor.trace", resource is Akr::AuditTarget)','resource.tier != "suspicion"'),
per_role("training.record",["ADM"],'permit (principal, action in [Akr::Action::"training.record", Akr::Action::"training.void"], resource is Akr::TrainingRecord)'),
'@id("training.no_self")\nforbid (principal, action in [Akr::Action::"training.record", Akr::Action::"training.void"], resource is Akr::TrainingRecord)\nwhen { resource.user == principal };',
])+"\n"
a,b=block("**`60_audit.cedar`** (copy-paste ready"); s=s[:a]+audit_+s[b:]
a,b=block("**`C0_governance.cedar`** (copy-paste ready"); s=s[:a]+gov+s[b:]
old="**`60_audit.cedar`** (copy-paste ready; one role per tag, the `@role` tag names the roles of the policy):"
assert s.count(old)==1
s=s.replace(old,"**`60_audit.cedar`** (copy-paste ready; one role per policy and per `@role` tag, no `containsAny`; lead review):")
old="The policy `pack.assessor.as` lets IA read and download an assessor pack, not create one."
assert s.count(old)==1
s=s.replace(old,"The policy `pack.assessor.read.ia` lets IA read and download an assessor pack, not create one; `context.pack_kind` is always read behind `context has pack_kind`.")
open(p,'w').write(s)
print('ok', s.count('containsAny'))
