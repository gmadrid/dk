use anyhow::Error;
use fehler::throws;
use structopt::StructOpt;

mod dk;

macro_rules! dispatch {
    ($(($command:ident, $proc:ident)),*) => {
        let subcommand = dk::Dk::from_args();
        match subcommand {
            $(
            dk::Dk::$command { args } => { dk::$proc(args)?; }
            )*
        }
    }
}

#[throws]
fn main() {
    dispatch!(
        (ImageConvert, image_convert),
        (Knitchart, knitchart),
        (Left, left),
        (Reflect, reflect),
        (Right, right),
        (Split, split),
        (Trim, trim),
        (Zip, zip)
    );
}
