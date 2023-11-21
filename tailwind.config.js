const colors = require("tailwindcss/colors");

/** @type {import('tailwindcss').Config} */
export default {
  content: ["./src/**/*.{html,js,svelte,ts}"],
  darkMode: "class",
  theme: {
    extend: {
      colors: {
        primary: colors.yellow,
        accent: colors.cyan,
        gray: {
          500: "#5B5B5B",
          700: "#2F2F2F",
          800: "#1A1A1A",
          900: "#111111",
        },
      },
      fontFamily: {
        sans: ["Ubuntu", "sans-serif"],
      },
      transitionProperty: {
        height: "height",
      },
    },
  },
  plugins: [],
};
