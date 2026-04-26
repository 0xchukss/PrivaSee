'use client';

import React, { useState, useEffect } from 'react';

interface PrivacyShieldProps {
  /** Whether a computation is currently in progress */
  isComputing: boolean;
  /** Current status label */
  status?: 'idle' | 'encrypting' | 'computing' | 'decrypting' | 'complete' | 'error';
  /** Optional message to display */
  message?: string;
}

/**
 * PrivacyShield — Animated status indicator for MPC computation.
 *
 * Shows a shield icon that animates while the Arcium MPC cluster
 * is processing the PSI computation. The orbiting particles represent
 * the distributed nature of multi-party computation.
 */
export function PrivacyShield({ isComputing, status = 'idle', message }: PrivacyShieldProps) {
  const [dots, setDots] = useState('');

  // Animate loading dots
  useEffect(() => {
    if (!isComputing) {
      setDots('');
      return;
    }
    const interval = setInterval(() => {
      setDots((prev) => (prev.length >= 3 ? '' : prev + '.'));
    }, 500);
    return () => clearInterval(interval);
  }, [isComputing]);

  const statusConfig = {
    idle: {
      color: 'from-white/20 to-white/5',
      ring: 'border-white/10',
      label: 'Ready',
      icon: '🛡️',
    },
    encrypting: {
      color: 'from-pv-500/40 to-pv-700/20',
      ring: 'border-pv-500/40',
      label: 'Encrypting locally',
      icon: '🔐',
    },
    computing: {
      color: 'from-teal-500/40 to-pv-500/30',
      ring: 'border-teal-500/40',
      label: 'MPC computing',
      icon: '⚡',
    },
    decrypting: {
      color: 'from-teal-400/40 to-teal-600/20',
      ring: 'border-teal-400/40',
      label: 'Decrypting result',
      icon: '🔓',
    },
    complete: {
      color: 'from-green-500/30 to-teal-500/20',
      ring: 'border-green-500/40',
      label: 'Complete',
      icon: '✅',
    },
    error: {
      color: 'from-red-500/30 to-red-700/20',
      ring: 'border-red-500/40',
      label: 'Error',
      icon: '❌',
    },
  };

  const config = statusConfig[status];

  return (
    <div className="flex flex-col items-center gap-4">
      {/* Shield container */}
      <div className="relative w-32 h-32">
        {/* Outer glow ring */}
        <div
          className={`absolute inset-0 rounded-full bg-gradient-to-br ${config.color} transition-all duration-700 ${
            isComputing ? 'animate-shield-pulse' : ''
          }`}
        />

        {/* Spinning ring (visible during computation) */}
        {isComputing && (
          <div className="absolute inset-2 rounded-full border-2 border-dashed border-teal-500/30 animate-shield-spin" />
        )}

        {/* Inner shield */}
        <div
          className={`absolute inset-4 rounded-full border-2 ${config.ring} bg-surface-900/80 backdrop-blur-sm flex items-center justify-center transition-all duration-500`}
        >
          <svg
            viewBox="0 0 64 64"
            fill="none"
            className={`w-12 h-12 transition-all duration-500 ${
              isComputing ? 'animate-float' : ''
            }`}
          >
            {/* Shield shape */}
            <path
              d="M32 8L12 18v12c0 11.1 8.54 21.48 20 24 11.46-2.52 20-12.9 20-24V18L32 8z"
              fill="url(#shieldGradient)"
              stroke="url(#shieldStroke)"
              strokeWidth="2"
            />
            {/* Inner eye/check */}
            {status === 'complete' ? (
              <path
                d="M22 32l6 6 14-14"
                stroke="white"
                strokeWidth="3"
                strokeLinecap="round"
                strokeLinejoin="round"
              />
            ) : (
              <>
                <circle cx="32" cy="30" r="6" fill="rgba(255,255,255,0.9)" />
                <circle cx="32" cy="30" r="2.5" fill="rgba(124,58,237,0.8)" />
              </>
            )}
            <defs>
              <linearGradient id="shieldGradient" x1="12" y1="8" x2="52" y2="44">
                <stop offset="0%" stopColor="rgba(124,58,237,0.3)" />
                <stop offset="100%" stopColor="rgba(36,168,170,0.2)" />
              </linearGradient>
              <linearGradient id="shieldStroke" x1="12" y1="8" x2="52" y2="44">
                <stop offset="0%" stopColor="rgba(180,157,255,0.8)" />
                <stop offset="100%" stopColor="rgba(63,196,197,0.6)" />
              </linearGradient>
            </defs>
          </svg>
        </div>

        {/* Orbiting particles (during computation) */}
        {isComputing && (
          <>
            <div
              className="absolute top-1/2 left-1/2 w-2 h-2 rounded-full bg-pv-400"
              style={{ animation: 'shield-orbit 3s linear infinite' }}
            />
            <div
              className="absolute top-1/2 left-1/2 w-1.5 h-1.5 rounded-full bg-teal-400"
              style={{ animation: 'shield-orbit 3s linear infinite 1s' }}
            />
            <div
              className="absolute top-1/2 left-1/2 w-1.5 h-1.5 rounded-full bg-pv-300"
              style={{ animation: 'shield-orbit 3s linear infinite 2s' }}
            />
          </>
        )}
      </div>

      {/* Status label */}
      <div className="text-center">
        <div className="flex items-center gap-2 justify-center">
          <span className="text-lg">{config.icon}</span>
          <span
            className={`text-sm font-medium ${
              isComputing ? 'text-teal-300' : 'text-white/60'
            }`}
          >
            {config.label}
            {isComputing && dots}
          </span>
        </div>
        {message && (
          <p className="text-xs text-white/40 mt-1 max-w-xs">{message}</p>
        )}
      </div>
    </div>
  );
}
