import path from 'node:path'
import { fileURLToPath } from 'node:url'
import HTMLWebpackPlugin from 'html-webpack-plugin'
import WasmPackPlugin from '@wasm-tool/wasm-pack-plugin'

const __filename = fileURLToPath(import.meta.url)
const __dirname = path.dirname(__filename)
const dist = path.resolve(__dirname, 'dist')

export default {
    mode: 'development',
    entry: './js/index.ts',
    output: {
        path: dist,
        filename: '[name].js',
    },
    module: {
        rules: [
            {
                test: /\.(m|c|t)?js$/,
                use: '',
            }
        ]
    },
    plugins: [
        new HTMLWebpackPlugin(),
        new WasmPackPlugin({
            crateDirectory: path.resolve(__dirname, 'rust'),
            outDir: path.resolve(__dirname, 'rust/pkg'),
        })
    ],
    experiments: {
        asyncWebAssembly: true,
    }
}
