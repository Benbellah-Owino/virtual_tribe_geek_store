use std::io::BufWriter;

// use image::codecs::png::PngEncoder;
// use image::{ImageEncoder, ImageReader};

// use fast_image_resize::images::Image;
// use fast_image_resize::{IntoImageView, Resizer};

#[derive(Clone, Debug)]
pub enum Error {
    ResizingError,
}
// pub fn resize_image(dest: String, dst_width: u32, dst_height: u32) -> Result<(), Error> {
//     let src_image = ImageReader::open(dest).unwrap().decode().unwrap();

//     // Create container for data of destination image

//     let mut dst_image = Image::new(dst_width, dst_height, src_image.pixel_type().unwrap());

//     // Create Resizer instance and resize source image
//     // into buffer of destination image
//     let mut resizer = Resizer::new();
//     resizer.resize(&src_image, &mut dst_image, None).unwrap();

//     // Write destination image as PNG-file
//     let mut result_buf = BufWriter::new(Vec::new());
//     PngEncoder::new(&mut result_buf)
//         .write_image(
//             dst_image.buffer(),
//             dst_width,
//             dst_height,
//             src_image.color().into(),
//         )
//         .unwrap();
//     Ok(())
// }
