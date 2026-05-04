pub fn get_test_image(width: u32, height: u32) -> image::DynamicImage {
    let mut img = image::ImageBuffer::<image::Rgba<u8>, Vec<u8>>::new(width, height);

    for y in 0..img.height() {
        for x in 0..img.width() {
            let n = (x + y) % 4;

            let color: [u8; 4] = match n {
                1 => [255, 0, 0, 255],
                2 => [0, 255, 0, 255],
                3 => [0, 0, 255, 255],
                _ => [255, 255, 255, 255],
            };

            img.put_pixel(x, y, image::Rgba(color));
        }
    }

    image::DynamicImage::ImageRgba8(img)
}
