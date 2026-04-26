import type { Metadata } from 'next';
import './globals.css';
import { WalletProviderWrapper } from './components/WalletProvider';
import { Navbar } from './components/Navbar';

export const metadata: Metadata = {
  title: 'PrivaSee — Private Contact Discovery on Solana',
  description:
    'Find mutual friends without exposing your contacts. Powered by Arcium MPC on Solana. Zero-knowledge contact matching using Private Set Intersection.',
  keywords: ['Solana', 'Arcium', 'MPC', 'privacy', 'contacts', 'PSI', 'Web3'],
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en" className="dark">
      <body>
        <WalletProviderWrapper>
          <Navbar />
          <main className="pt-20 min-h-screen">{children}</main>
        </WalletProviderWrapper>
      </body>
    </html>
  );
}
