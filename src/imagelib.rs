use image::{
    ImageBuffer,
    Rgba
};

pub type ImageBufferType = ImageBuffer<Rgba<u8>, Vec<u8>>;

pub fn create_image_buffer_from_bytes(
    bytes: &[u8]
) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let dynamic_image = image::load_from_memory(bytes).unwrap();
    dynamic_image.to_rgba8()
}
