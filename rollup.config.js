import { svelteSVG } from "rollup-plugin-svelte-svg";

export default {
    entry: "src/input.js",
    dest: "dist/output.js",
    plugins: [
        svelteSVG({
            // optional SVGO options
            // pass empty object to enable defaults
            svgo: {}
        }),
    ],
    ...
}
