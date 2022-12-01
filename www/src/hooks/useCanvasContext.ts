import { useCallback,  useState } from "react";

type ContextType = CanvasRenderingContext2D | WebGLRenderingContext | WebGL2RenderingContext | ImageBitmapRenderingContext;

export function useCanvasContext(contextType: "2d" | "webgl" | "webgl2"): {
	ctx: ContextType | null,
	canvasRef: (node: HTMLCanvasElement) => HTMLCanvasElement
} {
	const [ctx, setCtx] = useState<ContextType | null>(null);

	const canvasRef = useCallback((node: HTMLCanvasElement) => {
		// Check if a node is actually passed. Otherwise node would be null.
		if (!node)  {
			console.error("canvasRef could not be initialized. This could be because the element it was attached to no longer exists.")
		}

		if (node.tagName != "CANVAS") {
			console.error("canvasRef passed to a non-canvas element.")
		}

		setCtx(node.getContext(contextType))
		return node;
	}, [])
	
	return { ctx, canvasRef }

}