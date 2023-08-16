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
        lightgrey: '#949494',
        main: "#d4d4d4",
        highlight: "#f7be71",
      },
      borderColor: {
        lightgrey: '#949494',
        darkgrey: '#292929'
      },
      backgroundColor: {
        dark: "#191919",
        darkgrey: "#292929",
        lightgrey: "#949494",
        main: "#d4d4d4",
        highlight: "#f7be71",
        code: "#262626",

      },
    },
    },
    plugins: [],
}

