/** @type {import('next').NextConfig} */
const path = require('path');

const nextConfig = {
  reactStrictMode: true,
  images: {
    unoptimized: true,
  },
  webpack: (config) => {
    // Required for Solana wallet adapter compatibility
    config.resolve.fallback = {
      ...config.resolve.fallback,
      fs: false,
      os: false,
      path: false,
      crypto: false,
    };

    // Fix @noble/curves subpath resolution for Next.js SWC
    config.resolve.alias = {
      ...config.resolve.alias,
      '@noble/curves/ed25519': path.resolve(
        __dirname,
        'node_modules/@noble/curves/ed25519.js'
      ),
    };

    return config;
  },
};

module.exports = nextConfig;
