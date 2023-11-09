/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    files: ["*.html", "./src/**/*.rs"],
  },
  theme: {
    extend: {
      transitionProperty: {
        "grid-template-rows": "grid-template-rows",
      },
    },
  },
  plugins: [require("tailwind-scrollbar")],
};
