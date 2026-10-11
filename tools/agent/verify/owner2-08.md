# Owner-decision check, round 2, key 08 (Haiku draft for lead review)

### P-08-1  MEDIUM   (fix F8, [O-08-1])
- Problem: The F8 text makes an A2 maker see the change request as text and follow it in the inbox tab "Waiting on others", so A2 gets no G10 link. The base X3 rule still gives every change-request task panel a link to G10 detail, and the maker's "Waiting on others" rows open that same panel. An A2 maker therefore still reaches G10 and lands on X6, the fault O-08-1 names.
- Proof: docs/plan/phase-08.md:719 "as text for A2, whom G10 does not list" conflicts with docs/plan/phase-08.md:403 "For a change-request task the panel links to G10 detail." Supporting lines: docs/plan/phase-08.md:398 "Opening a row shows the **task panel** (`getTasksId`)"; docs/plan/phase-04.md:1156 "`waiting`: running instances whose maker is the caller, one row per open task"; docs/plan/phase-04.md:778 "config.author, config.approve, admin.users, admin.policies, admin.system, security.ops, audit.read" (the G10 detail takes the list's scopes, phase-04.md:780); docs/API.md:85, 86 and 93 show that A2 holds only queue.work, case.decide and report.prepare.
- Smallest fix: In docs/plan/phase-08.md line 403, change "For a change-request task the panel links to G10 detail." to "For a change-request task the panel links to G10 detail for a role that G10 lists; for any other role it shows the change request as text."

### P-08-2  LOW   (fix F6, [O-08-8])
- Problem: "no response gives the size limit" holds for the form's users (A2 and M) but not for ADM: GET /admin/settings lists the settings with their floors, bounds and governed mark, and goaml.corpus_case_max_bytes is a row of admin.settings. The fix still works (the client checks the type only and the server answers 422 file_too_large), so only the wording is wrong.
- Proof: docs/plan/phase-08.md:719 "no response gives the size limit"; docs/plan/phase-03.md:515 "| GET | `/admin/settings` | admin.system | Settings with floors, bounds and the governed mark |"; docs/plan/phase-07.md:1107 "('goaml.corpus_case_max_bytes', '1048576', 'integer', NULL, FALSE,"; docs/API.md:104 "| `admin.system` | Settings, flags, jobs, releases, ops cockpit | ADM |".
- Smallest fix: In docs/plan/phase-08.md line 719, change "no response gives the size limit" to "no response that this form receives gives the size limit".

## Summary
- Entries checked: 12 (F1 to F12, all in docs/plan/phase-08.md).
- Findings by severity: HIGH 0, MEDIUM 1, LOW 1.
- CHECKED: F1 ok, F2 ok, F3 ok, F4 ok, F5 ok, F6 P-08-2, F7 ok, F8 P-08-1, F9 P-08-1, F10 ok, F11 ok, F12 ok
