/** @type {import('tailwindcss').Config} */
module.exports = {
    mode: "all",
    content: [
        // include all rust, html and css files in the src directory
        "./src/**/*.{rs,html,css}",
        // include all html files in the output (dist) directory
        "./dist/**/*.html",
    ],
    theme: {
    extend: {
      maxWidth: {
        2000: "2000px",
      },
      textColor: {
        dark: "#191919",
        main: "#d4d4d4",
        highlight: "#f7be71",
        grey: '#949494'
      },
      borderColor: {
        grey: '#949494'
      },
      backgroundColor: {
        dark: "#191919",
        lightgrey: "#949494",
        darkgrey: "#292929",
        highlight: "#f7be71",
        main: "#d4d4d4",

      },
    },
    },
    plugins: [],
}

