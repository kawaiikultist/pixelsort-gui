use pixelsort::{Pixelsorter, types::{SortKey, SortPath}};

fn main() {
    let image = pixelsort::open("imgs/landscape.png")
        .expect("Could not open image!")
        .to_rgb8();

    let pxsort = Pixelsorter::new(
        image, 0.0, SortKey::Luma, SortPath::Linear, false
    );

    let output = pxsort.get_sorted_image();
    output.save("imgs/output.png")
        .expect("Could not save output!");
}
