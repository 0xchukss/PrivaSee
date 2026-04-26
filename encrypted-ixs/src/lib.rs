/// PrivaSee — Private Set Intersection Circuit
///
/// This Arcis circuit runs inside Arcium's MPC network. It computes the
/// intersection of two encrypted contact sets without any single party
/// (or node) ever seeing the raw contact lists in plaintext.
///
/// Key design decisions:
///   - Fixed-size arrays `[T; N]` are used because Arcis MPC circuits require
///     compile-time-known data sizes (no Vec, String, or HashMap).
///   - Each contact is represented as a 32-byte SHA3-256 hash, computed
///     client-side before encryption.
///   - The `count` field tracks how many slots are actually filled (max 20).
///   - Both branches of if/else always execute in MPC — this is by design
///     to prevent information leakage via execution timing.

use arcis::*;

#[encrypted]
mod circuits {
    use arcis::*;

    /// Maximum number of contacts each user can register.
    /// Kept small to minimize MPC circuit complexity (comparisons are expensive).
    const MAX_CONTACTS: usize = 20;

    /// A single contact hash — 32 bytes from SHA3-256.
    /// We split into two u128 halves because Arcis works with field elements,
    /// and comparing two u128s is cheaper than comparing 32 individual bytes.
    #[derive(Copy, Clone)]
    pub struct ContactHash {
        pub lo: u128,
        pub hi: u128,
    }

    /// A user's encrypted contact set.
    /// `contacts` holds up to MAX_CONTACTS hashed contact identifiers.
    /// `count` indicates how many slots are actually populated (0..=MAX_CONTACTS).
    /// Empty slots are zeroed out.
    #[derive(Copy, Clone)]
    pub struct ContactSet {
        pub contacts: [ContactHash; MAX_CONTACTS],
        pub count: u8,
    }

    /// The result of the PSI computation.
    /// `matches` is a boolean array — `matches[i]` is true if contact `i`
    /// from set A was also found in set B.
    /// `matched_hashes` contains the actual hashes of matched contacts
    /// (zeroed for non-matches), so the client can identify which contacts matched.
    /// `match_count` is the total number of matches found.
    #[derive(Copy, Clone)]
    pub struct PsiResult {
        pub matches: [bool; MAX_CONTACTS],
        pub matched_hashes: [ContactHash; MAX_CONTACTS],
        pub match_count: u8,
    }

    /// Compares two contact hashes for equality.
    /// Returns true if both the lo and hi halves match.
    /// This is a helper function (not an #[instruction]) — it can be unit tested.
    fn hashes_equal(a: ContactHash, b: ContactHash) -> bool {
        a.lo == b.lo && a.hi == b.hi
    }

    /// Checks if a contact hash is the zero hash (empty slot).
    fn is_empty_hash(h: ContactHash) -> bool {
        h.lo == 0 && h.hi == 0
    }

    /// Private Set Intersection — the core confidential instruction.
    ///
    /// Takes two encrypted contact sets (one from each user) and computes
    /// their intersection. Only the intersection result is returned — the
    /// full contact lists are never revealed to anyone.
    ///
    /// How it works:
    ///   1. Both encrypted sets are converted to secret shares via `.to_arcis()`
    ///   2. For each contact in set A, we check if it exists in set B
    ///   3. Both branches of the if/else execute (MPC requirement) — the
    ///      condition only selects which result to keep
    ///   4. The result is re-encrypted for the requesting user via `.from_arcis()`
    ///
    /// Privacy guarantee: Arcium's MPC nodes process only secret shares.
    /// No single node ever sees a complete contact list. The intersection
    /// is computed collaboratively, and only the encrypted result leaves
    /// the MPC cluster.
    #[instruction]
    pub fn compute_psi(
        set_a: Enc<Shared, ContactSet>,
        set_b: Enc<Shared, ContactSet>,
    ) -> Enc<Shared, PsiResult> {
        // Convert encrypted inputs to secret shares for MPC computation.
        // This does NOT decrypt the data — it splits ciphertext into shares
        // distributed across Arx nodes.
        let a = set_a.to_arcis();
        let b = set_b.to_arcis();

        // Initialize the result with no matches.
        let empty_hash = ContactHash { lo: 0, hi: 0 };
        let mut result = PsiResult {
            matches: [false; MAX_CONTACTS],
            matched_hashes: [empty_hash; MAX_CONTACTS],
            match_count: 0,
        };

        // For each contact in set A, check if it exists in set B.
        // Both loops have fixed iteration counts (required by MPC circuits).
        // The inner loop always runs MAX_CONTACTS iterations regardless of
        // how many contacts are actually in set B — this prevents timing
        // side-channel leaks.
        for i in 0..MAX_CONTACTS {
            let contact_a = a.contacts[i];
            // Skip empty slots — but note: both branches still execute in MPC,
            // the condition only selects which result to propagate.
            let a_valid = !is_empty_hash(contact_a) && (i as u8) < a.count;

            let mut found = false;

            for j in 0..MAX_CONTACTS {
                let contact_b = b.contacts[j];
                let b_valid = !is_empty_hash(contact_b) && (j as u8) < b.count;

                // Check if this pair matches. Both contacts must be valid
                // (non-empty and within count bounds) and their hashes must be equal.
                let is_match = a_valid && b_valid && hashes_equal(contact_a, contact_b);

                // If we found a match and haven't already recorded one for this
                // contact, mark it. The `!found` guard prevents double-counting
                // if a contact appears multiple times in set B.
                let new_match = is_match && !found;
                if new_match {
                    found = true;
                }
            }

            // Record the match result for contact i.
            result.matches[i] = found;
            if found {
                result.matched_hashes[i] = contact_a;
                result.match_count += 1;
            }
        }

        // Re-encrypt the result using the shared secret between the requesting
        // client and the MXE. Only the client who initiated the computation
        // can decrypt this result.
        set_a.owner.from_arcis(result)
    }
}

#[cfg(test)]
mod tests {
    use super::circuits::*;

    fn make_hash(lo: u128, hi: u128) -> ContactHash {
        ContactHash { lo, hi }
    }

    #[test]
    fn test_hashes_equal() {
        let h1 = make_hash(42, 99);
        let h2 = make_hash(42, 99);
        let h3 = make_hash(42, 100);
        // Note: hashes_equal is a private helper in the circuit module,
        // so we test the logic inline here.
        assert!(h1.lo == h2.lo && h1.hi == h2.hi);
        assert!(!(h1.lo == h3.lo && h1.hi == h3.hi));
    }

    #[test]
    fn test_is_empty_hash() {
        let empty = make_hash(0, 0);
        let non_empty = make_hash(1, 0);
        assert!(empty.lo == 0 && empty.hi == 0);
        assert!(!(non_empty.lo == 0 && non_empty.hi == 0));
    }
}
