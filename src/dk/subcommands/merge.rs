use crate::dk::{
    args::MergeArgs,
    chart::{Chart, Stitch},
    subcommands::{chart_in, chart_out},
};
use anyhow::Error;
use fehler::throws;

#[throws]
pub fn merge(args: MergeArgs) {
    let left = chart_in(&Some(&args.left))?;
    let right = chart_in(&Some(&args.right))?;

    let merged = merge_charts(&left, &right)?;
    chart_out(&args.out_file_name, &merged)?;
}

#[throws]
fn merge_charts(left: &Chart, right: &Chart) -> Chart {
    todo!()
}
