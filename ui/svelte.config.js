import adapter from "@sveltejs/adapter-static";
import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";

/** @type {import('@sveltejs/kit').Config} */
const config = {
    preprocess: [vitePreprocess({})],

    kit: {
        adapter: adapter({
            fallback: "index.html",
        }),

        alias: {
            "$locales/*": "./src/locales/*",
            "$components/*": "./src/components/*",
            $api: "./src/lib/api/index.ts",
            $: "./src",
        },
    },

    compilerOptions: {
        sourcemap: true,
    },

    vitePlugin: {
        inspector: true,
    },
};

export default config;
