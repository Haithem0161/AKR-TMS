# Second re-check, phase 30 (Haiku draft for lead review)

### S-30-1  LOW   (fix F8, [R-30-6])
- Problem: The F8 text removed the claim that the signed bundle is already on offline media (section 3.11) from section 7.7, but the recovery table of section 3.8 still makes the same claim with the same pointer. Section 3.11 (lines 305 to 320) has no word about offline media, and line 259 places the offline copy in section 7+.
- Proof: docs/plan/phase-30.md:254 "the signed bundle at the DR site and on offline media (section 3.11)" and docs/plan/phase-30.md:259 "the offline copy that section 7+ defines (SP-88)".
- Smallest fix: In line 254 change "(section 3.11)" to "(section 7.7)".

### S-30-2  HIGH   (fix F11, [R-30-10a])
- Problem: The new text makes the allows select every pod of akr-restore, so an unlabelled known-bad probe pod is selected too and can reach the repository, the object store and OpenBao. Section 7.5 item 2 runs the known-bad probe of section 3.3 in akr-restore and requires its rows to pass before the restore starts, so the two rules cannot both hold.
- Proof: docs/plan/phase-30.md:589 "The allows select every pod of the namespace `akr-restore` (the Job and the scratch pod are its only pods), so no membership label is needed." and docs/plan/phase-30.md:590 "probes `akr-restore` with the same table and the known-bad probe of section 3.3, and its rows must pass before the restore starts." and docs/plan/phase-30.md:136 "a probe pod with no membership labels must reach nothing, and the check fails if it does."
- Smallest fix: In line 589 replace "The allows select every pod of the namespace `akr-restore` (the Job and the scratch pod are its only pods), so no membership label is needed." with a sentence that selects only the Job and the scratch pod by a label that they carry and the probe pod does not (for example `akr.io/restore-role: "true"`), or, if every-pod selection is kept, delete the known-bad probe from line 590.

### S-30-3  MEDIUM   (fix F12, [R-30-10b])
- Problem: Section 4.4 still says zones.yaml gains three keys that the renderer reads. The F12 text, and section 7.5 item 1 where the renderer writes the restore allows from that entry, make backup.repository a fourth such key, so the count and the list are wrong.
- Proof: docs/plan/phase-30.md:471 "`deploy/zones.yaml` gains three keys that the renderer reads (`cluster.apiserver`, `cbs.listeners`, `gateway.cidrs`)" and docs/plan/phase-30.md:589 "(the repository is its entry `backup.repository`)".
- Smallest fix: In line 471 change "gains three keys that the renderer reads (`cluster.apiserver`, `cbs.listeners`, `gateway.cidrs`)" to "gains four keys that the renderer reads (`cluster.apiserver`, `cbs.listeners`, `gateway.cidrs`, `backup.repository`)".

## Summary
- Entries checked: 12 (F1 to F12).
- Findings: HIGH 1, MEDIUM 1, LOW 1.
- CHECKED: F1 ok, F2 ok, F3 ok, F4 ok, F5 ok, F6 ok, F7 ok, F8 S-30-1, F9 ok, F10 ok, F11 S-30-2, F12 S-30-3
