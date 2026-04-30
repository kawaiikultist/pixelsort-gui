# pixelsort

A pixel sorting tool written in Rust. Sorts pixels in an image along configurable paths and by various color channels.

## Features

- Sort pixels by luminance, red, green, or blue channel
- Configurable sort angle
- Define thresholds to target certain pixels (ex: sort pixels with luma between 0 and 150)
- Reversible sorting
- Standard sorting paths like Linear & Radial
- Unique sorting paths like Blocks (more to come)

### Planned

- Additional sorting paths (radial, wave, etc)
- Additional sorting keys (hue, saturation, etc)
- Additional sort keys (hue, saturation, etc)
- GUI

## Project Structure

This is a Cargo workspace with two crates:

- **`pixelsort_rs`** — core library implementing the pixel sorting algorithm
- **`ps_gui`** — binary that uses the library (Currently lacks GUI)

## Usage

```rust
use pixelsort::prelude::*;

fn main() {
    let image = pixelsort::open("path/to/file.png")
        .expect("Could not open image!")
        .to_rgb8();


    let pxsorter = Pixelsorter::new(
        image,
        f64::to_radians(0.0),
        SortKey::Luma,
        SortPath::Radial { x_offset: 0, y_offset: -150 },
        false,
        Threshold::new(0.0, 0.5, SortKey::Luma, false),
    );

    let output = pxsorter.get_sorted_image();
    output.save("imgs/threshold_test.png")
        .expect("Could not save output!");
    println!("Image exported!");
}
```

## Building

```sh
cargo build --release
```

## License

MIT
