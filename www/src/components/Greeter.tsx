import * as wasm from "rust-canvas-prototype";

export function Greeter() {
	return <button onClick={() => wasm.greet("lovely!")}>Touch me!</button>
}