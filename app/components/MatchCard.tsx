'use client';

import React from 'react';

interface MatchCardProps {
  /** The wallet address of the matched contact */
  address: string;
  /** Optional index for staggered animation */
  index?: number;
}

/**
 * MatchCard — Displays a single mutual contact match.
 *
 * Only shows contacts that were found in BOTH users' contact lists.
 * Non-matching contacts are simply never shown — zero leakage.
 */
export function MatchCard({ address, index = 0 }: MatchCardProps) {
  // Generate a deterministic color from the address for the avatar
  const hue = address
    .split('')
    .reduce((acc, char) => acc + char.charCodeAt(0), 0) % 360;

  const shortAddress = `${address.slice(0, 4)}...${address.slice(-4)}`;

  return (
    <div
      className="glass-card-hover p-5 animate-fade-up"
      style={{ animationDelay: `${index * 100}ms` }}
    >
      <div className="flex items-center gap-4">
        {/* Avatar */}
        <div className="relative flex-shrink-0">
          <div
            className="w-12 h-12 rounded-full flex items-center justify-center text-white font-bold text-lg"
            style={{
              background: `linear-gradient(135deg, hsl(${hue}, 70%, 50%), hsl(${hue + 60}, 70%, 40%))`,
            }}
          >
            {address.slice(0, 2)}
          </div>
          {/* Match indicator dot */}
          <div className="absolute -bottom-0.5 -right-0.5 w-4 h-4 rounded-full bg-green-500 border-2 border-surface-800 flex items-center justify-center">
            <svg className="w-2.5 h-2.5 text-white" viewBox="0 0 20 20" fill="currentColor">
              <path
                fillRule="evenodd"
                d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z"
                clipRule="evenodd"
              />
            </svg>
          </div>
        </div>

        {/* Info */}
        <div className="min-w-0 flex-1">
          <div className="flex items-center gap-2">
            <span className="font-mono text-sm text-white/90 truncate">
              {address}
            </span>
          </div>
          <div className="flex items-center gap-2 mt-1">
            <span className="badge-teal">
              <svg className="w-3 h-3" viewBox="0 0 20 20" fill="currentColor">
                <path d="M9 6a3 3 0 11-6 0 3 3 0 016 0zM17 6a3 3 0 11-6 0 3 3 0 016 0zM12.93 17c.046-.327.07-.66.07-1a6.97 6.97 0 00-1.5-4.33A5 5 0 0119 16v1h-6.07zM6 11a5 5 0 015 5v1H1v-1a5 5 0 015-5z" />
              </svg>
              Mutual Contact
            </span>
          </div>
        </div>

        {/* Copy button */}
        <button
          onClick={() => navigator.clipboard.writeText(address)}
          className="flex-shrink-0 p-2 rounded-lg text-white/30 hover:text-white/60 hover:bg-white/[0.05] transition-all"
          title="Copy address"
        >
          <svg className="w-4 h-4" viewBox="0 0 20 20" fill="currentColor">
            <path d="M8 3a1 1 0 011-1h2a1 1 0 110 2H9a1 1 0 01-1-1z" />
            <path d="M6 3a2 2 0 00-2 2v11a2 2 0 002 2h8a2 2 0 002-2V5a2 2 0 00-2-2 3 3 0 01-3 3H9a3 3 0 01-3-3z" />
          </svg>
        </button>
      </div>
    </div>
  );
}

interface MatchGridProps {
  /** List of matched wallet addresses */
  matches: string[];
}

/**
 * MatchGrid — Card grid of all mutual matches.
 * Non-matches are simply not displayed.
 */
export function MatchGrid({ matches }: MatchGridProps) {
  if (matches.length === 0) {
    return (
      <div className="glass-card p-8 text-center">
        <div className="text-4xl mb-3">🔍</div>
        <h3 className="text-lg font-semibold text-white/80 mb-2">
          No Mutual Contacts Found
        </h3>
        <p className="text-sm text-white/40 max-w-sm mx-auto">
          You and this user don&apos;t share any contacts. Your contact lists
          remain completely private — nothing was revealed.
        </p>
      </div>
    );
  }

  return (
    <div className="space-y-4">
      {/* Summary */}
      <div className="flex items-center justify-between">
        <h3 className="text-lg font-semibold text-white/90">
          🎉 {matches.length} Mutual Contact{matches.length !== 1 ? 's' : ''}{' '}
          Found
        </h3>
        <span className="badge">
          <svg className="w-3 h-3" viewBox="0 0 20 20" fill="currentColor">
            <path
              fillRule="evenodd"
              d="M5 9V7a5 5 0 0110 0v2a2 2 0 012 2v5a2 2 0 01-2 2H5a2 2 0 01-2-2v-5a2 2 0 012-2zm8-2v2H7V7a3 3 0 016 0z"
              clipRule="evenodd"
            />
          </svg>
          Verified via MPC
        </span>
      </div>

      {/* Cards */}
      <div className="grid gap-3">
        {matches.map((addr, i) => (
          <MatchCard key={addr} address={addr} index={i} />
        ))}
      </div>

      {/* Privacy footer */}
      <p className="text-xs text-white/25 text-center pt-2">
        Only mutual matches are shown. Non-matching contacts were never revealed
        to anyone — not to the other user, not to the network, not to anyone.
      </p>
    </div>
  );
}
