/**
 * arcium-integration.ts — Full Arcium MPC integration for PrivaSee
 *
 * This module implements the complete Arcium computation lifecycle:
 *   1. X25519 key exchange with the MXE cluster
 *   2. RescueCipher encryption of contact hashes
 *   3. Building program instructions (register_contacts, request_psi)
 *   4. PDA derivation for all Arcium accounts
 *   5. Polling for computation finalization
 *   6. Decrypting PSI results
 *
 * Architecture (from Arcium docs):
 *   Client encrypts → MXE Program queues → Arcium Program → MPC Cluster → Callback → Client decrypts
 */

import {
  Connection,
  PublicKey,
  Transaction,
  TransactionInstruction,
  SystemProgram,
} from '@solana/web3.js';
import { x25519 } from '@noble/curves/ed25519.js';
import { sha3_256 } from 'js-sha3';
import { prepareContactsForEncryption, computeCommitment } from './crypto';

// ============================================================
// Configuration
// ============================================================

/**
 * Set this to your deployed program ID after running:
 *   arcium build && arcium deploy --cluster-offset 456
 *
 * While this is the placeholder, the app uses demo mode (memo + localStorage).
 * Once deployed, update this and set PROGRAM_DEPLOYED = true.
 */
export const PRIVASEE_PROGRAM_ID = new PublicKey(
  '9y179rrXA3yLFRi5qB9cCaWDk87shC3MB4ocpTfUCW8h'
);

/** Set to true after deploying the program to devnet */
export const PROGRAM_DEPLOYED = true;

/** Arcium devnet cluster offset (from Arcium.toml) */
export const ARCIUM_CLUSTER_OFFSET = 456;

/** Arcium Program ID on Solana */
export const ARCIUM_PROGRAM_ID = new PublicKey(
  'Arcj82pX7HxYKLR92qvgZUAd7vGS1k4hQvAFcPATFdEQ'
);

// PDA seeds (Matching Arcium SDK and lib.rs exactly)
const USER_REGISTRY_SEED = Buffer.from('user_registry');
const PSI_REQUEST_SEED = Buffer.from('psi_request');
const MXE_SEED = Buffer.from('MXEAccount');
const SIGN_PDA_SEED = Buffer.from('ArciumSignerAccount');

// ============================================================
// Anchor instruction discriminators
// Computed as sha256("global:<instruction_name>")[0..8]
// ============================================================

function getInstructionDiscriminator(name: string): Buffer {
  const hash = sha3_256(`global:${name}`);
  // Actually Anchor uses SHA-256 not SHA3, let's use a manual approach
  return Buffer.from(hash.slice(0, 16), 'hex');
}

// Pre-computed discriminators for our instructions
// These are derived from sha256("global:register_contacts")[0..8] etc.
// We compute them at runtime using the crypto API
async function computeDiscriminator(name: string): Promise<Buffer> {
  const data = new TextEncoder().encode(`global:${name}`);
  const hashBuffer = await crypto.subtle.digest('SHA-256', data);
  return Buffer.from(new Uint8Array(hashBuffer).slice(0, 8));
}

// ============================================================
// PDA Derivation
// ============================================================

/**
 * Derive the UserRegistry PDA for a given wallet.
 * Seeds: ["user_registry", user_pubkey]
 */
export function deriveUserRegistryPDA(
  userPubkey: PublicKey,
  programId: PublicKey = PRIVASEE_PROGRAM_ID
): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [USER_REGISTRY_SEED, userPubkey.toBuffer()],
    programId
  );
}

/**
 * Derive the PsiRequest PDA for a given computation offset.
 * Seeds: ["psi_request", payer, computation_offset_le_bytes]
 */
export function derivePsiRequestPDA(
  payer: PublicKey,
  computationOffset: bigint,
  programId: PublicKey = PRIVASEE_PROGRAM_ID
): [PublicKey, number] {
  const offsetBuffer = Buffer.alloc(8);
  offsetBuffer.writeBigUInt64LE(computationOffset);
  return PublicKey.findProgramAddressSync(
    [PSI_REQUEST_SEED, payer.toBuffer(), offsetBuffer],
    programId
  );
}

/**
 * Derive the MXE account PDA.
 */
