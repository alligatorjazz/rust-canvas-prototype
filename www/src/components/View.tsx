import { useEffect, useRef, useState } from "react";
import { Universe } from "vite-rust-wasm";

export function View() {
	const [universe, setUniverse] = useState<Universe | null>(null);
	const [display, setDisplay] = useState("");

	const renderLoop = () => {
		if (universe) {
			setDisplay(universe?.render() ?? "");
			universe?.tick();
			requestAnimationFrame(renderLoop);
		}
	};

	useEffect(() => {
		if (!universe) { setUniverse(Universe.new()); }
		else { renderLoop(); }
	}, [universe])


	return (
		<pre onLoad={() => renderLoop()}>
			{display}
		</pre>
	);
}