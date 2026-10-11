#!/usr/bin/env python3
"""Owner-decision round 1: the phase 08 line that follows from O-07-6 (the hint at the key field)."""
import sys
sys.path.insert(0, "/home/haithem/Projects/AKR-TMS/tools/agent")
from ofix import Fixer

f = Fixer(round_no=1)
f.fix("O-07-6", "docs/plan/phase-08.md",
      "the key (it starts with a lower-case letter, then lower-case letters, digits or hyphens; 3 to 61 characters in all);",
      "the key (it starts with a lower-case letter, then lower-case letters, digits or hyphens; 3 to 61 characters in all; under the field the hint \"A short code for the rule. Never a name.\" in the user's language, phase 07 section 7.6 item 13);")
f.done()
