# Re-check, phase 27 (Haiku draft for lead review)

### R-27-1  LOW   (fix F4)
- Problem: The `bins` of `ScoreWindow` point to "the 7.x block on bins". "7.x" is a template placeholder; the block that defines the ten score bins is section 7.10.
- Proof: docs/plan/phase-27.md:907 "`bins` are the ten score counts of the 7.x block on bins"
- Smallest fix: At line 907, replace "the 7.x block on bins" with "section 7.10".

### R-27-2  LOW   (fix F8)
- Problem: The provisional cut compares a cumulative share (a fraction from 0 to 1) with `model.top_band_percent`, which is the integer 10 in per cent. Read literally, the share never reaches 10, so the cut is never found.
- Proof: docs/plan/phase-27.md:915 "the cumulative share of the reference counts reaches `model.top_band_percent`"; docs/plan/phase-27.md:318 "Share of highest scores that form the top band (per cent)"
- Smallest fix: At line 915, replace "reaches `model.top_band_percent`" with "reaches `model.top_band_percent` / 100 (the setting is in per cent)".

### R-27-3  LOW   (fix F9)
- Problem: The section 3.11 row for 428 and 409 still lists If-Match on promote, pin and engage only. Shadow now answers 428 and 409 as well (section 2 route row 9 and the new check 8 item q).
- Proof: docs/plan/phase-27.md:608 "`If-Match` on promote, pin, engage"; docs/plan/phase-27.md:918 "`POST /models/{id}/shadow` without `If-Match` answers 428"
- Smallest fix: At line 608, replace "promote, pin, engage" with "promote, pin, shadow, engage".

### R-27-4  LOW   (fix F11)
- Problem: The new 422 at `/version_id` is raised by pin in section 3.10, but the E_VALIDATION row of section 3.11 lists sections 3.1, 3.3, 3.6 and 3.8 only.
- Proof: docs/plan/phase-27.md:599 "sections 3.1, 3.3, 3.6, 3.8"; docs/plan/phase-27.md:921 "is 422 `E_VALIDATION` at `/version_id`"
- Smallest fix: At line 599, replace "sections 3.1, 3.3, 3.6, 3.8" with "sections 3.1, 3.3, 3.6, 3.8, 3.10".

### R-27-5  LOW   (fix F16)
- Problem: The kill state is re-read before each 50-alert chunk, but step 6 still writes one transaction and one audit per batch of up to 500 alerts, so the fix does not say what a stopped batch writes. If step 6 runs after a stop, the chunks scored before the engage are written after it.
- Proof: docs/plan/phase-27.md:496 "scored in chunks of 50 alerts; step 1 is applied again before each chunk"; docs/plan/phase-27.md:500 "Insert `alert.model_scores` in one transaction per batch"
- Smallest fix: In step 6 at line 500, replace "in one transaction per batch" and "per batch" with "per chunk", and state that a stopped batch keeps only its committed chunks.

### R-27-6  LOW   (fix F31)
- Problem: `revert` is a required method of the phase 04 trait, so "is not implemented" leaves its return value open; phase 16 gives the convention, a return of `ChangeError::NotRevertible` (docs/plan/phase-16.md:1467). It is never called here because `emergency_allowed` is false, so this is wording only.
- Proof: docs/plan/phase-27.md:924 "`revert` is not implemented"; docs/plan/phase-04.md:653 "async fn revert(&self, tx: &mut Tx, cr: &ChangeRequest, actor: ActorRef) -> Result<(), ChangeError>;"
- Smallest fix: At line 924, replace "`revert` is not implemented" with "`revert` returns `ChangeError::NotRevertible`, as in phase 16".

### R-27-7  MEDIUM   (fix F38)
- Problem: F38 makes a suspicion-tier alert answer 401 to a caller without `record_open` step-up before any visibility check, but two lines still state the old rule (the tipping-off row at line 714 and the E_NOT_FOUND row at line 603). The 401 also tells a caller without step-up that the id is a suspicion-tier alert, which section 3.14 says nobody outside the team may learn, so the lead must decide whether F38 stands (F46 at line 874 repeats the same rule and is consistent with it).
- Proof: docs/plan/phase-27.md:505 "answers 401 `E_STEPUP_REQUIRED` first"; docs/plan/phase-27.md:714 "a suspicion row is 404 for everyone else"; docs/plan/phase-27.md:603 "route 16 for suspicion-tier or invisible alerts"
- Smallest fix: Replace "a suspicion row is 404 for everyone else" at line 714 and "route 16 for suspicion-tier or invisible alerts" at line 603 with the F38 answers (401 first for a suspicion-tier alert without step-up, masked 404 for invisible rows and for suspicion-tier rows after step-up).

## Summary

- Fixes checked: 46 (F1 to F46). Each NEW text was found in docs/plan/phase-27.md; the OLD text of every REP is gone from the phase; the names used by the fixes were checked against phases 00, 01, 02, 04, 13, 16, 17, 18, 24 and 28; the route numbers of every "route N" reference match the section 2 table.
- Findings by severity: HIGH 0, MEDIUM 1 (R-27-7), LOW 6 (R-27-1 to R-27-6).
- CHECKED: F1 ok, F2 ok, F3 ok, F4 R-27-1, F5 ok, F6 ok, F7 ok, F8 R-27-2, F9 R-27-3, F10 ok, F11 R-27-4, F12 ok, F13 ok, F14 ok, F15 ok, F16 R-27-5, F17 ok, F18 ok, F19 ok, F20 ok, F21 ok, F22 ok, F23 ok, F24 ok, F25 ok, F26 ok, F27 ok, F28 ok, F29 ok, F30 ok, F31 R-27-6, F32 ok, F33 ok, F34 ok, F35 ok, F36 ok, F37 ok, F38 R-27-7, F39 ok, F40 ok, F41 ok, F42 ok, F43 ok, F44 ok, F45 ok, F46 ok
