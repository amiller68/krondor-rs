/** @type {import('tailwindcss').Config} */
module.exports = {
  content: { 
    files: ["*.html", "./src/**/*.rs"],
  },
  theme: {
    extend: {
      colors: {
        'egg-cream': '#f0ead6', // Replace with your exact egg cream color
      },
      fontFamily: {
        'comic-sans': ['Comic Sans MS', 'cursive'] // Make sure Comic Sans is available
      },
    },
  },
  plugins: [],
}
