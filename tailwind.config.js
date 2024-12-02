module.exports = {
    purge: {
        mode: "all",
        content: [
            "./src/**/*.rs",
            "./index.html",
            "./src/**/*.html",
            "./src/**/*.css",
        ],
    },
    plugins: [
        require("@tailwindcss/typography"), require("daisyui"),
    ],
    daisyui: {
        themes: ["dark", "black", "emerald", "black", "corporate", "autumn", "nord", "lofi", "pastel", "business", "dracula"],
    },
    theme: {
    },
    variants: {},
};
