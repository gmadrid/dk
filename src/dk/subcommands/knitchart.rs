use crate::dk::{args::KnitchartArgs, chart::Chart, thing::the_thing, subcommands::chart_in};
use anyhow::Error;
use fehler::throws;

#[throws]
pub fn knitchart(args: KnitchartArgs) {
    let chart = chart_in(&args.in_file_name)?;

    // TODO: use infilename if available and not provided.
    let mut out_file = args.out_file_name.unwrap_or_else(|| "chart.png".into());
    out_file.set_extension("png");
    the_thing(&out_file, &chart)?;
}
