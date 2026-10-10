# Re-check, phase 28 (Haiku draft for lead review)

### R-28-1  HIGH   (fix F12)
- Problem: The NEW text says each `<` in a record is written as a JSON escape, but the escape is typed as a bare less-than sign, which is the character it replaces. The same sentence then says the block text holds no `<`, and that cannot hold. Without a real escape, the guard against a record that contains `<<` or the nonce is not implemented. No JSON escape occurs anywhere in docs/plan/phase-28.md. The bare sign also appears in tools/agent/verify/applied-28.txt (F12 NEW).
- Proof: docs/plan/phase-28.md:479 "is serialised with every `<` written as the JSON escape `<`, so the block text holds no `<`"
- Smallest fix: In line 479, replace the bare `<` after "as the JSON escape" with the six characters backslash, u, 0, 0, 3, c (the JSON escape of less-than), so the escape differs from the character it replaces.

## Summary
- Fixes checked: 12 (F1 to F12).
- Findings by severity: HIGH 1, MEDIUM 0, LOW 0.
CHECKED: F1 ok, F2 ok, F3 ok, F4 ok, F5 ok, F6 ok, F7 ok, F8 ok, F9 ok, F10 ok, F11 ok, F12 R-28-1
