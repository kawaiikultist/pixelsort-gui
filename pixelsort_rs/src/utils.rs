use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_test_image(width: u32, height: u32) -> image::DynamicImage {
    let mut img = image::ImageBuffer::<image::Rgba<u8>, Vec<u8>>::new(width, height);

    for y in 0..img.height() {
        for x in 0..img.width() {
            let n = (x + y) % 4;

            let color: [u8; 4] = match n {
                1 => [255, 0, 0, 255],
                2 => [0, 255, 0, 255],
                3 => [0, 0, 255, 255],
                // _ => [255, 255, 255, 255],
                _ => [255, 255, 255, 255],
            };

            img.put_pixel(x, y, image::Rgba(color));
        }
    }

    image::DynamicImage::ImageRgba8(img)
}


pub fn get_noise_image(width: u32, height: u32) -> image::DynamicImage {
    let mut s = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as u64;
    let img = image::ImageBuffer::<image::Rgba<u8>, Vec<u8>>::from_fn(width, height, |_, _| {
        s ^= s << 13; s ^= s >> 7; s ^= s << 17;
        image::Rgba([s as u8, (s >> 8) as u8, (s >> 16) as u8, 255])
    });


    image::DynamicImage::ImageRgba8(img)
}


pub fn get_uv_square(width: u32, height: u32) -> image::DynamicImage {
    let mut img = image::ImageBuffer::<image::Rgba<u8>, Vec<u8>>::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let (r, g, b) = (
                (x as f32 / width as f32) * 255.0,  // R
                (y as f32 / width as f32) * 255.0,  // G
                0,                                  // B
            );

            let color = image::Rgba([
                r as u8, g as u8, b as u8, 255
            ]);

            img.put_pixel(x, y, color);
        }
    }

    image::DynamicImage::ImageRgba8(img)

}
