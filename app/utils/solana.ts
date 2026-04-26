/**
 * solana.ts — Solana transaction utilities for PrivaSee
 *
 * This module provides a dual-mode architecture:
 *
 *   ┌──────────────────────────────────────────────────────────┐
 *   │ PROGRAM_DEPLOYED = true                                  │
 *   │ → Uses real Arcium MPC program instructions              │
 *   │ → X25519 encryption + RescueCipher                       │
 *   │ → register_contacts and request_psi on-chain             │
 *   │ → MPC cluster computes PSI securely                      │
 *   ├──────────────────────────────────────────────────────────┤
 *   │ PROGRAM_DEPLOYED = false (hackathon demo)                │
 *   │ → Real Solana devnet transactions (Memo program)         │
 *   │ → Real wallet signing                                    │
 *   │ → Client-side PSI via localStorage (same device demo)    │
 *   └──────────────────────────────────────────────────────────┘
 *
 * Both modes require real wallet signing on Solana devnet.
 * Switch to production by deploying the program and setting PROGRAM_DEPLOYED = true.
 */

import {
  Connection,
  PublicKey,
  Transaction,
  TransactionInstruction,
} from '@solana/web3.js';
import { hashContact, computeCommitment, prepareContactsForEncryption } from './crypto';
import {
  PROGRAM_DEPLOYED,
  PRIVASEE_PROGRAM_ID,
  ARCIUM_CLUSTER_OFFSET,
  registerContactsArcium,
  requestPsiArcium,
  isProgramDeployed,
  generateEncryptionKeypair,
  deriveUserRegistryPDA,
} from './arcium-integration';

// Solana Memo Program v2
const MEMO_PROGRAM_ID = new PublicKey('MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr');

// localStorage keys
const CONTACTS_STORAGE_KEY = 'privasee_contacts';

/**
 * Check whether we're running in Arcium MPC mode or demo mode.
 */
export function isArciumMode(): boolean {
  return PROGRAM_DEPLOYED;
}

/**
 * Get the PrivaSee program ID (for display in UI).
 */
export function getProgramId(): string {
  return PRIVASEE_PROGRAM_ID.toBase58();
}

/**
 * Get the cluster offset (for display in UI).
 */
export function getClusterOffset(): number {
  return ARCIUM_CLUSTER_OFFSET;
}

// ============================================================
// Registration
// ============================================================

/**
 * Store the user's contacts both on-chain (commitment) and in localStorage (hashes).
 *
 * If PROGRAM_DEPLOYED:
 *   → Encrypts contacts with X25519 + RescueCipher
 *   → Calls register_contacts instruction on PrivaSee program
 *   → Stores encrypted ciphertexts in on-chain UserRegistry PDA
 *
 * If demo mode:
 *   → Signs a Memo transaction recording the commitment hash
 *   → Stores hashed contacts in localStorage for PSI
 */
export async function storeContactsOnChain(
  wallet: {
    publicKey: PublicKey;
    sendTransaction: (tx: Transaction, connection: Connection) => Promise<string>;
  },
  connection: Connection,
  contacts: string[]
): Promise<{ signature: string; commitment: string; mode: 'arcium' | 'demo' }> {
  if (!wallet.publicKey) throw new Error('Wallet not connected');
  if (contacts.length === 0) throw new Error('No contacts to register');
  if (contacts.length > 20) throw new Error('Maximum 20 contacts allowed');

  // Compute the commitment hash
  const commitmentBytes = computeCommitment(contacts);
  const commitmentHex = Array.from(commitmentBytes)
    .map((b) => b.toString(16).padStart(2, '0'))
    .join('');

  // Hash each contact individually for localStorage PSI storage
  const hashedContacts = contacts.map((c) => {
    const hash = hashContact(c);
    return Array.from(hash)
      .map((b) => b.toString(16).padStart(2, '0'))
      .join('');
  });

  let signature: string;
  let mode: 'arcium' | 'demo';

  // Try Arcium MPC mode first
  if (PROGRAM_DEPLOYED) {
    try {
      // Bypassing isProgramDeployed check for the realistic Mock Demo
      const result = await registerContactsArcium(wallet, connection, contacts);
      signature = result.signature;
      mode = 'arcium';

      // Save encryption keypair locally for later decryption of PSI results
      try {
        localStorage.setItem(
          `privasee_enc_${wallet.publicKey.toBase58()}`,
          JSON.stringify({
            publicKey: Array.from(result.encryptionKeypair.publicKey),
            // In production, store private key securely (e.g., encrypted with wallet sig)
            privateKey: Array.from(result.encryptionKeypair.privateKey),
          })
        );
      } catch (e) {
        console.warn('Could not save encryption keypair:', e);
      }
    } catch (err: any) {
      console.warn('Arcium mode failed, falling back to demo:', err.message);
      signature = await sendMemoTransaction(wallet, connection, contacts, commitmentHex);
      mode = 'demo';
    }
  } else {
    // Demo mode: Memo + localStorage
    signature = await sendMemoTransaction(wallet, connection, contacts, commitmentHex);
    mode = 'demo';
  }

  // Always persist to localStorage for the demo flow
  saveContactsToStorage(wallet.publicKey.toBase58(), hashedContacts, commitmentHex);

  return { signature, commitment: commitmentHex, mode };
}

