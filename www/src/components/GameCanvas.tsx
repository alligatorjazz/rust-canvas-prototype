import { Dispatch, SetStateAction, useEffect, useRef, useState } from "react";
import { Universe } from "vite-rust-wasm";
import { ALIVE_COLOR, CELL_SIZE, DEAD_COLOR, GRID_COLOR } from "../config";
import { memory } from "vite-rust-wasm/vite_rust_wasm_bg.wasm";
import styles from "./GameCanvas.module.css";
import { useFPS } from "../hooks/useFPS";

const bitIsSet = (n: number, bitArray: Uint8Array) => {
	const byte = Math.floor(n / 8);
	const mask = 1 << (n % 8);
	return (bitArray[byte] & mask) === mask;
};

const drawGrid = (universe: Universe, ctx: CanvasRenderingContext2D) => {
	ctx.beginPath();
	ctx.strokeStyle = GRID_COLOR;

	// Vertical lines.
	for (let i = 0; i <= universe.width(); i++) {
		ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
		ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * universe.height() + 1);
	}

	// Horizontal lines.
	for (let j = 0; j <= universe.height(); j++) {
		ctx.moveTo(0, j * (CELL_SIZE + 1) + 1);
		ctx.lineTo((CELL_SIZE + 1) * universe.width() + 1, j * (CELL_SIZE + 1) + 1);
	}

	ctx.stroke();
}

const getIndex = (universe: Universe, row: number, column: number) => {
	return row * (universe?.width() ?? 0) + column;
};

const drawCells = (universe: Universe, ctx: CanvasRenderingContext2D) => {
	if (universe && ctx) {
		// the pointer of the cells in rust memory
		const cellsPtr = universe.cells();

		// gets the cells by taking all of the memory buffer, starting at the memory location of 
		// the cells (cellsptr). then, takes a snapshot of that memory of size width * height
		const cells = new Uint8Array(memory.buffer, cellsPtr, universe.width() * universe.height() / 8);

		ctx.beginPath();

		for (let row = 0; row < universe.height(); row++) {
			for (let col = 0; col < universe.width(); col++) {
				const idx = getIndex(universe, row, col);

				ctx.fillStyle = bitIsSet(idx, cells)
					? DEAD_COLOR
					: ALIVE_COLOR;

				ctx.fillRect(
					col * (CELL_SIZE + 1) + 1,
					row * (CELL_SIZE + 1) + 1,
					CELL_SIZE,
					CELL_SIZE
				);
			}
		}

		ctx.stroke();
	}
};



const getRenderLoop = (
	universe: Universe,
	ctx: CanvasRenderingContext2D,
	animationDispatch: Dispatch<SetStateAction<number | null | undefined>>,
	recordFPS?: () => void
) => {
	if (universe && ctx) {
		const loop = () => {
			recordFPS ? recordFPS() : null;
			// debugger;
			universe.tick();
			drawGrid(universe, ctx);
			drawCells(universe, ctx);
			const id = requestAnimationFrame(loop);
			animationDispatch(id);
		};

		return loop;
	}

	return null;
}


export function GameCanvas() {
	const [universe, setUniverse] = useState<Universe | null>(null);
	const [ctx, setCtx] = useState<CanvasRenderingContext2D | null>(null);

	const [paused, setPaused] = useState<boolean>(false);

	// undefined on init, null when paused
	const [animationId, setAnimationId] = useState<number | null>();
	const canvasElement = useRef<HTMLCanvasElement>(null);

	const initialized = universe && ctx;

	const { fps, update } = useFPS();

	// initialization
	useEffect(() => {
		if (!universe) {
			setUniverse(Universe.new());
		}

		// if canvas element is loaded and universe is initialized
		if (canvasElement.current && universe && !ctx) {
			// intializing canvas
			canvasElement.current.height = (CELL_SIZE + 1) * universe.height() + 1;
			canvasElement.current.width = (CELL_SIZE + 1) * universe.width() + 1;
			setCtx(canvasElement.current.getContext("2d"));
		}

	}, [universe, ctx])

	// pause / play handling
	useEffect(() => {
		if (initialized) {
			if (!paused) {
				console.log("starting loop");
				const renderLoop = getRenderLoop(universe, ctx, setAnimationId, update);
				renderLoop ? renderLoop() : console.error("render loop not generated");
			}

			if (paused && animationId) {
				cancelAnimationFrame(animationId);
				setAnimationId(null);
			}
		}
	}, [universe, ctx, paused])

	// interactibility
	useEffect(() => {
		if (canvasElement.current && initialized) {
			const canvas = canvasElement.current;
			canvas.addEventListener("click", event => {
				const boundingRect = canvas.getBoundingClientRect();

				const scaleX = canvas.width / boundingRect.width;
				const scaleY = canvas.height / boundingRect.height;

				const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
				const canvasTop = (event.clientY - boundingRect.top) * scaleY;

				const row = Math.min(Math.floor(canvasTop / (CELL_SIZE + 1)), universe.height() - 1);
				const col = Math.min(Math.floor(canvasLeft / (CELL_SIZE + 1)), universe.width() - 1);

				universe.toggle_cell(row, col);

				drawGrid(universe, ctx);
				drawCells(universe, ctx);
			});
		}

	}, [universe, ctx])

	return (
		<div className={styles.Container}>
			<div className={styles.Dashboard}>
				<h5>FPS: {fps.latest}</h5>
			</div>
			<span className={styles.Controls}>
				<button onClick={() => setPaused(prev => !prev)}>⏯</button>
			</span>
			<canvas ref={canvasElement}></canvas>
		</div>
	)
}