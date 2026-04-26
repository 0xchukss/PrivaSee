'use client';

import React, { useMemo } from 'react';
import {
  ConnectionProvider,
  WalletProvider,
} from '@solana/wallet-adapter-react';
import { WalletModalProvider } from '@solana/wallet-adapter-react-ui';
import { PhantomWalletAdapter } from '@solana/wallet-adapter-phantom';
import { clusterApiUrl } from '@solana/web3.js';

// Import wallet adapter default styles
import '@solana/wallet-adapter-react-ui/styles.css';

/**
 * WalletProviderWrapper
 * 
 * Wraps the entire app with Solana wallet connectivity.
 * Supports Phantom and Backpack wallets.
 * Connected to Solana devnet for development.
 */
export function WalletProviderWrapper({
  children,
}: {
  children: React.ReactNode;
}) {
  const endpoint = useMemo(() => clusterApiUrl('devnet'), []);

  const wallets = useMemo(
    () => [
      new PhantomWalletAdapter(),
    ],
    []
  );

  return (
    <ConnectionProvider endpoint={endpoint}>
      <WalletProvider wallets={wallets} autoConnect>
        <WalletModalProvider>{children}</WalletModalProvider>
      </WalletProvider>
    </ConnectionProvider>
  );
}
