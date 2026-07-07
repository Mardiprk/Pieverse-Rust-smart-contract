# Rust / Anchor (Solana) Skill Test

**Format:** Take-home practical assignment

---

## 🩹 Part 1 — Find and Fix the Bugs

The vault program has **several real issues** hiding in it — spanning account security, arithmetic safety, and account sizing. We're not going to point out exactly where they are; part of the exercise is finding them the way you would in a real code review or while writing tests.

To guide your investigation, here's what you should be able to do with a **correct** implementation — if any of these don't hold, there's a bug to fix:

- [ ] A user can deposit SOL into their vault and see their on-chain balance actually go up.
- [ ] A user can withdraw SOL they own, and only they — no one else should be able to withdraw from someone else's vault.
- [ ] Withdrawing more than the vault's current balance should fail with a clean, expected error — not a runtime panic or an unexpected account state.
- [ ] Vault accounts should be sized correctly for the data they store, with no leftover risk of corruption or wasted rent.

> 💡 Tip: Writing tests (or extending the starter test file) is a great way to surface these issues before you even start reading through the code line by line.

For each bug you find, briefly explain in your `NOTES.md`:
- What was wrong
- Why it was a problem (what could go wrong in production)
- How you fixed it

---

## ✨ Part 2 — Add Features

Once the program is correct, extend it with the following:

1. **`close_vault` instruction** — lets the owner close their vault and reclaim the rent-exempt SOL, provided the vault balance is zero (or automatically sweep any remaining balance to the owner first — your call, just document the behavior).
2. **Events** — emit an on-chain event (via Anchor's `emit!`) on both `deposit` and `withdraw`, including the amount and the vault's resulting balance.
3. **Per-transaction withdrawal limit** — add a `max_withdrawal` field set at `initialize` time; `withdraw` should reject any single withdrawal that exceeds it, with a clear custom error.
4. **Pause / unpause** — add a `paused` flag that only the vault owner can toggle; while paused, `deposit` and `withdraw` should both be rejected.

You don't need to gold-plate every feature — working, well-tested, reasonably clean code for all four is the goal. If you run out of time, implement as many as you can and note what's left in `NOTES.md`.

---

## 🛠️ Tech Stack

| Layer | Technology |
|---|---|
| Smart contract | Rust + Anchor (0.30.x) |
| Runtime | Solana |
| Testing | Anchor's TypeScript test harness (Mocha/Chai), against `solana-test-validator` |

---

## ✅ Evaluation Criteria

| Criteria | What we're looking for |
|---|---|
| **Correctness** | Bugs are actually fixed, not just papered over |
| **Security awareness** | You reason about who can call what, and with what accounts |
| **Rust/Anchor idioms** | Checked arithmetic, proper constraints (`has_one`, `seeds`, `mut`), custom errors instead of panics |
| **Testing** | Meaningful tests covering both the fixes and the new features |
| **Communication** | Clear `NOTES.md` explaining what you found and why you made the choices you did |

---
