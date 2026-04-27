use pixelsort::{Pixelsorter, types::{SortKey, SortPath}};

fn main() {
    let image = pixelsort::open("imgs/landscape.png")
        .expect("Could not open image!")
        .to_rgb8();

    let pxsort = Pixelsorter::new(
        image,
        f64::to_radians(180.0),
        SortKey::Luma,
        SortPath::Linear,
        false
    );

    let output = pxsort.get_sorted_image();
    output.save("imgs/linear.png")
        .expect("Could not save output!");
    println!("Image exported!");
}
