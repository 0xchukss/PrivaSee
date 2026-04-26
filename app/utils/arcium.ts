/**
 * arcium.ts — Arcium SDK helpers for PrivaSee
 *
 * This module wraps the Arcium TypeScript client library to provide
 * high-level functions for:
 *   1. X25519 key exchange with the MXE cluster
 *   2. Encrypting contact data using RescueCipher
 *   3. Queuing PSI computations
 *   4. Polling for computation results
 *   5. Decrypting results client-side
 *
 * The encryption flow:
 *   Client keypair → Key exchange with MXE → Shared secret → RescueCipher → Encrypt contacts
 *
 * The MXE and Arcium MPC nodes NEVER see plaintext contacts.
 */

import {
  getComputationAccAddress,
  getClusterAccAddress,
  getMXEAccAddress,
  getMempoolAccAddress,
  getExecutingPoolAccAddress,
  getCompDefAccAddress,
  getCompDefAccOffset,
  getMXEPublicKeyWithRetry,
  awaitComputationFinalization,
  RescueCipher,
  deserializeLE,
} from '@arcium-hq/client';
import { x25519 } from '@noble/curves/ed25519';
import { randomBytes } from 'crypto';
import * as anchor from '@coral-xyz/anchor';
import { PublicKey, Connection } from '@solana/web3.js';

// Arcium devnet cluster offset (from Arcium.toml)
export const ARCIUM_CLUSTER_OFFSET = 456;

// Program ID for PrivaSee
export const PRIVASEE_PROGRAM_ID = new PublicKey(
  'PrvSee1111111111111111111111111111111111111'
);

/**
 * Generate a fresh X25519 keypair for encryption.
 * The private key stays client-side; the public key is sent with the ciphertext
 * so the MXE can derive the shared secret for decryption inside MPC.
 */
export function generateEncryptionKeypair() {
  const privateKey = x25519.utils.randomPrivateKey();
  const publicKey = x25519.getPublicKey(privateKey);
  return { privateKey, publicKey };
}

/**
 * Fetch the MXE's X25519 public key.
 * This is needed for the key exchange that produces the shared secret.
 */
export async function fetchMXEPublicKey(
  provider: anchor.AnchorProvider,
  programId: PublicKey = PRIVASEE_PROGRAM_ID
): Promise<Uint8Array> {
  return getMXEPublicKeyWithRetry(provider, programId);
}

/**
 * Create a RescueCipher from a key exchange between the client and MXE.
 *
 * The shared secret is derived via X25519 Diffie-Hellman, then the Rescue
 * key is derived by hashing with Rescue-Prime (as per Arcium's encryption spec).
 */
export function createCipher(
  clientPrivateKey: Uint8Array,
  mxePublicKey: Uint8Array
): RescueCipher {
  const sharedSecret = x25519.getSharedSecret(clientPrivateKey, mxePublicKey);
  return new RescueCipher(sharedSecret);
}

/**
 * Encrypt a set of prepared contact values using RescueCipher.
 *
 * @param values - Flat array of BigInts from prepareContactsForEncryption()
 * @param cipher - RescueCipher instance from createCipher()
 * @returns Object with ciphertexts and nonce for submission to the program
 */
export function encryptContacts(
  values: bigint[],
  cipher: RescueCipher
): {
  ciphertexts: Uint8Array[];
  nonce: Uint8Array;
} {
  // Generate a random 16-byte nonce for CTR mode encryption
  const nonce = randomBytes(16);

  // Encrypt all values in one batch
  const ciphertexts = cipher.encrypt(values, nonce);

  return { ciphertexts, nonce };
}

/**
 * Decrypt PSI result ciphertexts using the client's cipher.
 *
 * @param ciphertexts - Encrypted result from the callback event
 * @param nonce - Nonce from the callback event
 * @param cipher - The same RescueCipher used for encryption
 * @returns Array of decrypted BigInt values
 */
export function decryptResults(
  ciphertexts: Uint8Array[],
  nonce: Uint8Array,
  cipher: RescueCipher
): bigint[] {
  return cipher.decrypt(ciphertexts, nonce);
}

/**
 * Derive all the account addresses needed for a PSI computation.
 * These PDAs are deterministic and derived from the program ID and offsets.
 */
export function deriveComputationAccounts(
  computationOffset: anchor.BN,
  programId: PublicKey = PRIVASEE_PROGRAM_ID
) {
  const compDefOffsetBuffer = getCompDefAccOffset('compute_psi');
  const compDefOffset = Buffer.from(compDefOffsetBuffer).readUInt32LE();

  return {
    computationAccount: getComputationAccAddress(
      ARCIUM_CLUSTER_OFFSET,
      computationOffset
    ),
    clusterAccount: getClusterAccAddress(ARCIUM_CLUSTER_OFFSET),
    mxeAccount: getMXEAccAddress(programId),
    mempoolAccount: getMempoolAccAddress(ARCIUM_CLUSTER_OFFSET),
    executingPool: getExecutingPoolAccAddress(ARCIUM_CLUSTER_OFFSET),
    compDefAccount: getCompDefAccAddress(programId, compDefOffset),
  };
}

/**
 * Wait for a PSI computation to complete.
 *
 * Polls the Arcium network until the MPC cluster finishes processing
 * and invokes the callback instruction. Returns the finalization
 * transaction signature.
 */
export async function waitForPsiResult(
  provider: anchor.AnchorProvider,
  computationOffset: anchor.BN,
  programId: PublicKey = PRIVASEE_PROGRAM_ID
): Promise<string> {
  return awaitComputationFinalization(
    provider,
    computationOffset,
    programId,
    'confirmed'
  );
}

/**
 * Parse the decrypted PSI result into a list of matching contact indices.
 *
 * The PsiResult struct layout (41 values per contact set):
 *   matches[0..20]        - boolean flags (1 = match, 0 = no match)
 *   matched_hashes[0..40] - lo/hi pairs of matched contact hashes
 *   match_count           - total number of matches
 */
export function parsePsiResult(decryptedValues: bigint[]): {
  matchIndices: number[];
  matchCount: number;
} {
  const MAX_CONTACTS = 20;

  // First 20 values are the boolean match flags
  const matchIndices: number[] = [];
  for (let i = 0; i < MAX_CONTACTS; i++) {
    if (decryptedValues[i] !== BigInt(0)) {
      matchIndices.push(i);
    }
  }

  // Last value is the match count
  const matchCount = Number(decryptedValues[MAX_CONTACTS + MAX_CONTACTS * 2]);

  return { matchIndices, matchCount };
}
