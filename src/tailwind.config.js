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
        // main: "#FFFFFF",
        // hover: "#33FF00",
        // Gruvbox
        // main: "#fbf1c7",
        // hover: "#fe8019",
        // Nord
        // main: "#e5e9f0",
        // highlight: "#88c0d0",
        // Notion
        main: "#d4d4d4",
        highlight: "#88c0d0",
      },
      backgroundColor: {
        // dark: "#000000",
        // Gruvbox
        // dark: "#282828",
        // Nord
        // dark: "#2e3440",
        // Notion
        dark: "#191919",
      },
    },
    },
    plugins: [],
}