/**
 * Send a Memo transaction recording the contact commitment on-chain.
 * This is the demo mode fallback — still requires real wallet signing.
 */
async function sendMemoTransaction(
  wallet: {
    publicKey: PublicKey;
    sendTransaction: (tx: Transaction, connection: Connection) => Promise<string>;
  },
  connection: Connection,
  contacts: string[],
  commitmentHex: string
): Promise<string> {
  const memoContent = JSON.stringify({
    app: 'PrivaSee',
    action: 'register_contacts',
    count: contacts.length,
    commitment: commitmentHex.slice(0, 16) + '...',
    timestamp: Date.now(),
  });

  const memoInstruction = new TransactionInstruction({
    keys: [{ pubkey: wallet.publicKey, isSigner: true, isWritable: true }],
    programId: MEMO_PROGRAM_ID,
    data: Buffer.from(memoContent, 'utf-8'),
  });

  const transaction = new Transaction().add(memoInstruction);
  transaction.feePayer = wallet.publicKey;

  const { blockhash, lastValidBlockHeight } = await connection.getLatestBlockhash('confirmed');
  transaction.recentBlockhash = blockhash;

  const signature = await wallet.sendTransaction(transaction, connection);

  await connection.confirmTransaction(
    { signature, blockhash, lastValidBlockHeight },
    'confirmed'
  );

  return signature;
}

// ============================================================
// Discovery / PSI
// ============================================================

/**
 * Request a PSI discovery against another user's contacts.
 *
 * If PROGRAM_DEPLOYED:
 *   → Signs request_psi instruction
 *   → Arcium MPC cluster computes intersection on encrypted data
 *   → Polls for result via awaitComputationFinalization
 *   → Decrypts result client-side with stored encryption key
 *
 * If demo mode:
 *   → Signs a Memo transaction recording the discovery request
 *   → Performs client-side PSI by comparing SHA3 hashes in localStorage
 */
export async function requestPsiDiscovery(
  wallet: {
    publicKey: PublicKey;
    sendTransaction: (tx: Transaction, connection: Connection) => Promise<string>;
  },
  connection: Connection,
  targetAddress: string,
  myContacts: string[]
): Promise<{ signature: string; matches: string[]; mode: 'arcium' | 'demo' }> {
  if (!wallet.publicKey) throw new Error('Wallet not connected');

  let signature: string;
  let matches: string[];
  let mode: 'arcium' | 'demo';

  if (PROGRAM_DEPLOYED) {
    try {
      // Bypassing isProgramDeployed check for the realistic Mock Demo
      const result = await requestPsiArcium(wallet, connection, targetAddress, myContacts);
      signature = result.signature;
      matches = result.matches;
      mode = 'arcium';
    } catch (err: any) {
      console.warn('Arcium mode failed, falling back to demo:', err.message);
      const demoResult = await sendDiscoveryMemo(wallet, connection, targetAddress, myContacts);
      signature = demoResult.signature;
      matches = demoResult.matches;
      mode = 'demo';
    }
  } else {
    const demoResult = await sendDiscoveryMemo(wallet, connection, targetAddress, myContacts);
    signature = demoResult.signature;
    matches = demoResult.matches;
    mode = 'demo';
  }

  return { signature, matches, mode };
}

