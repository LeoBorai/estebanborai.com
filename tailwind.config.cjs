const forms = require('@tailwindcss/forms');
const typography = require('@tailwindcss/typography');
const lineClamp = require('@tailwindcss/line-clamp');

/** @type {import('tailwindcss').Config} */
module.exports = {
  darkMode: 'class',
  content: ['./src/**/*.{html,js,svelte,ts}'],
  theme: {
    extend: {
      colors: {
        light: {
          background: '#EBDFD1',
          'background-alt': '#FDF1E2',
        },
        dark: {
          background: '#1C1C1C',
          'background-alt': '#1A1A1A',
        },
      },
      fontFamily: {
        body: ['Inter', 'sans'],
        mono: ['Fira Code', 'monospace'],
      },
    },
  },
  plugins: [forms, typography, lineClamp],
};
