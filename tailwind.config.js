/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    './app/**/*.{js,ts,jsx,tsx,mdx}',
  ],
  darkMode: 'class',
  theme: {
    extend: {
      colors: {
        // PrivaSee brand palette — Solana-inspired purple/teal
        'pv': {
          50:  '#f3f0ff',
          100: '#e9e3ff',
          200: '#d4c8ff',
          300: '#b49dff',
          400: '#9366ff',
          500: '#7c3aed',
          600: '#6d28d9',
          700: '#5b21b6',
          800: '#4c1d95',
          900: '#3b0f7a',
          950: '#1e0547',
        },
        'teal': {
          50:  '#effcfc',
          100: '#d6f5f5',
          200: '#b2ebeb',
          300: '#7cdcdc',
          400: '#3fc4c5',
          500: '#24a8aa',
          600: '#1f888f',
          700: '#1f6d74',
          800: '#215960',
          900: '#204a51',
          950: '#0e2f35',
        },
        'surface': {
          50:  '#f8fafc',
          100: '#f1f5f9',
          200: '#e2e8f0',
          700: '#1e2337',
          800: '#151929',
          900: '#0d1017',
          950: '#080a10',
        },
      },
      fontFamily: {
        sans: ['Inter', 'system-ui', '-apple-system', 'sans-serif'],
        mono: ['JetBrains Mono', 'Fira Code', 'monospace'],
      },
      animation: {
        'shield-pulse': 'shield-pulse 2s ease-in-out infinite',
        'shield-spin': 'shield-spin 3s linear infinite',
        'gradient-shift': 'gradient-shift 6s ease infinite',
        'fade-up': 'fade-up 0.6s ease-out forwards',
        'float': 'float 3s ease-in-out infinite',
        'glow': 'glow 2s ease-in-out infinite alternate',
      },
      keyframes: {
        'shield-pulse': {
          '0%, 100%': { transform: 'scale(1)', opacity: '1' },
          '50%': { transform: 'scale(1.05)', opacity: '0.85' },
        },
        'shield-spin': {
          '0%': { transform: 'rotate(0deg)' },
          '100%': { transform: 'rotate(360deg)' },
        },
        'gradient-shift': {
          '0%, 100%': { backgroundPosition: '0% 50%' },
          '50%': { backgroundPosition: '100% 50%' },
        },
        'fade-up': {
          '0%': { opacity: '0', transform: 'translateY(20px)' },
          '100%': { opacity: '1', transform: 'translateY(0)' },
        },
        'float': {
          '0%, 100%': { transform: 'translateY(0px)' },
          '50%': { transform: 'translateY(-10px)' },
        },
        'glow': {
          '0%': { boxShadow: '0 0 20px rgba(124, 58, 237, 0.3)' },
          '100%': { boxShadow: '0 0 40px rgba(124, 58, 237, 0.6), 0 0 80px rgba(36, 168, 170, 0.2)' },
        },
      },
      backgroundImage: {
        'gradient-radial': 'radial-gradient(var(--tw-gradient-stops))',
        'hero-pattern': 'radial-gradient(circle at 30% 20%, rgba(124, 58, 237, 0.15) 0%, transparent 50%), radial-gradient(circle at 70% 80%, rgba(36, 168, 170, 0.1) 0%, transparent 50%)',
      },
    },
  },
  plugins: [],
};
