use pixelsort::prelude::*;

fn main() {
    let image = pixelsort::open("imgs/landscape.png")
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
