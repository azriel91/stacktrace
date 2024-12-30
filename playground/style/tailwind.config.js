/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    // Relative to workspace root / where you run `cargo leptos` from.
    '**/src/*.rs',
    '**/src/**/*.rs',
  ],
  theme: {
    extend: {
      backgroundImage: {
        'arrow': "url(\"data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' width='8' height='10'><path d='M 0 0 L 8 5 L 0 10 Z' fill='black' stroke='black' /></svg>\")",
      }
    },
  },
  plugins: [],
}
