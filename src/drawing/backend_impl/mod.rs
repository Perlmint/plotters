#[cfg(all(not(target_arch = "wasm32"), feature = "svg"))]
mod svg;
#[cfg(all(not(target_arch = "wasm32"), feature = "svg"))]
pub use self::svg::SVGBackend;

#[cfg(all(not(target_arch = "wasm32"), feature = "image"))]
mod bitmap;
#[cfg(all(not(target_arch = "wasm32"), feature = "image"))]
pub use bitmap::BitMapBackend;

#[cfg(target_arch = "wasm32")]
mod canvas;
#[cfg(target_arch = "wasm32")]
pub use canvas::CanvasBackend;

#[cfg(test)]
mod mocked;
#[cfg(test)]
pub use mocked::{create_mocked_drawing_area, MockedBackend};
