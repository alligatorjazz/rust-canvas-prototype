import * as wasm from "vite-rust-wasm";

export function Greeter() {
	return <button onClick={() => wasm.greet("lovely!")}>Touch me!</button>
}