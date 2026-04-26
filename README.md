# pixelsort

A pixel sorting tool written in Rust. Sorts pixels in an image along configurable paths and by various color channels.

## Features

- Sort pixels by luminance, red, green, or blue channel
- Configurable sort angle
- Forward or reverse sort order
- Linear pixel sorting

### Planned

- Additional sorting paths (radial, wave, etc)
- Additional sorting keys (hue, saturation, etc)
- Thresholds (ex: Sort only pixels with a luma between X & Y)
- Additional sort keys (hue, saturation, etc)
- GUI

## Project Structure

This is a Cargo workspace with two crates:

- **`pixelsort_rs`** — core library implementing the pixel sorting algorithm
- **`ps_gui`** — binary that uses the library (Currently lacks GUI)

## Usage

```rust
use pixelsort::{Pixelsorter, types::{SortKey, SortPath}};

let image = pixelsort::open("input.png").unwrap().to_rgb8();

let sorter = Pixelsorter::new(
    image,
    0.0,          // angle in radians
    SortKey::Luma,
    SortPath::Linear,
    false,        // reverse
);

let output = sorter.get_sorted_image();
output.save("output.png").unwrap();
```

## Building

```sh
cargo build --release
```

## License

MIT
