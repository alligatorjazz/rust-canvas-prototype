import { useEffect, useRef, useState } from "react";
import { CanvasSource } from "rust-canvas-prototype";
import { memory } from "rust-canvas-prototype/rust_canvas_prototype_bg.wasm";
import styles from "./DirectCanvas.module.css";

const getRenderLoop = (
	source: CanvasSource,
	ctx: CanvasRenderingContext2D,
) => {
	if (source && ctx) {
		const loop = () => {
			const sourceDataPtr = source.data();
			
			const width = source.width();
			const height = source.height();
			const regionSize = width * height * 4;
			
			
			const pixelData = new Uint8ClampedArray(
				memory.buffer,
				sourceDataPtr,
				regionSize
			)

			const imageData = new ImageData(pixelData, width, height);

			ctx.putImageData(imageData, 0, 0)
		};

		return loop;
	}

	return null;
}


export function DirectCanvas() {
	const [source, setSource] = useState<CanvasSource>();
	const [ctx, setCtx] = useState<CanvasRenderingContext2D | null>(null);

	const [paused, setPaused] = useState<boolean>(false);

	// undefined on init, null when paused
	const [animationId, setAnimationId] = useState<number>(0);
	const canvasElement = useRef<HTMLCanvasElement>(null);

	const initialized = source && ctx;

	// initialization
	useEffect(() => {
		if (!source) {
			console.log("loading source");
			let [width, height] = [100, 100];
			// uncomment below to cause error
			// [width, height] = [358, 358]
			setSource(CanvasSource.new(width, height, new Uint8Array([])))
		}

		if (source && !ctx && canvasElement.current) {
			canvasElement.current.height = source.height();
			canvasElement.current.width = source.width();
			setCtx(canvasElement.current.getContext("2d"));
		}
	}, [source, ctx])


	useEffect(() => {
		if (initialized) {
			const renderLoop = getRenderLoop(source, ctx);
			if (renderLoop) {
				renderLoop();
				setTimeout(() => {
					setAnimationId(prev => prev + 1);
				}, 10)
			}
		}
	}, [source, ctx, animationId]);

	return (
		<div className={styles.Container}>
			<span className={styles.Controls}>
				<button onClick={() => source?.cover_in_blood()}>Splatter</button>
			</span>
			<canvas ref={canvasElement}></canvas>
		</div>
	)
}