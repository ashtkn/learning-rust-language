// import * as wasm from "../pkg/mandelbrot";

const wasm = import('../pkg').catch((e) => console.error(e))

wasm.then(({ greet }) => {
    greet();
})