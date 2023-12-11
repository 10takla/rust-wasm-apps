import wasm from 'vite-plugin-wasm';
import { defineConfig } from "vite";
import svgr from 'vite-plugin-svgr';
import path from 'path';

export default defineConfig({
    plugins: [
        wasm(),
        svgr({ exportAsDefault: true })
    ],
    resolve: {
        alias: [
            { find: '@', replacement: path.resolve(__dirname, 'src') },
        ],
    },
    build: {
        target: 'esnext'
    }
});
