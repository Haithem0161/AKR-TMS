# Re-check, phase 30 (Haiku draft for lead review)

### R-30-1  HIGH   (fix F4)
- Problem: The decision rule makes Patroni the default whenever it meets all ten criteria, with no exception, but the tie-break in the same paragraph says that when both candidates qualify the bank's acceptance of another backup tool decides, and that without it Patroni stays the default. When the bank accepts another tool both candidates qualify (Patroni still meets criterion 5), so the two sentences name different defaults.
- Proof: docs/plan/phase-30.md:160 "Patroni is the default when it meets all ten criteria, criterion 6 being the acceptance of its footprint by the bank's operators." and docs/plan/phase-30.md:160 "the bank's acceptance of another backup tool decides: without it Patroni stays the default."
- Smallest fix: In docs/plan/phase-30.md line 160, change "Patroni is the default when it meets all ten criteria" to "Patroni is the default when it meets all ten criteria and the bank accepts no other backup tool".

### R-30-2  MEDIUM   (fix F8)
- Problem: Item 1 of the 7.6 block makes the method return `None` on an overlap, yet the same sentence has the probe log and the command print the id of the row found, and the return type `Result<Option<BlindPeriod>, AppError>` carries no id.
- Proof: docs/plan/phase-30.md:597 "When one exists it writes nothing and returns `None`: the probe logs the id of the row found and still writes `ha.failover_observed`, and the command exits 2 and prints that id."
- Smallest fix: In docs/plan/phase-30.md line 597 replace "writes nothing and returns `None`" with "writes nothing and returns `Ok(Err(id))` with the id of the overlapping row (return type `Result<Result<BlindPeriod, Uuid>, AppError>`)".

### R-30-3  MEDIUM   (fix F8)
- Problem: Line 63 has the `ha outage record` command write "through the `akr-ctl` database role of phase 00", a role that phase 00 does not define, while the 7.6 block says both callers connect as `akr_app`, so the phase names two different roles for the same write.
- Proof: docs/plan/phase-30.md:63 "uses phase 11 `BlindPeriodService` through the `akr-ctl` database role of phase 00; no new repository" and docs/plan/phase-30.md:596 "Both callers connect as `akr_app`, which holds `SELECT, INSERT` on `admin.blind_periods`".
- Smallest fix: In docs/plan/phase-30.md line 63 replace "through the `akr-ctl` database role of phase 00" with "as `akr_app` (section 7.6)".

### R-30-4  LOW   (fix F9)
- Problem: The new pointer says phase 11 section 3.6 owns the table, but the table's DDL is in phase 11 section 1.1, which the same subsection cites a line earlier as "phase 11 section 1 DDL".
- Proof: docs/plan/phase-30.md:229 "(phase 11 section 1 DDL)" and docs/plan/phase-30.md:230 "phase 11 section 3.6 owns the table and the anchor rule".
- Smallest fix: In docs/plan/phase-30.md line 230 replace "phase 11 section 3.6 owns the table and the anchor rule" with "phase 11 section 1.1 defines the table and section 3.6 owns the anchor rule".

### R-30-5  MEDIUM   (fix F11)
- Problem: Item 4(d) of the 7.7 block names `akr-ctl assess keys rotate`, a command that exists nowhere in docs/plan; phase 29 defines the decision-key rotation as `akr-ctl assess keys generate --purpose decision`, so an operator following the runbook would run a command that is not there.
- Proof: docs/plan/phase-30.md:608 "a new decision key (`akr-ctl assess keys rotate`)" and docs/plan/phase-29.md:979 "`akr-ctl assess keys generate --purpose decision`, run by an operator with `admin.users`".
- Smallest fix: In docs/plan/phase-30.md line 608 replace "(`akr-ctl assess keys rotate`)" with "(`akr-ctl assess keys generate --purpose decision`, phase 29 section 3.4)".

