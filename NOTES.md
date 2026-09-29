## Fixes

**1. Vault was not market as changeable:** Vault Account in the deposit  section was missing 'mut'. Nobody could have put money in it. 

WHY: Without mut, the vault is read-only, so the deposit would fail.

**2. Money math problem**  Switched to checked_add and checked_sub, which stop with an error if something goes wrong because it was using plain += and -= on balances.                                                       

WHY: Plain += and -= can cause numeric overflow.Checked math fails early instead of wrapping around, which is critical for money math. 

**3. Zero amount deposits/withdrawal was allowed** No checks for depositing or withdrawing 0. Added 'require!()' checks to prevent this. Added a clear ZeroAmount error message.

WHY: It does nothing useful, wastes fees, and fills logs with noise.

**4. No check for owner permission** Anyone could close someone else's vault. Added a has_one = owner check to prevent this.

WHY: Anyone could close someone else's vault. Added a has_one = owner check to prevent this.

**5. Account size was worked out by hand** Used Anchor's #[derive(InitSpace)] macro to calculate account size automatically and safely.

WHY: Manual size calculation is a;lso good but InitSpace avoids off-by-one errors and keeps sizes in sync with struct changes.

**6. Transfer code was long and easy to get wrong** Used Anchor's built-in transfer helper instead. Short and clean.

WHY: while it works but using the solana transfer is error prone.

## Features Added: Events, Withdrawl Limits, Pause/unpause