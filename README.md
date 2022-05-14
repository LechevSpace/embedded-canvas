# `embedded-canvas`
> canvas - a piece of cloth backed or framed as a surface for a painting


Draw anything with ease on a canvas before drawing it on an embedded display, compatible with [`embedded-graphics`].

- `no_std`
- Requires `alloc`

- [`embedded-graphics`]: https://crates.io/crates/embedded-graphics
## How to work with canvases

There are 2 main structs you can work with:

### `Canvas`

A canvas which you can draw on with origin `Point::zero()`. This canvas location is not set for the provided display. Draw anything and then decide where to place it on the display.

### `CanvasAt`

Canvas ready to be drawn on the display at specified location.

## License

Licensed under either of [Apache License, Version 2.0](./LICENSE-APACHE)
or [MIT license](./LICENSE-MIT) at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this project by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.