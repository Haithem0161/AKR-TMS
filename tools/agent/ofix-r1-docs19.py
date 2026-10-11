#!/usr/bin/env python3
"""Owner-decision round 1: system documents in step with the phase 19, 14 and 07 fixes."""
import sys
sys.path.insert(0, "/home/haithem/Projects/AKR-TMS/tools/agent")
from ofix import Fixer

f = Fixer(round_no=1)
f.fix("O-19-17", "docs/MODULES.md",
      "sources, two of them for the MLRO only), through a map",
      "sources, four of them for the MLRO only), through a map")
f.fix("O-19-18", "docs/DESIGN-SYSTEM.md",
      "\"Propose changes\" (CFG) sets or removes rows through a change request. CHK, M and RFI\nread the tab.",
      "\"Propose changes\" (CFG) sets or removes rows through a change request. CHK and M\nread the tab; the RFI desk reads the map on W8.")
f.fix("O-19-17", "docs/DESIGN-SYSTEM.md",
      "A proposal from a\nsensitive source says \"Needs the MLRO\" for a caller who is not the MLRO.",
      "A proposal from a\nsensitive source (four of the ten) says \"Needs the MLRO\" for a caller who is not the MLRO; the MLRO sees the\nactivity figures only under the purpose Investigation.")
f.done()
