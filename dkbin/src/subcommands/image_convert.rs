use crate::args::commandargs::*;

#[throws]
pub fn image_convert(args: ImageConvertArgs) {
    let original_image = image::open(args.image_name)?;

    let chart = image_convert_img(&original_image, args.height, args.width)?;

    let out_file_name = args
        .out_file_name
        .map(|pb| make_knit_pathbuf(pb, None))
        .transpose()?;
    chart_out(&out_file_name, &chart)?;
}

