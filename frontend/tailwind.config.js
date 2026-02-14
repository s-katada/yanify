/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        smoke: {
          50: '#f5f0e8',
          100: '#e8ddd0',
          200: '#c4b49e',
          300: '#9e8b74',
          400: '#7a6a55',
          500: '#5c4e3c',
          600: '#433929',
          700: '#342c1f',
          800: '#261f16',
          900: '#1a150e',
        },
        amber: {
          400: '#f59e0b',
          500: '#d97706',
          600: '#b45309',
        },
      },
      animation: {
        'smoke-1': 'smoke 4s ease-out infinite',
        'smoke-2': 'smoke 5s ease-out infinite 1s',
        'smoke-3': 'smoke 6s ease-out infinite 2s',
        'fade-in': 'fadeIn 0.5s ease-in-out',
      },
      keyframes: {
        smoke: {
          '0%': {
            transform: 'translateY(0) scale(1)',
            opacity: '0.6',
          },
          '50%': {
            transform: 'translateY(-40px) scale(1.5)',
            opacity: '0.3',
          },
          '100%': {
            transform: 'translateY(-80px) scale(2)',
            opacity: '0',
          },
        },
        fadeIn: {
          '0%': { opacity: '0', transform: 'translateY(10px)' },
          '100%': { opacity: '1', transform: 'translateY(0)' },
        },
      },
    },
  },
  plugins: [],
}
