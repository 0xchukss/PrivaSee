'use client';

import React from 'react';
import Link from 'next/link';
import { usePathname } from 'next/navigation';
import { useWallet } from '@solana/wallet-adapter-react';
import { WalletMultiButton } from '@solana/wallet-adapter-react-ui';

/**
 * Navbar — Fixed top navigation with wallet connection.
 */
export function Navbar() {
  const pathname = usePathname();
  const { connected } = useWallet();

  const navLinks = [
    { href: '/', label: 'Home' },
    { href: '/register', label: 'Register' },
    { href: '/discover', label: 'Discover' },
  ];

  return (
    <nav className="navbar">
      <div className="page-container flex items-center justify-between h-16">
        {/* Logo */}
        <Link href="/" className="flex items-center gap-2.5 group">
          <div className="relative w-8 h-8">
            <div className="absolute inset-0 rounded-lg bg-gradient-to-br from-pv-500 to-teal-500 opacity-80 group-hover:opacity-100 transition-opacity" />
            <svg
              viewBox="0 0 32 32"
              fill="none"
              className="relative z-10 w-8 h-8"
            >
              <path
                d="M16 4L6 10v8c0 5.55 4.27 10.74 10 12 5.73-1.26 10-6.45 10-12v-8L16 4z"
                fill="rgba(255,255,255,0.15)"
                stroke="white"
                strokeWidth="1.5"
              />
              <circle cx="16" cy="16" r="3" fill="white" opacity="0.9" />
            </svg>
          </div>
          <span className="text-lg font-bold gradient-text-bright">
            PrivaSee
          </span>
        </Link>

        {/* Nav Links */}
        <div className="hidden sm:flex items-center gap-1">
          {navLinks.map((link) => (
            <Link
              key={link.href}
              href={link.href}
              className={`px-4 py-2 rounded-lg text-sm font-medium transition-all duration-200 ${
                pathname === link.href
                  ? 'text-white bg-white/[0.08]'
                  : 'text-white/50 hover:text-white/80 hover:bg-white/[0.04]'
              }`}
            >
              {link.label}
            </Link>
          ))}
        </div>

        {/* Wallet Button */}
        <div className="flex items-center gap-3">
          {connected && (
            <span className="hidden sm:flex items-center gap-1.5 text-xs text-teal-400">
              <span className="w-1.5 h-1.5 rounded-full bg-teal-400 animate-pulse" />
              Connected
            </span>
          )}
          <WalletMultiButton className="!bg-pv-600 !rounded-xl !h-10 !text-sm !font-medium hover:!bg-pv-500 !transition-all" />
        </div>
      </div>
    </nav>
  );
}
