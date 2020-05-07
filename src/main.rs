use anyhow::Error;
use fehler::throws;
use structopt::StructOpt;

mod dk;

#[throws]
fn main() {
    let subcommand = dk::Dk::from_args();
    match subcommand {
        dk::Dk::ImageConvert { args } => {
            dk::image_convert(args)?;
        }
        dk::Dk::Split { args } => {
            dk::split(args)?;
        }
        dk::Dk::Trim { args } => {
            dk::trim(args)?;
        }
        dk::Dk::Reflect { args } => {
            dk::reflect(args)?;
        }
    };
}
