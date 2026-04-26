'use client';

import React, { useState, useCallback, useEffect } from 'react';
import { useWallet, useConnection } from '@solana/wallet-adapter-react';
import { WalletMultiButton } from '@solana/wallet-adapter-react-ui';
import { ContactInput } from '../components/ContactInput';
import { PrivacyShield } from '../components/PrivacyShield';
import { prepareContactsForEncryption, computeCommitment } from '../utils/crypto';
import {
  storeContactsOnChain,
  hasRegisteredContacts,
  getRegisteredContactCount,
  isArciumMode,
} from '../utils/solana';

type RegistrationStatus = 'idle' | 'encrypting' | 'submitting' | 'complete' | 'error';

export default function RegisterPage() {
  const { connected, publicKey, sendTransaction } = useWallet();
  const { connection } = useConnection();
  const [status, setStatus] = useState<RegistrationStatus>('idle');
  const [txSignature, setTxSignature] = useState<string | null>(null);
  const [errorMessage, setErrorMessage] = useState<string | null>(null);
  const [registeredCount, setRegisteredCount] = useState<number>(0);
  const [alreadyRegistered, setAlreadyRegistered] = useState(false);
  const [existingCount, setExistingCount] = useState(0);
  const [resultMode, setResultMode] = useState<'arcium' | 'demo'>('demo');

  // Check if user already has registered contacts
  useEffect(() => {
    if (publicKey) {
      const addr = publicKey.toBase58();
      setAlreadyRegistered(hasRegisteredContacts(addr));
      setExistingCount(getRegisteredContactCount(addr));
    }
  }, [publicKey, status]);

  const handleSubmit = useCallback(async (contacts: string[]) => {
    if (!connected || !publicKey) return;
    try {
      setStatus('encrypting');
      setErrorMessage(null);

      // Step 1: Hash and prepare contacts client-side
      const preparedValues = prepareContactsForEncryption(contacts);
      const commitment = computeCommitment(contacts);
      console.log(`Prepared ${contacts.length} contacts (${preparedValues.length} fields)`);
      console.log(`Commitment: ${Array.from(commitment).map(b => b.toString(16).padStart(2, '0')).join('').slice(0, 16)}...`);

      // Step 2: Sign and send real Solana transaction
      setStatus('submitting');

      const result = await storeContactsOnChain(
        { publicKey, sendTransaction },
        connection,
        contacts
      );

      setTxSignature(result.signature);
      setRegisteredCount(contacts.length);
      setResultMode(result.mode);
      setStatus('complete');
      console.log(`✅ Contacts registered on-chain! Mode: ${result.mode}, Tx: ${result.signature}`);
    } catch (err: any) {
      console.error('Registration failed:', err);
      let msg = 'Registration failed';
      if (err?.message?.includes('User rejected')) {
        msg = 'Transaction was rejected by your wallet';
      } else if (err?.message?.includes('insufficient')) {
        msg = 'Insufficient SOL for transaction fees. Get devnet SOL from a faucet.';
      } else if (err instanceof Error) {
        msg = err.message;
      }
      setErrorMessage(msg);
      setStatus('error');
    }
  }, [connected, publicKey, sendTransaction, connection]);

  if (!connected) {
    return (
      <div className="page-container py-20">
        <div className="max-w-md mx-auto text-center">
          <div className="text-5xl mb-6">🔗</div>
          <h1 className="text-2xl font-bold mb-3">Connect Your Wallet</h1>
          <p className="text-white/40 mb-8">Connect your Solana wallet to register contacts privately on devnet.</p>
          <WalletMultiButton className="!bg-pv-600 !rounded-xl !h-12 !font-semibold hover:!bg-pv-500 !transition-all !mx-auto" />
          <p className="text-xs text-white/25 mt-4">Make sure your wallet is set to <span className="text-teal-400">Solana Devnet</span></p>
        </div>
      </div>
    );
  }

  return (
    <div className="page-container py-12 md:py-20">
      <div className="max-w-2xl mx-auto">
        <div className="text-center mb-10">
          <h1 className="text-3xl md:text-4xl font-bold mb-3">
            Register Your <span className="gradient-text-bright">Contacts</span>
          </h1>
          <p className="text-white/40 max-w-md mx-auto">
            Add up to 20 wallet addresses. They will be hashed and encrypted locally, then signed on Solana devnet.
          </p>
          {/* Mode indicator */}
          <div className="mt-3 flex items-center justify-center gap-2">
            <span className={`inline-flex items-center gap-1.5 px-3 py-1 rounded-full text-xs font-medium ${
              isArciumMode()
                ? 'bg-teal-500/10 border border-teal-500/30 text-teal-400'
                : 'bg-purple-500/10 border border-purple-500/30 text-purple-400'
            }`}>
              <span className={`w-1.5 h-1.5 rounded-full ${isArciumMode() ? 'bg-teal-400 animate-pulse' : 'bg-purple-400'}`} />
              {isArciumMode() ? 'Arcium MPC Mode' : 'Demo Mode (Memo + PSI)'}
            </span>
          </div>
        </div>

        {/* Already registered notice */}
        {alreadyRegistered && status === 'idle' && (
          <div className="mb-6 glass-card p-4 flex items-center gap-3 border-teal-500/20">
            <span className="text-teal-400">ℹ️</span>
            <div>
              <span className="text-sm text-teal-300">You have {existingCount} contacts already registered.</span>
              <span className="text-xs text-white/30 ml-2">Registering again will replace them.</span>
            </div>
          </div>
        )}

        <div className="mb-8">
          <PrivacyShield
            isComputing={status === 'encrypting' || status === 'submitting'}
            status={status === 'encrypting' ? 'encrypting' : status === 'submitting' ? 'computing' : status === 'complete' ? 'complete' : status === 'error' ? 'error' : 'idle'}
            message={status === 'encrypting' ? 'Hashing with SHA3-256 and preparing transaction...' : status === 'submitting' ? 'Sign the transaction in your wallet...' : status === 'complete' ? `${registeredCount} contacts registered on-chain` : undefined}
          />
        </div>
        {status === 'complete' ? (
          <div className="glass-card p-8 text-center animate-fade-up">
            <div className="text-5xl mb-4">🎉</div>
            <h2 className="text-xl font-bold mb-2">Contacts Registered On-Chain!</h2>
            <p className="text-white/40 mb-2">{registeredCount} contacts encrypted locally and commitment stored on Solana devnet.</p>
            {/* Show mode badge */}
            <div className="mb-4">
              <span className={`inline-flex items-center gap-1.5 px-2.5 py-1 rounded-full text-xs ${
                resultMode === 'arcium'
                  ? 'bg-teal-500/15 text-teal-400'
                  : 'bg-purple-500/15 text-purple-400'
              }`}>
                {resultMode === 'arcium' ? '⚡ Encrypted via Arcium MPC' : '📝 Recorded via Memo Program'}
              </span>
            </div>
            {txSignature && (
              <div className="mb-6 p-3 rounded-lg bg-surface-900/60">
                <span className="text-xs text-white/30">Transaction: </span>
                <a
                  href={`https://explorer.solana.com/tx/${txSignature}?cluster=devnet`}
                  target="_blank"
                  rel="noopener noreferrer"
                  className="tx-signature"
                >
                  {txSignature.slice(0, 32)}...
                </a>
                <div className="mt-1">
                  <a
                    href={`https://explorer.solana.com/tx/${txSignature}?cluster=devnet`}
                    target="_blank"
                    rel="noopener noreferrer"
                    className="text-xs text-teal-400 hover:text-teal-300"
                  >
                    View on Solana Explorer ↗
                  </a>
                </div>
              </div>
            )}
            <div className="flex gap-3 justify-center">
              <a href="/discover" className="btn-primary">Discover Matches →</a>
              <button onClick={() => { setStatus('idle'); setTxSignature(null); }} className="btn-secondary">Register More</button>
            </div>
          </div>
        ) : (
          <div className="glass-card p-6 md:p-8">
            <ContactInput onSubmit={handleSubmit} isSubmitting={status === 'encrypting' || status === 'submitting'} />
          </div>
        )}
        {status === 'error' && errorMessage && (
          <div className="mt-4 p-4 rounded-xl bg-red-500/10 border border-red-500/20 text-red-400 text-sm">
            <strong>Error:</strong> {errorMessage}
            {errorMessage.includes('SOL') && (
              <div className="mt-2">
                <a
                  href="https://faucet.solana.com/"
                  target="_blank"
                  rel="noopener noreferrer"
                  className="text-teal-400 hover:text-teal-300 underline"
                >
                  Get free devnet SOL →
                </a>
              </div>
            )}
          </div>
        )}
        <div className="mt-8 glass-card p-6">
          <h3 className="text-sm font-semibold text-white/60 mb-3">🔒 What happens when you register</h3>
          <ol className="space-y-2 text-xs text-white/35 list-decimal list-inside">
            <li>Each address is hashed with SHA3-256 <span className="text-teal-400/50">(in your browser)</span></li>
            <li>Hashes are split into u128 pairs matching the Arcis circuit struct <span className="text-teal-400/50">(client-side)</span></li>
            {isArciumMode() ? (
              <>
                <li>X25519 key exchange derives shared secret with MXE cluster <span className="text-pv-400/50">(ECDH)</span></li>
                <li>Contacts encrypted with RescueCipher using the shared secret <span className="text-pv-400/50">(Arcium SDK)</span></li>
                <li>Encrypted ciphertexts stored in on-chain UserRegistry PDA <span className="text-pv-400/50">(PrivaSee program)</span></li>
              </>
            ) : (
              <>
                <li>A commitment hash is computed from all contacts <span className="text-teal-400/50">(in your browser)</span></li>
                <li>A real Solana transaction is created with the commitment <span className="text-pv-400/50">(memo program)</span></li>
                <li>You sign the transaction with your wallet <span className="text-pv-400/50">(wallet popup)</span></li>
              </>
            )}
            <li>Transaction is confirmed on Solana devnet <span className="text-pv-400/50">(on-chain proof)</span></li>
          </ol>
        </div>
      </div>
    </div>
  );
}
