// index.js

import init, { WorldWrapper } from './pkg/wasm_triangle.js';

const runWasm = async () => {
    await init(); // Initialize the wasm module

    const canvas = document.getElementById('canvas');
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;

    const width = 64;
    const height = 64;
    const worldWrapper = WorldWrapper.new(width, height);

    // Call the update_and_render method on the WorldWrapper instance
    worldWrapper.update_and_render(canvas);
};

runWasm();
