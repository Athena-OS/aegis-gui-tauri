/** @type {import('tailwindcss').Config} */
export default {
  content: ["./src/**/*.{html,js,svelte,ts}"],
  theme: {
    fontFamily: {
      sans: ['Ubuntu', 'sans-serif'],
      serif: ['Ubuntu', 'serif'],
    },
    extend: {
      backgroundColor: {
        "yellow-500": "#FFB800",
      },
      textColor: {
        "gray-500": "#B0B0B0",
        "gray-600": "#2F2F2F",
        "yellow-500": "#FFB800",
        'on-secondary-container': 'black',
      },
      ringColor: {
        "yellow-500": "#FFB800",
      },
      borderColor: {
        "yellow-500": "#FFB800",
      },
      boxShadow: {
        'elevation2': '0px 2px 3px rgba(0, 0, 0, 0.45)',
        'elevation3': '0px 2px 5px #000000',
      },
      borderWidth: {
        8: "6px"
      },
      colors: {
        primary: "#111111",
      },
    }
  },
  plugins: [],
}

