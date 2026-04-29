use pixelsort::{Pixelsorter, types::{SortKey, SortPath, Threshold}};

fn main() {
    let image = pixelsort::open("imgs/landscape.png")
        .expect("Could not open image!")
        .to_rgb8();

    // Radial Point should be 194
    // Full y is 930
    // (930 / 2) - 194 = 271

    let pxsort = Pixelsorter::new(
        image,
        f64::to_radians(0.0),
        SortKey::Luma,
        SortPath::Radial { x_offset: 0, y_offset: -150 },
        false,
        Threshold::new(0.0, 0.5, SortKey::Luma, false),
    );

    let output = pxsort.get_sorted_image();
    output.save("imgs/threshold_test.png")
        .expect("Could not save output!");
    println!("Image exported!");
}
