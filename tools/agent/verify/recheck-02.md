# Re-check, phase 02 (Haiku draft for lead review)

### R-02-1  HIGH   (fix F11)
- Problem: 7.17 step 2 of `Verifier::run` rebuilds the case_event heads from the rows of `casework.case_events`, but `Verifier::run` reads as `akr_readonly` (base text), and `akr_readonly` has no SELECT on any casework table. The block adds no grant and no definer function, so that step fails with permission denied for this stream.
- Proof: docs/plan/phase-02.md:1327 "rebuilds the heads from the rows of `[range_from, range_to]`" and docs/plan/phase-02.md:780 "reads as `akr_readonly`" (second quote, base text); cross-phase: docs/plan/phase-17.md:568 "No grant to akr_readonly, akr_ingest, akr_audit_writer or akr_monitor".
- Smallest fix: In the 7.17 Verifier bullet, name the role that reads the case rows in step 2 and give it a grant that exists: `akr_app`, which phase 17 grants SELECT on `casework.case_events` (line 558), or a SECURITY DEFINER function returning `(case_id, seq, at, hash)` for a range, executable by `akr_readonly` and added to phase 17.

### R-02-2  HIGH   (fix F8)
- Problem: 7.16 defines `ShredOutcome` as the struct `{ shredded_at: DateTime<Utc> }`, but phase 23 section 7.8 defines the same name as the enum `{ Shredded, AlreadyShredded }`, the value that `EvidenceShredder::shred` (the same method, named in phase 23 line 500) returns. One name, two shapes; both cannot hold.
- Proof: docs/plan/phase-02.md:1302 "`ShredOutcome` is what `shred` returns (7.3): `{ shredded_at: DateTime<Utc> }`" and cross-phase: docs/plan/phase-23.md:1048 "`ShredOutcome { Shredded, AlreadyShredded }`. `EvidenceShredder::shred` on a row whose `shredded_at` is set returns `AlreadyShredded`".
- Smallest fix: Replace the 7.16 text with "`ShredOutcome` is the enum `{ Shredded, AlreadyShredded }` of phase 23 section 7.8, which `shred` returns; `shredded_at` is written only by the `Shredded` call, in the same transaction."

### R-02-3  HIGH   (fix F6)
- Problem: 7.14 says `RenderJob` has no template name and no template version, so `RenderService::render` cannot know the template. Base step 4 (and the section 4 metric list at line 1061) still has the service record `akr_render_duration_seconds{template}`, which cannot be filled. The impact is one metric label, but the two rules cannot both hold.
- Proof: docs/plan/phase-02.md:905 "`akr_render_duration_seconds{template}`, `akr_render_failures_total{reason}`" and docs/plan/phase-02.md:1293 "It has no template name and no template" (second quote, the 7.14 block).
- Smallest fix: In 7.14, either make the template name a parameter of `RenderService::render` (not a RenderJob field) or drop `{template}` from line 905 and from line 1061, and say which in 7.14.

### R-02-4  MEDIUM   (fix F17)
- Problem: The 7.1 row keeps a dash for all three owner kinds, and the column rule says a dash means the owning phase states no retention. Phase 24 states one for `register_export` (R6 for the kind `decisions`, R9 for the other registers) and states R11 for `validation_pack` (phase 24 line 704), so the dash is wrong for two of the three kinds.
- Proof: docs/plan/phase-02.md:1185 "| `validation_pack`, `training_record`, `register_export` | 24 | - |" and, cross-phase, docs/plan/phase-24.md:1348 "retention class `R6` for the kind `decisions` and `R9` for every other register of this phase".
- Smallest fix: Split the row in 7.1 into `register_export` | 24 | R6 (decisions), R9 (other registers); `validation_pack` | 24 | R11; and keep `training_record` | 24 | -.

### R-02-5  MEDIUM   (fix F5)
- Problem: 7.13 puts "the phase 02 kinds whose row says SIEM" into the SIEM flag set. The row of `evidence.rejected` says "SIEM when infected", but the catalogue flag is a boolean (`siem: bool`, line 550), so a developer cannot tell whether that kind is always exported or only when infected.
- Proof: docs/plan/phase-02.md:978 "`evidence.rejected` | infected or rejected (reason, signature, engine version); SIEM when infected" and docs/plan/phase-02.md:1288 "and the phase 02 kinds whose row says SIEM" (second quote, the 7.13 block).
- Smallest fix: In 7.13, replace "and the phase 02 kinds whose row says SIEM" with the two unconditional kinds (`audit.chain_break_detected`, `evidence.hash_mismatch`) and one sentence saying whether `evidence.rejected` is always exported or only for the infected outcome.

## Summary
- Fixes checked: 18 (F1 to F18 of applied-02.txt).
- Findings: HIGH 3, MEDIUM 2, LOW 0.
CHECKED: F1 ok, F2 ok, F3 ok, F4 ok, F5 R-02-5, F6 R-02-3, F7 ok, F8 R-02-2, F9 ok, F10 ok, F11 R-02-1, F12 ok, F13 ok, F14 ok, F15 ok, F16 ok, F17 R-02-4, F18 ok