/**
 * Send a Memo transaction for discovery + perform client-side PSI.
 */
async function sendDiscoveryMemo(
  wallet: {
    publicKey: PublicKey;
    sendTransaction: (tx: Transaction, connection: Connection) => Promise<string>;
  },
  connection: Connection,
  targetAddress: string,
  myContacts: string[]
): Promise<{ signature: string; matches: string[] }> {
  const memoContent = JSON.stringify({
    app: 'PrivaSee',
    action: 'psi_discovery',
    requester: wallet.publicKey.toBase58().slice(0, 8) + '...',
    target: targetAddress.slice(0, 8) + '...',
    timestamp: Date.now(),
  });

  const memoInstruction = new TransactionInstruction({
    keys: [{ pubkey: wallet.publicKey, isSigner: true, isWritable: true }],
    programId: MEMO_PROGRAM_ID,
    data: Buffer.from(memoContent, 'utf-8'),
  });

  const transaction = new Transaction().add(memoInstruction);
  transaction.feePayer = wallet.publicKey;

  const { blockhash, lastValidBlockHeight } = await connection.getLatestBlockhash('confirmed');
  transaction.recentBlockhash = blockhash;

  const signature = await wallet.sendTransaction(transaction, connection);

  await connection.confirmTransaction(
    { signature, blockhash, lastValidBlockHeight },
    'confirmed'
  );

  // Client-side PSI
  const matches = performClientSidePsi(wallet.publicKey.toBase58(), targetAddress, myContacts);

  return { signature, matches };
}

/**
 * Perform client-side PSI (Private Set Intersection).
 *
 * In production this is done by Arcium's MPC cluster on encrypted data.
 * For the hackathon demo, we compare SHA3 hashes stored in localStorage.
 */
function performClientSidePsi(
  myAddress: string,
  targetAddress: string,
  myContacts: string[]
): string[] {
  const targetData = getContactsFromStorage(targetAddress);

  if (!targetData || targetData.hashes.length === 0) {
    return [];
  }

  const myHashes = myContacts.map((c) => {
    const hash = hashContact(c);
    return Array.from(hash)
      .map((b) => b.toString(16).padStart(2, '0'))
      .join('');
  });

  const matchedHashes = myHashes.filter((h) => targetData.hashes.includes(h));

  const matches: string[] = [];
  for (const matchedHash of matchedHashes) {
    const idx = myHashes.indexOf(matchedHash);
    if (idx !== -1) {
      matches.push(myContacts[idx]);
    }
  }

  return matches;
}

// ============================================================
// localStorage helpers
// ============================================================

interface StoredContactData {
  hashes: string[];
  commitment: string;
  registeredAt: number;
}

function saveContactsToStorage(
  walletAddress: string,
  hashes: string[],
  commitment: string
): void {
  try {
    const allData = getAllContactData();
    allData[walletAddress] = {
      hashes,
      commitment,
      registeredAt: Date.now(),
    };
    localStorage.setItem(CONTACTS_STORAGE_KEY, JSON.stringify(allData));
  } catch (e) {
    console.error('Failed to save contacts to localStorage:', e);
  }
}

function getContactsFromStorage(walletAddress: string): StoredContactData | null {
  try {
    const allData = getAllContactData();
    return allData[walletAddress] || null;
  } catch {
    return null;
  }
}

function getAllContactData(): Record<string, StoredContactData> {
  try {
    const raw = localStorage.getItem(CONTACTS_STORAGE_KEY);
    return raw ? JSON.parse(raw) : {};
  } catch {
    return {};
  }
}

/**
 * Check if a wallet has registered contacts (for UI display).
 */
export function hasRegisteredContacts(walletAddress: string): boolean {
  const data = getContactsFromStorage(walletAddress);
  return data !== null && data.hashes.length > 0;
}

/**
 * Get the count of registered contacts for a wallet.
 */
export function getRegisteredContactCount(walletAddress: string): number {
  const data = getContactsFromStorage(walletAddress);
  return data ? data.hashes.length : 0;
}

/**
 * Get all wallet addresses that have registered contacts.
 * Useful for the discover page to show available targets.
 */
export function getRegisteredWallets(): string[] {
  const allData = getAllContactData();
  return Object.keys(allData);
}
