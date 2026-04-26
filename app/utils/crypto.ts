/**
 * crypto.ts — Client-side hashing utilities for PrivaSee
 *
 * All contact hashing happens HERE, in the user's browser, BEFORE any
 * encryption or network communication. This ensures raw contact addresses
 * never leave the client.
 *
 * We use SHA3-256 (Keccak) because:
 *   1. It matches the SHA3 available inside Arcis circuits
 *   2. It's a well-established cryptographic hash function
 *   3. It produces fixed 32-byte output suitable for MPC comparison
 */

import { sha3_256 } from 'js-sha3';

/**
 * Hash a single contact address using SHA3-256.
 * Returns the hash as a Uint8Array (32 bytes).
 */
export function hashContact(address: string): Uint8Array {
  const normalized = address.trim().toLowerCase();
  const hashHex = sha3_256(normalized);
  return hexToBytes(hashHex);
}

/**
 * Hash a list of contacts, returning an array of 32-byte hashes.
 */
export function hashContacts(addresses: string[]): Uint8Array[] {
  return addresses.map(hashContact);
}

/**
 * Split a 32-byte hash into two u128 halves (lo, hi) as BigInts.
 * This matches the ContactHash struct in the Arcis circuit:
 *   struct ContactHash { lo: u128, hi: u128 }
 *
 * The split is little-endian: lo = bytes[0..16], hi = bytes[16..32]
 */
export function splitHash(hash: Uint8Array): { lo: bigint; hi: bigint } {
  if (hash.length !== 32) {
    throw new Error(`Expected 32-byte hash, got ${hash.length} bytes`);
  }

  let lo = BigInt(0);
  let hi = BigInt(0);

  // Little-endian: first byte is least significant
  for (let i = 15; i >= 0; i--) {
    lo = (lo << BigInt(8)) | BigInt(hash[i]);
  }
  for (let i = 31; i >= 16; i--) {
    hi = (hi << BigInt(8)) | BigInt(hash[i]);
  }

  return { lo, hi };
}

/**
 * Prepare contacts for encryption.
 * Returns a flat array of BigInts ready to be encrypted with RescueCipher:
 *   [contact_0_lo, contact_0_hi, contact_1_lo, contact_1_hi, ..., count]
 *
 * Empty slots are padded with zeros up to MAX_CONTACTS (20).
 */
export function prepareContactsForEncryption(
  addresses: string[],
  maxContacts: number = 20
): bigint[] {
  const hashes = hashContacts(addresses);
  const values: bigint[] = [];

  // Add contact hashes (pad with zeros for empty slots)
  for (let i = 0; i < maxContacts; i++) {
    if (i < hashes.length) {
      const { lo, hi } = splitHash(hashes[i]);
      values.push(lo, hi);
    } else {
      values.push(BigInt(0), BigInt(0)); // Empty slot
    }
  }

  // Add count
  values.push(BigInt(hashes.length));

  return values;
}

/**
 * Compute a commitment hash for the entire contact set.
 * This is stored on-chain as a compact proof that the user registered,
 * without revealing any contact information.
 */
export function computeCommitment(addresses: string[]): Uint8Array {
  const sorted = [...addresses].map((a) => a.trim().toLowerCase()).sort();
  const combined = sorted.join('|');
  return hexToBytes(sha3_256(combined));
}

// ---- Helpers ----

function hexToBytes(hex: string): Uint8Array {
  const bytes = new Uint8Array(hex.length / 2);
  for (let i = 0; i < hex.length; i += 2) {
    bytes[i / 2] = parseInt(hex.substring(i, i + 2), 16);
  }
  return bytes;
}
