'use client';

import React from 'react';
import Link from 'next/link';
import { useWallet } from '@solana/wallet-adapter-react';
import { WalletMultiButton } from '@solana/wallet-adapter-react-ui';
import { PrivacyShield } from './components/PrivacyShield';

/**
 * Landing page — explains PrivaSee and guides users to connect their wallet.
 */
export default function HomePage() {
  const { connected } = useWallet();

  return (
    <div className="page-container py-12 md:py-20">
      {/* Hero Section */}
      <section className="text-center mb-24 relative">
        {/* Background glow */}
        <div className="absolute inset-0 -z-10 overflow-hidden">
          <div className="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[600px] h-[600px] rounded-full bg-pv-500/[0.07] blur-[100px]" />
          <div className="absolute top-1/4 right-1/4 w-[400px] h-[400px] rounded-full bg-teal-500/[0.05] blur-[80px]" />
        </div>

        <div className="animate-fade-up">
          <span className="badge mb-6 inline-flex">
            <span className="w-1.5 h-1.5 rounded-full bg-teal-400 animate-pulse" />
            Powered by Arcium MPC on Solana
          </span>
        </div>

        <h1
          className="text-5xl md:text-7xl font-black mb-6 animate-fade-up"
          style={{ animationDelay: '100ms' }}
        >
          Find friends{' '}
          <span className="gradient-text-bright">without exposing</span>
          <br />
          your contacts
        </h1>

        <p
          className="text-lg md:text-xl text-white/50 max-w-2xl mx-auto mb-10 animate-fade-up"
          style={{ animationDelay: '200ms' }}
        >
          PrivaSee uses Private Set Intersection via Multi-Party Computation
          to discover mutual contacts. Only matches are revealed — your full
          contact list stays mathematically provably private.
        </p>

        <div
          className="flex flex-col sm:flex-row items-center justify-center gap-4 animate-fade-up"
          style={{ animationDelay: '300ms' }}
        >
          {connected ? (
            <>
              <Link href="/register" className="btn-primary text-lg px-8 py-4">
                <svg className="w-5 h-5" viewBox="0 0 20 20" fill="currentColor">
                  <path d="M10 2a5 5 0 00-5 5v2a2 2 0 00-2 2v5a2 2 0 002 2h10a2 2 0 002-2v-5a2 2 0 00-2-2V7a5 5 0 00-5-5zm3 7V7a3 3 0 00-6 0v2h6z" />
                </svg>
                Register Your Contacts
              </Link>
              <Link href="/discover" className="btn-secondary text-lg px-8 py-4">
                Discover Matches
              </Link>
            </>
          ) : (
            <div className="flex flex-col items-center gap-3">
              <WalletMultiButton className="!bg-pv-600 !rounded-xl !h-14 !text-lg !font-semibold hover:!bg-pv-500 !transition-all !px-8" />
              <span className="text-sm text-white/30">
                Connect your wallet to get started
              </span>
            </div>
          )}
        </div>

        {/* Shield illustration */}
        <div
          className="mt-16 animate-fade-up"
          style={{ animationDelay: '400ms' }}
        >
          <PrivacyShield isComputing={false} status="idle" />
        </div>
      </section>

      {/* How PrivaSee Protects You — 3-step diagram */}
      <section className="mb-24" id="how-it-works">
        <div className="text-center mb-12">
          <h2 className="text-3xl md:text-4xl font-bold mb-4">
            How PrivaSee{' '}
            <span className="gradient-text">Protects You</span>
          </h2>
          <p className="text-white/40 max-w-lg mx-auto">
            Three steps to private contact discovery. Your data never leaves
            your device unencrypted.
          </p>
        </div>

        <div className="grid md:grid-cols-3 gap-6">
          {/* Step 1 */}
          <div className="glass-card-hover p-8 text-center">
            <div className="step-circle mx-auto mb-5">1</div>
            <div className="text-3xl mb-3">🔐</div>
            <h3 className="text-lg font-semibold mb-3 text-white/90">
              Hashed & Encrypted Locally
            </h3>
            <p className="text-sm text-white/40 leading-relaxed">
              Your contacts are hashed with SHA3-256 and encrypted using
              Arcium&apos;s Rescue cipher — all inside your browser. Raw addresses
              never leave your device.
            </p>
            <div className="mt-4 p-3 rounded-lg bg-surface-900/60 font-mono text-xs text-pv-300/70">
              sha3(&quot;alice.sol&quot;) → 0x7f3a...
              <br />
              encrypt(hash) → ████████
            </div>
          </div>

          {/* Step 2 */}
          <div className="glass-card-hover p-8 text-center">
            <div className="step-circle mx-auto mb-5">2</div>
            <div className="text-3xl mb-3">⚡</div>
            <h3 className="text-lg font-semibold mb-3 text-white/90">
              MPC Nodes Compute on Encrypted Data
            </h3>
            <p className="text-sm text-white/40 leading-relaxed">
              Arcium&apos;s distributed MPC nodes compute the set intersection
              on fully encrypted data. Each node sees only random-looking
              secret shares — never your actual contacts.
            </p>
            <div className="mt-4 p-3 rounded-lg bg-surface-900/60 text-xs text-teal-300/70">
              <div className="flex items-center justify-center gap-2">
                <span className="w-2 h-2 rounded-full bg-pv-400/60" />
                Node A: share₁
                <span className="w-2 h-2 rounded-full bg-teal-400/60" />
                Node B: share₂
              </div>
              <div className="mt-1">
                intersect(████, ████) → result
              </div>
            </div>
          </div>

          {/* Step 3 */}
          <div className="glass-card-hover p-8 text-center">
            <div className="step-circle mx-auto mb-5">3</div>
            <div className="text-3xl mb-3">✅</div>
            <h3 className="text-lg font-semibold mb-3 text-white/90">
              Only Matches Are Revealed
            </h3>
            <p className="text-sm text-white/40 leading-relaxed">
              The encrypted result is sent back and decrypted locally.
              Only mutual matches are shown. Non-matching contacts are
              mathematically impossible to reconstruct.
            </p>
            <div className="mt-4 p-3 rounded-lg bg-surface-900/60 text-xs text-green-300/70">
              ✓ alice.sol — Mutual contact
              <br />
              ✓ bob.sol — Mutual contact
              <br />
              <span className="text-white/20">
                ✗ Others — Never revealed
              </span>
            </div>
          </div>
        </div>

        {/* Connection lines (desktop only) */}
        <div className="hidden md:flex justify-center mt-8 gap-4">
          <div className="flex items-center gap-2 text-xs text-white/20">
            <div className="w-20 h-px bg-gradient-to-r from-pv-500/30 to-teal-500/30" />
            <span>encrypted</span>
            <div className="w-20 h-px bg-gradient-to-r from-teal-500/30 to-green-500/30" />
            <span>verified</span>
            <div className="w-20 h-px bg-green-500/30" />
          </div>
        </div>
      </section>

      {/* Privacy Guarantee Section */}
      <section className="mb-24">
        <div className="glass-card p-8 md:p-12 relative overflow-hidden">
          {/* Background accent */}
          <div className="absolute top-0 right-0 w-64 h-64 rounded-full bg-pv-500/[0.06] blur-[60px]" />

          <div className="relative z-10 max-w-3xl">
            <span className="badge-teal mb-4 inline-flex">
              Privacy Guarantee
            </span>
            <h2 className="text-2xl md:text-3xl font-bold mb-4">
              Your contacts are{' '}
              <span className="gradient-text-bright">
                mathematically provably private
              </span>
            </h2>
            <p className="text-white/50 mb-6 leading-relaxed">
              Arcium&apos;s MPC nodes process only ciphertexts — your contact list
              is never reconstructible by any single party. Under Arcium&apos;s
              dishonest majority model, privacy holds even if all but one
              MPC node are compromised. The only information revealed is the
              intersection — which contacts you have in common.
            </p>

            <div className="grid sm:grid-cols-3 gap-4">
              <div className="p-4 rounded-xl bg-surface-900/60 border border-white/[0.04]">
                <div className="text-2xl mb-2">🔒</div>
                <div className="text-sm font-medium text-white/70">
                  End-to-End Encrypted
                </div>
                <div className="text-xs text-white/30 mt-1">
                  X25519 + Rescue cipher
                </div>
              </div>
              <div className="p-4 rounded-xl bg-surface-900/60 border border-white/[0.04]">
                <div className="text-2xl mb-2">🌐</div>
                <div className="text-sm font-medium text-white/70">
                  Decentralized MPC
                </div>
                <div className="text-xs text-white/30 mt-1">
                  No single point of trust
                </div>
              </div>
              <div className="p-4 rounded-xl bg-surface-900/60 border border-white/[0.04]">
                <div className="text-2xl mb-2">⛓️</div>
                <div className="text-sm font-medium text-white/70">
                  On-Chain Verification
                </div>
                <div className="text-xs text-white/30 mt-1">
                  Solana-verified results
                </div>
              </div>
            </div>
          </div>
        </div>
      </section>

      {/* Architecture Diagram */}
      <section className="mb-24">
        <div className="text-center mb-8">
          <h2 className="text-2xl font-bold mb-2">Architecture</h2>
          <p className="text-white/40 text-sm">
            How PrivaSee integrates with Arcium and Solana
          </p>
        </div>
        <div className="glass-card p-6 md:p-8 font-mono text-xs md:text-sm text-white/60 overflow-x-auto">
          <pre className="whitespace-pre leading-relaxed">{`
  ┌─────────────────────────────────────────────────────────────┐
  │                        CLIENT BROWSER                       │
  │                                                             │
  │  Contacts ──→ SHA3 Hash ──→ X25519+Rescue Encrypt ──→ ████ │
  │                                                     │       │
  │  Decrypted Matches ←── Rescue Decrypt ←─────────────┼───┐   │
  └─────────────────────────────────────────────────────┼───┼───┘
                                                        │   │
                                                        ▼   │
  ┌─────────────────────────────────────────────────────────┼───┐
  │                    SOLANA BLOCKCHAIN                     │   │
  │                                                         │   │
  │  PrivaSee Program (MXE)                                 │   │
  │  ├─ register_contacts(encrypted_set) ──→ UserRegistry   │   │
  │  ├─ request_psi(user_a, user_b)      ──→ Queue Comp     │   │
  │  └─ compute_psi_callback(result)     ──→ PsiResultEvent─┘   │
  │                         │                                    │
  │                         ▼                                    │
  │              Arcium Program (CPI)                            │
  │              └─ queue_computation()                           │
  └──────────────────────┬──────────────────────────────────────┘
                         │
                         ▼
  ┌─────────────────────────────────────────────────────────────┐
  │                   ARCIUM MPC CLUSTER                        │
  │                                                             │
  │  Node A: share₁ ──┐                                        │
  │  Node B: share₂ ──┼──→ compute_psi(████, ████) ──→ result  │
  │  Node C: share₃ ──┘                                        │
  │                                                             │
  │  ⚡ No node sees plaintext contacts                         │
  │  ⚡ Only encrypted intersection returned                    │
  └─────────────────────────────────────────────────────────────┘
          `}</pre>
        </div>
      </section>

      {/* CTA */}
      <section className="text-center pb-16">
        <h2 className="text-3xl font-bold mb-4">
          Ready to discover contacts{' '}
          <span className="gradient-text">privately</span>?
        </h2>
        <p className="text-white/40 mb-8">
          Connect your wallet and register your contacts in seconds.
        </p>
        {connected ? (
          <Link href="/register" className="btn-primary text-lg px-8 py-4">
            Get Started →
          </Link>
        ) : (
          <WalletMultiButton className="!bg-pv-600 !rounded-xl !h-14 !text-lg !font-semibold hover:!bg-pv-500 !transition-all !px-8" />
        )}
      </section>
    </div>
  );
}
