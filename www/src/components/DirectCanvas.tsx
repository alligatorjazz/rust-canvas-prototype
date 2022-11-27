import { useEffect, useRef, useState } from "react";
import { CanvasSource } from "rust-canvas-prototype";
import { memory } from "../../../pkg/rust_canvas_prototype_bg.wasm";
import { useFPS } from "../hooks/useFPS";
import styles from "./DirectCanvas.module.css";

const getRenderLoop = (
	source: CanvasSource,
	ctx: CanvasRenderingContext2D,
	recordFPS?: () => void
) => {
	if (source && ctx) {
		const loop = () => {
			recordFPS ? recordFPS() : null;
			// debugger;
			const sourceDataPtr = source.data();

			const width = source.width();
			const height = source.height();

			const pixelData = new ImageData(
				new Uint8ClampedArray(
					memory.buffer,
					sourceDataPtr,
					source.width() * source.height() * 4
				), width, height);

			ctx.putImageData(pixelData, 0, 0)
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

	const { fps, update } = useFPS();

	// initialization
	useEffect(() => {
		if (!source) {
			console.log("loading source");
			const [width, height] = [385, 385]
			setSource(CanvasSource.new(width, height, new Uint8Array(width * height * 4)))
		}

		if (source && !ctx && canvasElement.current) {
			canvasElement.current.height = source.height();
			canvasElement.current.width = source.width();
			setCtx(canvasElement.current.getContext("2d"));
		}
	}, [source, ctx])


	useEffect(() => {
		if (initialized) {
			const renderLoop = getRenderLoop(source, ctx, update);
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
			<div className={styles.Dashboard}>
				<h5>FPS: {fps.latest}</h5>
			</div>
			<span className={styles.Controls}>
				<button onClick={() => source?.cover_in_blood()}>Splatter</button>
			</span>
			<canvas ref={canvasElement}></canvas>
		</div>
	)
}