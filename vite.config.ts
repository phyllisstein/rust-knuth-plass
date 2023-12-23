import topLevelAwait from 'vite-plugin-top-level-await'
import wasm from 'vite-plugin-wasm'

/** @type {import('vite').UserConfig} */
export default {
    plugins: [
        wasm(),
        topLevelAwait(),
    ],
}
