pub fn get_test_image(width: u32, height: u32) -> image::ImageBuffer<image::Rgb<u8>, Vec<u8>> {
    let mut img = image::ImageBuffer::<image::Rgb<u8>, Vec<u8>>::new(width, height);

    for y in 0..img.height() {
        for x in 0..img.width() {
            let n = (x + y) % 4;

            let color: [u8; 3] = match n {
                1 => [255, 0, 0],
                2 => [0, 255, 0],
                3 => [0, 0, 255],
                _ => [255, 255, 255],
            };

            img.put_pixel(x, y, image::Rgb(color));
        }
    }

    img
}