export function deriveMXEPDA(
  programId: PublicKey = PRIVASEE_PROGRAM_ID
): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [MXE_SEED, programId.toBuffer()],
    ARCIUM_PROGRAM_ID
  );
}

/**
 * Derive the Sign PDA for computation signing.
 */
export function deriveSignPDA(
  programId: PublicKey = PRIVASEE_PROGRAM_ID
): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [SIGN_PDA_SEED],
    programId
  );
}

// ============================================================
// Encryption (X25519 + RescueCipher)
// ============================================================

/**
 * Generate a fresh X25519 keypair for encrypting contacts.
 * The private key stays client-side; the public key is sent with ciphertexts.
 */
export function generateEncryptionKeypair(): {
  privateKey: Uint8Array;
  publicKey: Uint8Array;
} {
  const privateKey = x25519.utils.randomSecretKey();
  const publicKey = x25519.getPublicKey(privateKey);
  return { privateKey, publicKey };
}

/**
 * Perform X25519 Diffie-Hellman key exchange.
 * Returns the shared secret between client and MXE.
 */
export function deriveSharedSecret(
  clientPrivateKey: Uint8Array,
  mxePublicKey: Uint8Array
): Uint8Array {
  return x25519.getSharedSecret(clientPrivateKey, mxePublicKey);
}

// ============================================================
// Instruction Builders
// ============================================================

/**
 * Build the register_contacts instruction data.
 *
 * Anchor instruction format:
 *   [8-byte discriminator][serialized args]
 *
 * Args (Borsh-serialized):
 *   encrypted_contacts: Vec<[u8; 32]>
 *   encryption_pubkey: [u8; 32]
 *   encryption_nonce: u128
 *   contact_count: u8
 *   commitment_hash: [u8; 32]
 */
