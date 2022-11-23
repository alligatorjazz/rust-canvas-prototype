import { useEffect, useState } from "react";

type FPSReport = {
	latest: number;
	last100avg?: number;
	last100min?: number;
	last100max?: number;
}

type Stream = (output: FPSReport) => unknown;
class FPSMonitor {
	frames: number[]
	lastFrameTimeStamp: number;
	report?: FPSReport;

	constructor() {
		this.frames = [];
		this.lastFrameTimeStamp = performance.now();
	}

	static check(monitor: FPSMonitor) {
		return monitor.report;
	}

	static record(monitor: FPSMonitor) {
	}
};

export function useFPS() {
	const [lastFrameTimeStamp, setLastFrameTimeStamp] = useState(0);
	const [frames, setFrames] = useState<number[]>([]);
	const [fps, setFPS] = useState<FPSReport>({latest: 0});
	const [update, setUpdate] = useState<number>(0);

	useEffect(() => {
		// of frames per second.
		const now = performance.now();
		// debugger;
		// console.log(this);
		const delta = now - lastFrameTimeStamp;
		setLastFrameTimeStamp(now);
		const deltaReading = 1 / delta * 1000;

		// Save only the latest 100 timings.
		setFrames(prev => prev.concat([deltaReading]))
		if (frames.length > 100) {
			setFrames(prev => prev.slice(1))
		}

		// Find the max, min, and mean of our 100 latest timings.
		let min = Infinity;
		let max = -Infinity;
		let sum = 0;
		for (let i = 0; i < frames.length; i++) {
			sum += frames[i];
			min = Math.min(frames[i], min);
			max = Math.max(frames[i], max);
		}
		let mean = sum / frames.length;

		const report = {
			latest: Math.round(deltaReading),
			last100avg: Math.round(mean),
			last100min: Math.round(min),
			last100max: Math.round(max)
		}

		setFPS(report)
	}, [update])

	return {fps, update: () => setUpdate(prev => prev + 1)}
}