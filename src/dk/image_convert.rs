use anyhow::{anyhow, Error};
use fehler::{throw, throws};
use image::GenericImageView;

use crate::dk::chart::Stitch;
use crate::dk::{args::ImageConvertArgs, chart::Chart, thing::the_thing};
use std::path::PathBuf;

// returns (width, height).
fn image_size_preserving_ar(
    argwidth: Option<u16>,
    argheight: Option<u16>,
    imagewidth: u32,
    imageheight: u32,
) -> (u32, u32) {
    // If both args are provided, then just use the args.
    if argheight.is_some() && argwidth.is_some() {
        // unwrap: safe because is_some() was tested.
        (u32::from(argwidth.unwrap()), u32::from(argheight.unwrap()))
    } else if let Some(argwidth) = argwidth {
        let ar = f64::from(imagewidth) / f64::from(imageheight);
        (u32::from(argwidth), (f64::from(argwidth) / ar) as u32)
    } else if let Some(argheight) = argheight {
        let ar = f64::from(imagewidth) / f64::from(imageheight);
        (((ar * f64::from(argheight)) as u32), u32::from(argheight))
    } else {
        // No args were provided, so use the image size.
        (imagewidth, imageheight)
    }
}

#[throws]
pub fn image_convert(args: ImageConvertArgs) {
    for filename in args.files {
        let original_image = image::open(&filename)?;

        let (img_width, img_height) = original_image.dimensions();
        let (chart_width, chart_height) =
            image_size_preserving_ar(args.width, args.height, img_width, img_height);

        // TODO: range check the chart size.
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

        let luma = original_image
            .grayscale()
            .thumbnail_exact(chart_width, chart_height)
            .into_luma();

        // TODO: allow specifying the desired grayscale threshold.
        let threshold = 128;
        let mut output = image::ImageBuffer::new(chart_width, chart_height);
        for (x, y, pixel) in output.enumerate_pixels_mut() {
            let source_color = luma.get_pixel(x, y);
            let bw = if source_color.0[0] < threshold {
                image::Luma([0u8; 1])
            } else {
                image::Luma([255u8; 1])
            };
            *pixel = bw;
        }

        let mut chart = Chart::new(chart_width as u16, chart_height as u16);
        for (col, row, pixel) in output.enumerate_pixels() {
            // Pixel value will be either 0 or 1 by this point.
            // TODO: maybe consilidate these two loops.
            // - prevents creating an extra image, and
            // - makes this code a little more clear.
            let stitch = if pixel.0[0] == 0 {
                Stitch::Purl
            } else {
                Stitch::Knit
            };
            chart.set_stitch(row as u16, col as u16, stitch)?;
        }

        // unwrap: should be safe since we were able to open the file.
        let outfilename = PathBuf::from(filename.file_name().unwrap()).with_extension("png");
        the_thing(&outfilename.to_string_lossy(), &chart)?;

        let chartname = PathBuf::from(filename.file_name().unwrap()).with_extension("knit");
        chart.write_to_file(chartname)?;
    }
}

#[cfg(test)]
mod test {
    use crate::dk::image_convert::image_size_preserving_ar;

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