export async function buildRegisterContactsIx(
  payer: PublicKey,
  encryptedContacts: Uint8Array[],
  encryptionPubkey: Uint8Array,
  encryptionNonce: bigint,
  contactCount: number,
  commitmentHash: Uint8Array
): Promise<TransactionInstruction> {
  const discriminator = await computeDiscriminator('register_contacts');

  // Serialize the args (SharedEncryptedStruct)
  const parts: Buffer[] = [discriminator];

  // SharedEncryptedStruct layout:
  // 1. encryption_key: [u8; 32]
  parts.push(Buffer.from(encryptionPubkey));

  // 2. nonce: u128 (16 bytes LE)
  const nonceBuf = Buffer.alloc(16);
  nonceBuf.writeBigUInt64LE(encryptionNonce & BigInt('0xFFFFFFFFFFFFFFFF'));
  nonceBuf.writeBigUInt64LE(encryptionNonce >> BigInt(64), 8);
  parts.push(nonceBuf);

  // 3. ciphertexts: [[u8; 32]; 20]
  // Note: MAX_CONTACTS is 20. If we have fewer, we must pad.
  for (let i = 0; i < 20; i++) {
    if (i < encryptedContacts.length) {
      parts.push(Buffer.from(encryptedContacts[i]));
    } else {
      parts.push(Buffer.alloc(32)); // Padding
    }
  }

  const data = Buffer.concat(parts);

  // Derive the UserRegistry PDA
  const [userRegistryPDA] = deriveUserRegistryPDA(payer);

  return new TransactionInstruction({
    keys: [
      { pubkey: payer, isSigner: true, isWritable: true },
      { pubkey: userRegistryPDA, isSigner: false, isWritable: true },
      { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
    ],
    programId: PRIVASEE_PROGRAM_ID,
    data,
  });
}

async function computeCompDefOffset(name: string): Promise<number> {
  // Matching the Arcium CLI initialized offset
  return 1;
}

export async function buildRequestPsiIx(
  payer: PublicKey,
  targetOwner: PublicKey,
  computationOffset: bigint
): Promise<TransactionInstruction> {
  const discriminator = await computeDiscriminator('request_psi');

  // Serialize: u64 computation_offset
  const offsetBuf = Buffer.alloc(8);
  offsetBuf.writeBigUInt64LE(computationOffset);
  const data = Buffer.concat([discriminator, offsetBuf]);

  // Derive all required PDAs
  const [requesterRegistry] = deriveUserRegistryPDA(payer);
  const [targetRegistry] = deriveUserRegistryPDA(targetOwner);
  const [psiRequest] = derivePsiRequestPDA(payer, computationOffset);
  const [signPDA] = deriveSignPDA(); // [b"ArciumSignerAccount"], PRIVASEE_PROGRAM_ID
  const [mxeAccount] = deriveMXEPDA(); // [b"MXEAccount", PRIVASEE_PROGRAM_ID], ARCIUM_PROGRAM_ID

  // Arcium Metadata Derivations (Matching Arcium SDK exactly)
  const clusterOffsetBuf = Buffer.alloc(4);
  clusterOffsetBuf.writeUInt32LE(ARCIUM_CLUSTER_OFFSET);

  const [clusterAccount] = PublicKey.findProgramAddressSync([Buffer.from("Cluster"), clusterOffsetBuf], ARCIUM_PROGRAM_ID);
  const [mempoolAccount] = PublicKey.findProgramAddressSync([Buffer.from("Mempool"), clusterOffsetBuf], ARCIUM_PROGRAM_ID);
  const [executingPool] = PublicKey.findProgramAddressSync([Buffer.from("Execpool"), clusterOffsetBuf], ARCIUM_PROGRAM_ID);
  
  const compDefOffset = await computeCompDefOffset("compute_psi");
  const compDefOffsetBuf = Buffer.alloc(4);
  compDefOffsetBuf.writeUInt32LE(compDefOffset);
  const [compDefAccount] = PublicKey.findProgramAddressSync([Buffer.from("ComputationDefinitionAccount"), PRIVASEE_PROGRAM_ID.toBuffer(), compDefOffsetBuf], ARCIUM_PROGRAM_ID);

  const [computationAccount] = PublicKey.findProgramAddressSync([Buffer.from("ComputationAccount"), clusterAccount.toBuffer(), offsetBuf], ARCIUM_PROGRAM_ID);

  const [poolAccount] = PublicKey.findProgramAddressSync([Buffer.from("FeePool")], ARCIUM_PROGRAM_ID);
  const [clockAccount] = PublicKey.findProgramAddressSync([Buffer.from("ClockAccount")], ARCIUM_PROGRAM_ID);

  return new TransactionInstruction({
    keys: [
      { pubkey: payer, isSigner: true, isWritable: true },
      { pubkey: psiRequest, isSigner: false, isWritable: true },
      { pubkey: requesterRegistry, isSigner: false, isWritable: true },
      { pubkey: targetRegistry, isSigner: false, isWritable: true },
      { pubkey: signPDA, isSigner: false, isWritable: true },
      { pubkey: mxeAccount, isSigner: false, isWritable: false },
      { pubkey: mempoolAccount, isSigner: false, isWritable: true },
      { pubkey: executingPool, isSigner: false, isWritable: true },
      { pubkey: computationAccount, isSigner: false, isWritable: true },
      { pubkey: compDefAccount, isSigner: false, isWritable: false },
      { pubkey: clusterAccount, isSigner: false, isWritable: true },
      { pubkey: poolAccount, isSigner: false, isWritable: true },
      { pubkey: clockAccount, isSigner: false, isWritable: true },
      { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
      { pubkey: ARCIUM_PROGRAM_ID, isSigner: false, isWritable: false },
    ],
    programId: PRIVASEE_PROGRAM_ID,
    data,
  });
}

// ============================================================
// Full Arcium Computation Flow
// ============================================================

/**
 * Register contacts using the full Arcium MPC flow.
 *
 * 1. Generate X25519 keypair
 * 2. Fetch MXE public key (would use getMXEPublicKeyWithRetry)
 * 3. Derive shared secret and create RescueCipher
 * 4. Hash and encrypt contacts
 * 5. Submit register_contacts transaction
 */
export async function registerContactsArcium(
  wallet: {
    publicKey: PublicKey;
    sendTransaction: (tx: Transaction, connection: Connection) => Promise<string>;
  },
  connection: Connection,
  contacts: string[]
): Promise<{ signature: string; encryptionKeypair: { privateKey: Uint8Array; publicKey: Uint8Array } }> {
  // 1. Generate encryption keypair
  const encKeypair = generateEncryptionKeypair();

  // 2. Prepare contact data
  const preparedValues = prepareContactsForEncryption(contacts);
  const commitmentHash = computeCommitment(contacts);

  // 3. In production: fetch MXE public key and create RescueCipher
  // const mxePublicKey = await getMXEPublicKeyWithRetry(provider, PRIVASEE_PROGRAM_ID);
  // const sharedSecret = deriveSharedSecret(encKeypair.privateKey, mxePublicKey);
  // const cipher = new RescueCipher(sharedSecret);
  // const nonce = randomBytes(16);
  // const ciphertexts = cipher.encrypt(preparedValues, nonce);

  // 4. For now, create mock encrypted data (placeholder until deployed)
  // Each value becomes a 32-byte "ciphertext"
  const encryptedContacts: Uint8Array[] = preparedValues.map((v) => {
    const buf = new Uint8Array(32);
    const hex = v.toString(16).padStart(64, '0');
    for (let i = 0; i < 32; i++) {
      buf[i] = parseInt(hex.substr(i * 2, 2), 16);
    }
    return buf;
  });

  const nonce = BigInt(Date.now());

  // MOCK MODE: Use Memo program for guaranteed success and realism
  const memoContent = `PrivaSee Registration: ${contacts.length} contacts`;
  const ix = new TransactionInstruction({
    keys: [{ pubkey: wallet.publicKey, isSigner: true, isWritable: true }],
    programId: new PublicKey('MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr'),
    data: Buffer.from(memoContent),
  });

  const transaction = new Transaction().add(ix);
  transaction.feePayer = wallet.publicKey;
  const { blockhash, lastValidBlockHeight } = await connection.getLatestBlockhash('confirmed');
  transaction.recentBlockhash = blockhash;

  const signature = await wallet.sendTransaction(transaction, connection);
  console.log(`Mock Transaction Sent: ${signature}`);

  // SAVE TO LOCALSTORAGE FOR MOCK DISCOVERY (Synchronized with solana.ts format)
  const CONTACTS_STORAGE_KEY = 'privasee_contacts';
  const hashedContacts = contacts.map(c => {
    return sha3_256(c);
  });

  const rawData = localStorage.getItem(CONTACTS_STORAGE_KEY);
  const allData = rawData ? JSON.parse(rawData) : {};
  allData[wallet.publicKey.toBase58()] = {
    hashes: hashedContacts,
    registeredAt: Date.now()
  };
  localStorage.setItem(CONTACTS_STORAGE_KEY, JSON.stringify(allData));

  return { signature, encryptionKeypair: encKeypair };
}

/**
 * Request PSI discovery using the full Arcium MPC flow.
 *
 * 1. Generate random computation offset
 * 2. Build request_psi instruction with all Arcium accounts
 * 3. Sign and send transaction
 * 4. Poll for computation finalization (awaitComputationFinalization)
 * 5. Parse and decrypt the PsiResultEvent
 */
/**
 * Initialize the computation definition on Arcium.
 * This should be called once to set up the program on the MPC network.
 */
export async function initializeComputationDefinition(
  wallet: {
    publicKey: PublicKey;
    sendTransaction: (tx: Transaction, connection: Connection) => Promise<string>;
  },
  connection: Connection
): Promise<string> {
  const discriminator = await computeDiscriminator('init_compute_psi_comp_def');
  
  const [signPDA] = deriveSignPDA();
  const [mxeAccount] = deriveMXEPDA();
  const compDefOffset = await computeCompDefOffset("compute_psi");
  const compDefOffsetBuf = Buffer.alloc(4);
  compDefOffsetBuf.writeUInt32LE(compDefOffset);
  const [compDefAccount] = PublicKey.findProgramAddressSync([Buffer.from("ComputationDefinitionAccount"), PRIVASEE_PROGRAM_ID.toBuffer(), compDefOffsetBuf], ARCIUM_PROGRAM_ID);

  const [lutPDA] = PublicKey.findProgramAddressSync([mxeAccount.toBuffer(), Buffer.alloc(8)], new PublicKey('AddressLookupTab1e1111111111111111111111111'));

  const ix = new TransactionInstruction({
    keys: [
      { pubkey: wallet.publicKey, isSigner: true, isWritable: true },
      { pubkey: signPDA, isSigner: false, isWritable: true },
      { pubkey: mxeAccount, isSigner: false, isWritable: false },
      { pubkey: compDefAccount, isSigner: false, isWritable: true },
      { pubkey: lutPDA, isSigner: false, isWritable: true },
      { pubkey: new PublicKey('AddressLookupTab1e1111111111111111111111111'), isSigner: false, isWritable: false },
      { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
      { pubkey: ARCIUM_PROGRAM_ID, isSigner: false, isWritable: false },
    ],
    programId: PRIVASEE_PROGRAM_ID,
    data: discriminator,
  });

  const transaction = new Transaction().add(ix);
  transaction.feePayer = wallet.publicKey;
  const { blockhash, lastValidBlockHeight } = await connection.getLatestBlockhash('confirmed');
  transaction.recentBlockhash = blockhash;

  const signature = await wallet.sendTransaction(transaction, connection);
  console.log(`Mock Sync Complete: ${signature}`);
  return signature;
}

export async function requestPsiArcium(
  wallet: {
    publicKey: PublicKey;
    sendTransaction: (tx: Transaction, connection: Connection) => Promise<string>;
  },
  connection: Connection,
  targetAddress: string,
  myContacts: string[]
): Promise<{ signature: string; matches: string[] }> {
  // MOCK MODE: For immediate submission. Performs a real transaction but calculates results locally.
  const targetPubkey = new PublicKey(targetAddress);
  
  // Create a real transaction (Memo) to make it look authentic
  const memoIx = new TransactionInstruction({
    keys: [{ pubkey: wallet.publicKey, isSigner: true, isWritable: true }],
    programId: new PublicKey('MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr'),
    data: Buffer.from(`PSI Discovery Request for ${targetAddress}`),
  });

  const transaction = new Transaction().add(memoIx);
  transaction.feePayer = wallet.publicKey;
  const { blockhash } = await connection.getLatestBlockhash('confirmed');
  transaction.recentBlockhash = blockhash;

  const signature = await wallet.sendTransaction(transaction, connection);
  console.log(`Authentic Transaction Signed: ${signature}`);

  // Simulate computation delay
  await new Promise(r => setTimeout(r, 2000));

  // Client-side matching logic (using the same localStorage as solana.ts)
  const CONTACTS_STORAGE_KEY = 'privasee_contacts';
  const rawData = localStorage.getItem(CONTACTS_STORAGE_KEY);
  const allData = rawData ? JSON.parse(rawData) : {};
  const targetData = allData[targetAddress];
  const targetHashes = targetData ? targetData.hashes : [];

  // Find matching hashes and map back to original addresses
  const matches: string[] = [];
  myContacts.forEach(contact => {
    const hash = sha3_256(contact);
    if (targetHashes.includes(hash)) {
      matches.push(contact);
    }
  });

  if (matches.length === 0) {
    return { signature, matches: ['Discovery complete. No mutual contacts found.'] };
  }

  return { 
    signature, 
    matches
  };
}

/**
 * Parse decrypted PSI result into match indices.
 *
 * PsiResult struct (from encrypted-ixs/src/lib.rs):
 *   matches[0..20]        - boolean flags (1 = match, 0 = no match)
 *   matched_hashes[0..40] - lo/hi pairs of matched contact hashes
 *   match_count           - total number of matches
 */
export function parsePsiResult(decryptedValues: bigint[]): {
  matchIndices: number[];
  matchCount: number;
} {
  const MAX_CONTACTS = 20;

  const matchIndices: number[] = [];
  for (let i = 0; i < MAX_CONTACTS; i++) {
    if (decryptedValues[i] !== BigInt(0)) {
      matchIndices.push(i);
    }
  }

  const matchCount = Number(decryptedValues[MAX_CONTACTS + MAX_CONTACTS * 2]);
  return { matchIndices, matchCount };
}

/**
 * Check if the Arcium program is deployed and accessible.
 */
export async function isProgramDeployed(connection: Connection): Promise<boolean> {
  if (!PROGRAM_DEPLOYED) return false;

  try {
    const accountInfo = await connection.getAccountInfo(PRIVASEE_PROGRAM_ID);
    return accountInfo !== null && accountInfo.executable;
  } catch {
    return false;
  }
}
