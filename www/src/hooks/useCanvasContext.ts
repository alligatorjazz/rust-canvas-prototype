import { useCallback,  useState } from "react";



export function useCanvasContext(): {
	ctx: WebGL2RenderingContext | null,
	canvasRef: (node: HTMLCanvasElement) => HTMLCanvasElement
} {
	const [ctx, setCtx] = useState<WebGL2RenderingContext | null>(null);

	const canvasRef = useCallback((node: HTMLCanvasElement) => {
		// Check if a node is actually passed. Otherwise node would be null.
		if (!node)  {
			console.error("canvasRef could not be initialized. This could be because the element it was attached to no longer exists.")
		}

		if (node.tagName != "CANVAS") {
			console.error("canvasRef passed to a non-canvas element.")
		}

		setCtx(node.getContext("webgl2"))
		return node;
	}, [])
	
	return { ctx, canvasRef }

}