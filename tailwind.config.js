/** @type {import('tailwindcss').Config} */
export default {
  content: ["./src/**/*.{html,js,svelte,ts}"],
  theme: {
    fontFamily: {
      sans: ['Ubuntu', 'sans-serif'],
      serif: ['Ubuntu', 'serif'],
    },
    extend: {
      colors: {
        primary: "#11131C",
      },
    }
  },
  plugins: [],
}