### R-30-6  LOW   (fix F11)
- Problem: Item 1 of the 7.7 block says the signed release bundle "is already on offline media (section 3.11)", but section 3.11 never mentions offline media, and no phase states that a copy of the bundle is kept offline (phase 25 says only that bundles can be delivered offline).
- Proof: docs/plan/phase-30.md:605 "The signed release bundle is already on offline media (section 3.11)." and docs/plan/phase-30.md:308 "1. Verify the bundle first (section 3.12). A refused bundle stops the upgrade before any change."
- Smallest fix: In docs/plan/phase-30.md line 605 replace "is already on offline media (section 3.11)" with "is kept on offline media by the bank, a procedure that section 3.11 does not yet state, so add that sentence to section 3.11".

### R-30-7  LOW   (fix F11)
- Problem: Item 1 of the 7.7 block says the pgBackRest repository "is encrypted, phase 00", but the phase 00 `pgbackrest` row names the schedule and the repository location and no encryption, and no phase 00 line mentions a pgBackRest cipher.
- Proof: docs/plan/phase-30.md:605 "the pgBackRest repository as it stands after the newest full backup (it is encrypted, phase 00)" and docs/plan/phase-00.md:694 "WAL every 60 s, weekly full, nightly differential, four fulls; repository on the backup server plus offsite (SP-84)".
- Smallest fix: In docs/plan/phase-30.md line 605 replace "(it is encrypted, phase 00)" with "(it is encrypted; phase 00 must state the encryption in its `pgbackrest` row)".

### R-30-8  LOW   (fix F11)
- Problem: Item 4 of the 7.7 block says the transit keys "are rotated by the key ceremony of phase 00", but phase 00 never uses the term "key ceremony" and describes the OpenBao ceremony runbook as creating the transit keys once per install, not rotating them.
- Proof: docs/plan/phase-30.md:608 "The transit keys are rotated by the key ceremony of phase 00 when the bank's security officer finds that OpenBao itself was exposed." and docs/plan/phase-00.md:1101 "are created once per install by the OpenBao ceremony runbook (SP-12)".
- Smallest fix: In docs/plan/phase-30.md line 608 replace "the key ceremony of phase 00" with "the OpenBao ceremony runbook (SP-12, phase 00), which the bank runs for rotation as well as for creation".

### R-30-9  MEDIUM   (fix F12)
- Problem: Fix F12 makes the offline copy the only copy an attacker on the network cannot change, and section 7.7 and Open doubts 22 say no repository is object-locked, yet section 3.9 still lets the ransomware restore run "from the immutable or offline copy", which keeps the old rule in a second line.
- Proof: docs/plan/phase-30.md:291 "The ransomware variant (SP-88) is this command run on a clean environment from the immutable or offline copy" and docs/plan/phase-30.md:259 "the copy that an attacker on the network cannot change is the offline copy that section 7+ defines (SP-88)".
- Smallest fix: In docs/plan/phase-30.md line 291 replace "from the immutable or offline copy" with "from the offline copy (section 7.7)".

### R-30-10  MEDIUM   (fix F5)
- Problem: Item 1 of the 7.5 block allows the restore pods egress to "the pgBackRest repository ... on the ports `zones.yaml` names", but the plan gives zones.yaml no edge or address for the repository (section 3.1 lists the external addresses without it, and phase 00 names edges for OpenBao and the object store but none for the repository), and the block does not say which membership label the restore pods carry, which each rendered allow needs.
- Proof: docs/plan/phase-30.md:589 "egress from the two to the pgBackRest repository, the object store and OpenBao on the ports `zones.yaml` names for those destinations" and docs/plan/phase-30.md:77 "and the external addresses (CBS listeners, bank SMTP, SIEM, the proxy of phase 07, the payment hub CIDRs of section 3.3)".
- Smallest fix: In docs/plan/phase-30.md line 589 name the membership label that the restore Job and the scratch pod carry (for example `akr.io/net-z2_app`), and add the pgBackRest repository address and port to the external addresses on line 77.

## Summary
- Fixes checked: 18 (F1 to F18).
- Findings: HIGH 1, MEDIUM 5, LOW 4 (10 in total).
- CHECKED: F1 ok, F2 ok, F3 ok, F4 R-30-1, F5 R-30-10, F6 ok, F7 ok, F8 R-30-2 and R-30-3, F9 R-30-4, F10 ok, F11 R-30-5 to R-30-8, F12 R-30-9, F13 ok, F14 ok, F15 ok, F16 ok, F17 ok, F18 ok
