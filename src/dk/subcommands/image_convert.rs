use anyhow::{anyhow, Error};
use fehler::{throw, throws};
use image::{DynamicImage, GenericImageView, GrayImage, ImageBuffer, Luma};

use crate::dk::chart::Stitch;
use crate::dk::subcommands::chart_out;
use crate::dk::units::{Cols, Height, Rows, Width};
use crate::dk::util::make_knit_pathbuf;
use crate::dk::{args::ImageConvertArgs, chart::Chart};
use std::convert::TryFrom;

// returns (width, height).
fn image_size_preserving_ar(
    argwidth: Option<u16>,
    argheight: Option<u16>,
    imagewidth: u32,
    imageheight: u32,
) -> (u32, u32) {
    if let Some(argwidth) = argwidth {
        if let Some(argheight) = argheight {
            // Both args are provided, so just use the args.
            (u32::from(argwidth), u32::from(argheight))
        } else {
            // Only got a width, so compute the height.
            let ar = f64::from(imagewidth) / f64::from(imageheight);
            (u32::from(argwidth), (f64::from(argwidth) / ar) as u32)
        }
    } else if let Some(argheight) = argheight {
        // Only got a height, so compute the width.
        let ar = f64::from(imagewidth) / f64::from(imageheight);
        (((ar * f64::from(argheight)) as u32), u32::from(argheight))
    } else {
        // Didn't get either arg, so use values from the image.
        (imagewidth, imageheight)
    }
}

#[throws]
pub fn check_chart_size(chart_width: u32, chart_height: u32) {
    // ensure!
    if chart_height > u32::from(u16::MAX) {
        throw!(anyhow!(
            "Computed height, {}, exceeds max height of {}",
            chart_height,
            u16::MAX
        ));
    }
    // ensure!
    if chart_width > u32::from(u16::MAX) {
        throw!(anyhow!(
            "Computed width, {}, exceeds max width of {}",
            chart_width,
            u16::MAX
        ));
    }
}

#[throws]
fn convert_to_scaled_grayscale_image(image: &DynamicImage, width: u32, height: u32) -> GrayImage {
    image.grayscale().thumbnail_exact(width, height).into_luma()
}

#[throws]
fn convert_to_bw_image(image: &GrayImage, threshold: u8) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let mut output = ImageBuffer::new(image.width(), image.height());
    for (x, y, pixel) in output.enumerate_pixels_mut() {
        let source_color = image.get_pixel(x, y);
        let bw = if source_color.0[0] < threshold {
            image::Luma([0u8; 1])
        } else {
            image::Luma([255u8; 1])
        };
        *pixel = bw;
    }
    output
}

#[throws]
pub fn convert_bw_image_to_chart(image: &ImageBuffer<Luma<u8>, Vec<u8>>) -> Chart {
    let mut chart = Chart::new(
        Width::try_from(image.width())?,
        Height::try_from(image.height())?,
    );
    for (x, y, pixel) in image.enumerate_pixels() {
        // Pixel value will always be 0 or 255 at this point because we converted to bw image.
        let stitch = if pixel.0[0] == 0 {
            Stitch::Purl
        } else {
            Stitch::Knit
        };
        chart.set_stitch(Rows::try_from(y)?, Cols::try_from(x)?, stitch)?;
    }
    chart
}

#[throws]
pub fn image_convert(args: ImageConvertArgs) {
    let original_image = image::open(args.image_name)?;

    let (img_width, img_height) = original_image.dimensions();
    let (chart_width, chart_height) =
        image_size_preserving_ar(args.width, args.height, img_width, img_height);

    check_chart_size(chart_width, chart_height)?;

    let grayscale = convert_to_scaled_grayscale_image(&original_image, chart_width, chart_height)?;

    // TODO: allow specifying the desired grayscale threshold.
    let threshold = 128_u8;
    let bw = convert_to_bw_image(&grayscale, threshold)?;

    let chart = convert_bw_image_to_chart(&bw)?;

    let out_file_name = args
        .out_file_name
        .map(|pb| make_knit_pathbuf(pb, None))
        .transpose()?;
    chart_out(&out_file_name, &chart)?;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sizes() {
        // Both present.
        assert_eq!(
            (55, 33),
            image_size_preserving_ar(Some(55), Some(33), 128, 77)
        );

        // Both absent.
        assert_eq!((23, 86), image_size_preserving_ar(None, None, 23, 86));

        // Height missing. (Probably the most common case.)
        assert_eq!((60, 120), image_size_preserving_ar(Some(60), None, 20, 40));

        // Width missing.
        assert_eq!((45, 135), image_size_preserving_ar(None, Some(135), 20, 60));
    }
}
