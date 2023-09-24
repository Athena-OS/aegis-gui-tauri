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
        'secondary-container': '#FFB800',
      },
      textColor: {
        'on-primary': '#492900',
        'on-secondary-container': 'black',
      },
      boxShadow: {
        'elevation2': '0px 2px 3px rgba(0, 0, 0, 0.45)',
        'elevation3': '0px 2px 5px #000000',
      },
      borderWidth: {
        8: "6px"
      },
      colors: {
        primary: "#000000",
      },
    }
  },
  plugins: [],
}

