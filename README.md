# Rust Canvas Prototype
**A Rust webassembly starter project with vite as the build tool and React as the frontend library..**

Based largely on the Game of Life [rustwasm](https://rustwasm.github.io/) walkthrough found [here](https://rustwasm.github.io/docs/book/game-of-life/implementing.html). More detailed implementation specs for the original project can be found in ORIGNAL_README.md.

## Components
The project comes with two built-in components to test WASM rendering capabilities.

`<GameCanvas />` is a Game of Life implementation with a built-in pause / play button. The computation that generates the universe is all done on the rust-side, while the actual work of drawing grid lines and squares to update the canvas display is done in JS. The code for this component was pulled from the [vite-rust-wasm](https://github.com/alligatorjazz/vite-rust-wasm) repo.

TODO: `<DirectCanvas/ >` is a 2D grid where an image can be clicked and drawn across the grid. The image can be changed to one that the user uploads. This component has *all* of the rendering done on the Rust side, with JS simply reading the image data from a memory buffer.
