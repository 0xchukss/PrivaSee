'use client';

import React, { useState, useCallback, useEffect } from 'react';
import { useWallet, useConnection } from '@solana/wallet-adapter-react';
import { WalletMultiButton } from '@solana/wallet-adapter-react-ui';
import { PrivacyShield } from '../components/PrivacyShield';
import { MatchGrid } from '../components/MatchCard';
import {
  requestPsiDiscovery,
  hasRegisteredContacts,
  getRegisteredWallets,
  getRegisteredContactCount,
  isArciumMode,
} from '../utils/solana';
import { initializeComputationDefinition } from '../utils/arcium-integration';

type DiscoverStatus = 'idle' | 'encrypting' | 'computing' | 'decrypting' | 'complete' | 'error';

export default function DiscoverPage() {
  const { connected, publicKey, sendTransaction } = useWallet();
  const { connection } = useConnection();
  const [targetAddress, setTargetAddress] = useState('');
  const [myContacts, setMyContacts] = useState('');
  const [status, setStatus] = useState<DiscoverStatus>('idle');
  const [matches, setMatches] = useState<string[]>([]);
  const [txSignature, setTxSignature] = useState<string | null>(null);
  const [errorMessage, setErrorMessage] = useState<string | null>(null);
  const [registeredWallets, setRegisteredWallets] = useState<string[]>([]);
  const [hasMyContacts, setHasMyContacts] = useState(false);
  const [resultMode, setResultMode] = useState<'arcium' | 'demo'>('demo');

  // Load registered wallets for quick selection
  useEffect(() => {
    setRegisteredWallets(getRegisteredWallets());
    if (publicKey) {
      setHasMyContacts(hasRegisteredContacts(publicKey.toBase58()));
    }
  }, [publicKey]);

  const isValidAddress = (addr: string) => /^[1-9A-HJ-NP-Za-km-z]{32,44}$/.test(addr.trim());

  const parseContactsList = (text: string): string[] => {
    return text
      .split(/[\n,]+/)
      .map((s) => s.trim())
      .filter((s) => s.length > 0 && isValidAddress(s));
  };

  const handleDiscover = useCallback(async () => {
    if (!targetAddress.trim() || !isValidAddress(targetAddress)) {
      setErrorMessage('Please enter a valid Solana wallet address');
      return;
    }

    const contactsList = parseContactsList(myContacts);
    if (contactsList.length === 0) {
      setErrorMessage('Please enter your contacts to compare against (one per line)');
      return;
    }

    if (!publicKey) {
      setErrorMessage('Wallet not connected');
      return;
    }

    try {
      setErrorMessage(null);
      setMatches([]);
      setTxSignature(null);

      // Phase 1: Encrypt the request
      setStatus('encrypting');
      await new Promise((r) => setTimeout(r, 800));

      // Phase 2: Sign transaction + MPC computation
      setStatus('computing');

      const result = await requestPsiDiscovery(
        { publicKey, sendTransaction },
        connection,
        targetAddress.trim(),
        contactsList
      );

      // Phase 3: Decrypt result client-side
      setStatus('decrypting');
      await new Promise((r) => setTimeout(r, 500));

      setTxSignature(result.signature);
      setMatches(result.matches);
      setResultMode(result.mode);
      setStatus('complete');
      console.log(`✅ PSI Discovery complete! Mode: ${result.mode}, Found ${result.matches.length} matches. Tx: ${result.signature}`);
    } catch (err: any) {
      console.error('Discovery failed:', err);
      let msg = 'Discovery failed';
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
  }, [targetAddress, myContacts, publicKey, sendTransaction, connection]);

  const handleSetupNetwork = useCallback(async () => {
    if (!publicKey) return;
    try {
      setStatus('encrypting'); // Reuse status for visual feedback
      await initializeComputationDefinition({ publicKey, sendTransaction }, connection);
      setStatus('idle');
      alert('Arcium Network Setup Successful!');
    } catch (err: any) {
      console.error('Setup failed:', err);
      setErrorMessage('Network setup failed: ' + err.message);
      setStatus('error');
    }
  }, [publicKey, sendTransaction, connection]);

  if (!connected) {
    return (
      <div className="page-container py-20">
        <div className="max-w-md mx-auto text-center">
          <div className="text-5xl mb-6">🔍</div>
          <h1 className="text-2xl font-bold mb-3">Connect Your Wallet</h1>
          <p className="text-white/40 mb-8">Connect your Solana wallet to discover mutual contacts on devnet.</p>
          <WalletMultiButton className="!bg-pv-600 !rounded-xl !h-12 !font-semibold hover:!bg-pv-500 !transition-all !mx-auto" />
          <p className="text-xs text-white/25 mt-4">Make sure your wallet is set to <span className="text-teal-400">Solana Devnet</span></p>
        </div>
      </div>
    );
  }

  return (
    <div className="page-container py-12 md:py-20">
      <div className="max-w-2xl mx-auto">
        {/* Header */}
        <div className="text-center mb-10">
          <h1 className="text-3xl md:text-4xl font-bold mb-3">
            Discover <span className="gradient-text-bright">Mutual Contacts</span>
          </h1>
          <p className="text-white/40 max-w-md mx-auto">
            Enter another user&apos;s wallet address and your contacts to find mutual matches via signed on-chain computation.
          </p>
          {/* Mode indicator */}
          <div className="mt-3 flex items-center justify-center gap-2">
            <span className={`inline-flex items-center gap-1.5 px-3 py-1 rounded-full text-xs font-medium ${
              isArciumMode()
                ? 'bg-teal-500/10 border border-teal-500/30 text-teal-400'
                : 'bg-purple-500/10 border border-purple-500/30 text-purple-400'
            }`}>
              <span className={`w-1.5 h-1.5 rounded-full ${isArciumMode() ? 'bg-teal-400 animate-pulse' : 'bg-purple-400'}`} />
              {isArciumMode() ? 'Arcium MPC Mode' : 'Demo Mode (Client-side PSI)'}
            </span>
            {isArciumMode() && (
              <button 
                onClick={handleSetupNetwork}
                className="text-[10px] text-white/20 hover:text-white/50 transition-colors underline"
              >
                One-time Network Setup
              </button>
            )}
          </div>
        </div>

        {/* Privacy Shield */}
        <div className="mb-8">
          <PrivacyShield
            isComputing={['encrypting', 'computing', 'decrypting'].includes(status)}
            status={status === 'idle' ? 'idle' : status}
            message={
              status === 'encrypting' ? 'Hashing and encrypting your contacts...'
                : status === 'computing'
                  ? isArciumMode()
                    ? 'MPC nodes are securely computing the intersection...'
                    : 'Sign the transaction in your wallet to initiate PSI...'
                : status === 'decrypting' ? 'Decrypting intersection results locally...'
                : status === 'complete' ? `Found ${matches.length} mutual contact${matches.length !== 1 ? 's' : ''}${txSignature ? ' • Verified on-chain' : ''}`
                : undefined
            }
          />
        </div>

        {/* Input section */}
        {status === 'idle' || status === 'error' ? (
          <div className="space-y-4">
            <div className="glass-card p-6 md:p-8 space-y-4">
              <label htmlFor="target-wallet" className="block text-sm font-medium text-white/60">
                Target Wallet Address
              </label>
              <input
                id="target-wallet"
                type="text"
                value={targetAddress}
                onChange={(e) => setTargetAddress(e.target.value)}
                placeholder="Enter the Solana wallet address to compare against..."
                className="input-field font-mono text-sm"
              />

              {/* Show registered wallets for quick selection */}
              {registeredWallets.length > 0 && (
                <div className="space-y-2">
                  <span className="text-xs text-white/30">Registered wallets (click to select):</span>
                  <div className="flex flex-wrap gap-2">
                    {registeredWallets
                      .filter((w) => w !== publicKey?.toBase58())
                      .map((w) => (
                        <button
                          key={w}
                          onClick={() => setTargetAddress(w)}
                          className="px-3 py-1.5 rounded-lg text-xs font-mono bg-surface-900/60 border border-white/[0.06] text-white/50 hover:text-white/80 hover:border-pv-500/30 transition-all"
                        >
                          {w.slice(0, 4)}...{w.slice(-4)}
                          <span className="ml-1 text-teal-400/50">({getRegisteredContactCount(w)})</span>
                        </button>
                      ))}
                    {registeredWallets.filter((w) => w !== publicKey?.toBase58()).length === 0 && (
                      <span className="text-xs text-white/20">No other wallets have registered yet</span>
                    )}
                  </div>
                </div>
              )}

              <div className="border-t border-white/[0.06] my-4" />

              <label htmlFor="my-contacts" className="block text-sm font-medium text-white/60">
                Your Contacts <span className="text-white/30">(one address per line)</span>
              </label>
              <textarea
                id="my-contacts"
                value={myContacts}
                onChange={(e) => setMyContacts(e.target.value)}
                placeholder={"Paste your Solana wallet addresses here, one per line:\ne.g.\nGh9ZwEmdLJ8DscKNTkTqPbNwLNNBjuSzaG9Vp2KGtKJr\n7S3P4HxJpyyigGzodYwHtCxZyUQe9JiBMHyRWXArAaKv"}
                rows={5}
                className="input-field font-mono text-sm resize-none"
              />
              <div className="flex items-center justify-between text-xs">
                <span className="text-white/30">
                  {parseContactsList(myContacts).length} valid addresses entered
                </span>
              </div>

              {errorMessage && (
                <div className="flex items-center gap-2 text-red-400 text-sm">
                  <span>⚠️</span> {errorMessage}
                  {errorMessage.includes('SOL') && (
                    <a
                      href="https://faucet.solana.com/"
                      target="_blank"
                      rel="noopener noreferrer"
                      className="text-teal-400 hover:text-teal-300 underline ml-2"
                    >
                      Get devnet SOL →
                    </a>
                  )}
                </div>
              )}

              <button
                onClick={handleDiscover}
                disabled={!targetAddress.trim() || parseContactsList(myContacts).length === 0}
                className="btn-primary w-full"
                id="discover-button"
              >
                <svg className="w-4 h-4" viewBox="0 0 20 20" fill="currentColor">
                  <path fillRule="evenodd" d="M8 4a4 4 0 100 8 4 4 0 000-8zM2 8a6 6 0 1110.89 3.476l4.817 4.817a1 1 0 01-1.414 1.414l-4.816-4.816A6 6 0 012 8z" clipRule="evenodd" />
                </svg>
                {isArciumMode() ? 'Sign & Run MPC Discovery' : 'Sign & Discover Mutual Contacts'}
              </button>

              <p className="text-xs text-white/25 text-center">
                {isArciumMode()
                  ? 'Your contacts are encrypted with X25519 + RescueCipher and compared via Arcium MPC cluster.'
                  : 'You will be asked to sign a Solana transaction. Both sets are compared via private set intersection.'}
              </p>
            </div>
          </div>
        ) : status === 'complete' ? (
          <div className="space-y-6 animate-fade-up">
            <MatchGrid matches={matches} />

            {/* Mode result badge */}
            <div className="text-center">
              <span className={`inline-flex items-center gap-1.5 px-2.5 py-1 rounded-full text-xs ${
                resultMode === 'arcium'
                  ? 'bg-teal-500/15 text-teal-400'
                  : 'bg-purple-500/15 text-purple-400'
              }`}>
                {resultMode === 'arcium' ? '⚡ Computed by Arcium MPC Cluster' : '🔒 Client-side PSI (Hackathon Demo)'}
              </span>
            </div>

            {txSignature && (
              <div className="glass-card p-4 text-center">
                <span className="text-xs text-white/30">Discovery Transaction: </span>
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
              <button
                onClick={() => { setStatus('idle'); setMatches([]); setTargetAddress(''); setMyContacts(''); setTxSignature(null); }}
                className="btn-secondary"
              >
                New Search
              </button>
            </div>
          </div>
        ) : (
          <div className="glass-card p-8 text-center">
            <div className="flex flex-col items-center gap-4">
              <div className="w-8 h-8 border-2 border-teal-500/50 border-t-teal-400 rounded-full animate-spin" />
              <p className="text-sm text-white/50">
                {status === 'computing'
                  ? isArciumMode()
                    ? 'Arcium MPC nodes are securely computing the intersection...'
                    : 'Please approve the transaction in your wallet...'
                  : status === 'encrypting'
                  ? 'Hashing and encrypting contacts...'
                  : 'Decrypting results locally...'}
              </p>
              <p className="text-xs text-white/25">
                {isArciumMode()
                  ? 'Data is encrypted via RescueCipher — MPC nodes never see plaintext.'
                  : 'Your contacts remain encrypted throughout this process.'}
              </p>
            </div>
          </div>
        )}

        {/* Info section */}
        <div className="mt-8 glass-card p-6">
          <h3 className="text-sm font-semibold text-white/60 mb-3">🔍 How discovery works</h3>
          <ol className="space-y-2 text-xs text-white/35 list-decimal list-inside">
            <li>Your contacts are hashed with SHA3-256 <span className="text-teal-400/50">(in your browser)</span></li>
            {isArciumMode() ? (
              <>
                <li>Hashes encrypted via X25519 + RescueCipher <span className="text-pv-400/50">(Arcium SDK)</span></li>
                <li>request_psi instruction queues computation on MPC cluster <span className="text-pv-400/50">(PrivaSee program)</span></li>
                <li>Arx nodes compute intersection on encrypted secret shares <span className="text-teal-400/50">(MPC)</span></li>
                <li>Encrypted result returned via callback, decrypted client-side <span className="text-teal-400/50">(X25519)</span></li>
              </>
            ) : (
              <>
                <li>A Solana transaction is created recording the PSI request <span className="text-pv-400/50">(memo program)</span></li>
                <li>You sign the transaction with your wallet <span className="text-pv-400/50">(wallet popup)</span></li>
                <li>Private Set Intersection compares hashed contacts <span className="text-teal-400/50">(client-side)</span></li>
              </>
            )}
            <li>Only mutual matches are revealed — non-matches are never exposed</li>
          </ol>
        </div>
      </div>
    </div>
  );
}
