#!/usr/bin/env python3
"""Owner round 1 fixes: phase 18 (O-18-1) and phase 08 (O-08-1 to O-08-8). Verdicts: tools/agent/owner-verdict.md."""
import sys
sys.path.insert(0, "/home/haithem/Projects/AKR-TMS/tools/agent")
from ofix import Fixer

f = Fixer(1)
P18, P08 = "docs/plan/phase-18.md", "docs/plan/phase-08.md"

f.fix("O-18-1", P18,
      "that the row already returns (section 3.14 step 5), so the bank",
      "that the row already returns (section 3.14, step 5 of `GET /parties/{id}/transactions`), so the bank")
f.fix("O-18-1-note", P18,
      "and the copied value is an id the caller already holds. Test: the menu",
      "and the copied value is an id the caller already holds. This is so at every tier: the value is a UUID in a response the caller was already given, and it names no person. Test: the menu")

# O-08-4: section 7.9 keeps only what stands
f.fix("O-08-4", P08,
      "the rest stands. The text of the verification pass was: The `Name` component shows what the source gives. The original is the party's `canon.party_name` row in Arabic or Kurdish script; the transliteration beneath it is the row of the same party and `name_type` with `script = 'lat'`, when the source supplied one, and its source label is the word `typed`: a Latin name held in a CBS column was keyed by staff, and no mapped field says that it was read from a card or a passport (`source_field` is a bare column name, phase 10). A party with no Latin row shows no second line. The product generates no Latin form and reads no document, so the labels `card`, `passport` and `generated` of DESIGN-SYSTEM 7.6 are not used in this plan; phase 14's back-transliteration serves matching only.",
      "what follows stands. The `Name` component shows what the source gives. The original is the party's `canon.party_name` row in Arabic or Kurdish script; the transliteration beneath it is the row of the same party and `name_type` with `script = 'lat'`, when the source supplied one. A party with no Latin row shows no second line. The product generates no Latin form and reads no document.")
# O-08-5
f.fix("O-08-5", P08,
      "The source label is shown after the Latin line, in the muted small style of DESIGN-SYSTEM 7.6, as a word",
      "The source label is shown after the Latin line, smaller, as DESIGN-SYSTEM 7.6 says, and in the secondary text colour (`--color-stone`), as a word")
# O-08-6
f.fix("O-08-6", P08,
      "Phase 14's back-transliteration serves matching only and is never shown under a name.",
      "`Name` shows only a Latin row that the source gave; a back-transliteration made for matching (phase 14) is never passed to it.")
# O-08-7 (item 1 and item 3), O-08-1 (items 3 and 4), O-08-2, O-08-3, O-08-8
f.fix("O-08-7a", P08,
      "read with `getGoamlCorpusCases` for the profile code shown.",
      "read with `getGoamlCorpusCases` for the profile code of the version list (`IQ` unless the bank keeps a second profile code).")
f.fix("O-08-3a", P08,
      "the expected result in words (\"Would be rejected\" with its rule codes, or \"Accepted\")",
      "the expected result in words (`corpus_case.expect.would_reject` with its rule codes, or `corpus_case.expect.graded`)")
f.fix("O-08-8", P08,
      "A form: `FileDrop` limited to one XML file; the key (lower-case letters, digits and hyphens, 3 to 61 characters);",
      "A form: `FileDrop` limited to one XML file (the client checks the type only; no response gives the size limit, so a file that is too large is the server's 422 `file_too_large`, shown at the file field); the key (it starts with a lower-case letter, then lower-case letters, digits or hyphens; 3 to 61 characters in all);")
f.fix("O-08-3b", P08,
      "the expected outcome (two choices, \"Would be rejected\" and \"Accepted\"); the expected block codes, a multi-select over the rule codes of the active version, enabled only with \"Would be rejected\";",
      "the expected outcome (two choices, with the words of `corpus_case.expect.would_reject` and `corpus_case.expect.graded`); the expected block codes, a multi-select over the rule codes of the active version, enabled only with the first choice;")
f.fix("O-08-1a", P08,
      "Submit sends `postGoamlCorpusCases` (multipart) with a generated `Idempotency-Key`; on 202 the page links to the change request on G10.",
      "Submit sends `postGoamlCorpusCases` (multipart) with a generated `Idempotency-Key` and with `profile_code` = the profile code of item 1. On 202 the page shows the reference of the change request: as a link to G10 for M, whom G10 lists, and as text for A2, whom G10 does not list and who follows the request in the inbox tab \"Waiting on others\" (X3). An A2 maker therefore has no withdraw screen in this version; an unwanted proposal is closed by the approver's reject (phase 07 section 7.6 item 7).")
f.fix("O-08-1b", P08,
      "then `postGoamlCorpusCasesIdRetire`; on 202 the link to the change request.",
      "then `postGoamlCorpusCasesIdRetire`; on 202 the reference of the change request, shown as in item 3.")
f.fix("O-08-3c", P08,
      "| `corpus_case.state.rejected` | مرفوضة | Rejected |\n",
      "| `corpus_case.state.rejected` | مرفوضة | Rejected |\n| `corpus_case.expect.would_reject` | سيُرفض | Would be rejected |\n| `corpus_case.expect.graded` | مقبول | Accepted |\n")
f.fix("O-08-3d", P08,
      "the form with \"Accepted\" chosen disables the block-code select;",
      "the form with the choice `corpus_case.expect.graded` (\"Accepted\") disables the block-code select;")
f.done()
