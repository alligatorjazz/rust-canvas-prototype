# Rust Canvas Prototype - Minimal Reproducible Example

The problem is as follows:
- I am trying to create a struct that has a large enough data buffer to hold HTML5 canvas ImageData larger than 64 x 64 pixels.
- The function works correctly for sizes of ~100x100 or less, but once the total area begins to exceed that JS throws the following error:

```
Uncaught RangeError: attempting to construct out-of-bounds Uint8ClampedArray on ArrayBuffer
loop DirectCanvas.tsx:23
DirectCanvas DirectCanvas.tsx:77
...
```

- [Preliminary research](https://www.reddit.com/r/rust/comments/872fc4/how_to_increase_the_stack_size/) suggests that it is a stack size problem on Rust's end, but attempts to increase the stack size in the config.toml throw errors of their own: 
```= note: rust-lld: error: unknown argument: -Wl,-zstack-size=29491200```

**How do I allocate a large enough stack size to paint to canvases larger than 100x100?**

