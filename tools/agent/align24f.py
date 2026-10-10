p='docs/MODULES.md'
s=open(p).read()
R=[
("""DR-191), the jurisdiction profiles (Open doubt 9).
""",
"""DR-191), the jurisdiction profiles (Open doubt 9).
Phase file: docs/plan/phase-24.md (DR-218 to DR-231): the audit viewer (every search written as `audit.searched`,
`purpose` required), one-event view, chain and anchor status, verify runs as a job (counts and finding kinds only)
and the per-record access history, all read through a new akr-api read-only pool; suspicion-tier rows are left out
for IA and SEC through the SECURITY DEFINER `audit.resource_tier`, which mirrors phase 17 tier rule 2; screen G6.
"""),
("""bank-set age); the kill switch pressed during scoring (in-flight alerts keep their rule ranking); a model artifact
with a bad signature (refused at load).
""",
"""bank-set age); the kill switch pressed during scoring (in-flight alerts keep their rule ranking); a model artifact
with a bad signature (refused at load).
Phase file: docs/plan/phase-24.md (DR-218 to DR-231): validation packs as written-once versions with seven parts and
a computed verdict (the validator never the item owner; a `pass` moves the dates through phase 04); the findings and
warnings register (append-and-close, fed by phase 04 post-change reviews, the review calendar and fairness reviews);
decisions and exclusions as read views; the protected-attribute register and allowed-use matrix (never the sole
trigger); monthly fairness metrics with suppression and a review ratio; the derived review calendar; screens G4 and
G5. Not carried: models, scores and the assistant (P2, phases 27 and 28).
"""),
("""suspicion-tier case (allowed by the time-boxed grant, every view logged); a pack regenerated after data changed (new
version; the old one stays with its hash).
""",
"""suspicion-tier case (allowed by the time-boxed grant, every view logged); a pack regenerated after data changed (new
version; the old one stays with its hash).
Phase file: docs/plan/phase-24.md (DR-218 to DR-231): the live command centre (counts through each list's predicate,
1 to 4 suppressed, the tipping-off tile for M only), the read-only clocks board, the assessor workspace (role `AS`
with `valid_to`, the D2 map as a signed data file, seeded recorded samples, a metadata-only trace that reaches a
suspicion alert only through the assessor's own sample), pack kinds `board`, `case`, `report`, `assessor` and
`certification` on phase 06's builder (download step-up `export`, uniform verify answer), the training register
(recorded by ADM, never for oneself); screens G1, G8, G9, R4. Not carried: the capacity workbench, correspondent and
validator packs, the typology exposure index (P2), redaction and the requester watermark (Open doubt 11).
"""),
]
for a,b in R:
    assert s.count(a)==1,a[:60]
    s=s.replace(a,b)
open(p,'w').write(s)
print('ok',len(R))
