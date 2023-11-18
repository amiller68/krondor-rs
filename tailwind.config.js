/** @type {import('tailwindcss').Config} */
module.exports = {
  content: { 
    files: ["*.html", "./src/**/*.rs"],
  },
  theme: {
    extend: {
      colors: {
        dark: '#121212', // primary dark color
        gray: {
          DEFAULT: '#1e1e1e',
          dark: '#181818',
          light: '#242424'
        },
        accent: '#14b8a6'
      },
      fontFamily: {
        'body': ['Roboto Mono', 'monospace'],
      },
      spacing: {
        '10p': '10%', // for header height
      },
      keyframes: {
        colorCycle: {
          '0%, 100%': { color: '#FF6347' },
          '33%': { color: '#4A90E2' },
          '66%': { color: '#32CD32' }
        },
      },
      animation: {
        colorCycle: 'colorCycle 3s ease-in-out infinite',
      },
    },
  },
  plugins: [],
}
