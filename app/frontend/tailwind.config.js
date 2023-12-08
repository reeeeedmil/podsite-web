/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    './src/**/*.{html,vue,js}',
  ],
    theme: {
    extend: {
      colors: {
        'smoky': '#090c02',
        'coyote': '#7e6551',
        'buff': '#db995a',
        'vanilla': {
          base: '#d6d4a0',
          light: '#D5E4C3',
        },
        'nyanza': '#d5ecd4',
        'ash': '#a1b0ab',
        'rufous': '#a72608',
      },
      fontFamily: {
              "ibm": "ibm",
              "skyhook": "skyhook",
              "unibody8": "unibody8",
            },
    },
  },
  plugins: [],
}

