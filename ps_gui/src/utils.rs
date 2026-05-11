use iced::widget::image::Handle;
use pixelsort::image::DynamicImage;

pub fn get_image_handle(image: &DynamicImage) -> Handle {
    Handle::from_rgba(image.width(), image.height(), image.to_rgba8().into_raw())
}
