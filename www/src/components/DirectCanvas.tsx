import { useEffect, useRef, useState } from "react";
import { CanvasSource } from "rust-canvas-prototype/rust_canvas_prototype";
import { memory } from "rust-canvas-prototype/rust_canvas_prototype_bg.wasm";
import { useCanvasContext } from "../hooks/useCanvasContext";
import { useFPS } from "../hooks/useFPS";
import styles from "./DirectCanvas.module.css";

// const getRenderLoop = (
// 	source: CanvasSource,
// 	ctx: WebGL2RenderingContext,
// 	recordFPS?: () => void
// ) => {
// 	if (source && ctx) {
// 		const loop = () => {
// 			recordFPS ? recordFPS() : null;
// 			// debugger;
// 			const sourceDataPtr = source.data();

// 			const width = source.width();
// 			const height = source.height();

// 			const pixelData = new ImageData(
// 				new Uint8ClampedArray(
// 					memory.buffer,
// 					sourceDataPtr,
// 					source.width() * source.height() * 4
// 				), width, height);

// 			// ctx.putImageData(pixelData, 0, 0)
// 		};

// 		return loop;
// 	}

// 	return null;
// }


export function DirectCanvas() {
	const { ctx, canvasRef } = useCanvasContext("webgl2");
	const [source, setSource] = useState<CanvasSource | null>(null);
	// initialization
	useEffect(() => {
		if (ctx) {
			console.log("loading source");
			const [width, height] = [385, 385];
			console.assert(ctx ? true : false);
			setSource(CanvasSource.new(
				width, height,
				new Uint8Array(width * height * 4),
				ctx as WebGL2RenderingContext
			));
		}
	}, [ctx]);

	return (
		<div className={styles.Container}>
			<div className={styles.Dashboard}>
				<h5>FPS: N/A</h5>
			</div>
			<span className={styles.Controls}>
				<button onClick={() => source?.init()}>Splatter</button>
			</span>
			<canvas ref={canvasRef}></canvas>
		</div>
	)
}