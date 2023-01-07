import { useEffect, useRef, useState } from "react";
import { CanvasSource } from "rust-canvas-prototype/rust_canvas_prototype";
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


const loadCanvas = (
	width: number,
	height: number,
	ctx: WebGL2RenderingContext | null,
) => {
	if (ctx) {
		console.log("loading CanvasSource");
		console.assert(ctx ? true : false);
		return CanvasSource.new(
			width, height,
			ctx as WebGL2RenderingContext
		);
	}

	return null;
}
export function DirectCanvas() {
	const { ctx, canvasRef } = useCanvasContext();
	const [source, setSource] = useState<CanvasSource | null>(null);

	const [width, height] = [640, 480];

	return (
		<div className={styles.Container}>
			<div className={styles.Dashboard}>
				<h5>FPS: N/A</h5>
			</div>
			<span className={styles.Controls}>
				<button onClick={() => setSource(loadCanvas(
					width,
					height,
					ctx
				))}>Splatter</button>
			</span>
			<canvas ref={canvasRef} width={width} height={height}></canvas>
		</div>
	)
}