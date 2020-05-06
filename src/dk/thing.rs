use anyhow::Error;
use crate::dk::{
    chart::{Chart, Stitch}
};
use fehler::throws;
use graphics::{grid::Grid, line::Line};
use graphics_buffer::{RenderBuffer, IDENTITY};
use std::path::PathBuf;
use std::io::Write;

#[throws]
pub fn the_thing(filename: &str, chart: &Chart) {
    let background_color = [ 1.0, 1.0, 1.0, 1.0 ];
    let dot_size = 10.0;
    let cell_size = 15.0;
    let grid_color = [ 0.0, 0.0, 0.0, 1.0];

    let rows = u32::from(chart.rows());
    let cols = u32::from(chart.cols());

    let cell_size_int = cell_size as u32;
    let mut buffer = RenderBuffer::new(cols * cell_size_int, rows * cell_size_int);

    buffer.clear(background_color);

    let grid = Grid {
        cols,
        rows,
        units: f64::from(cell_size),
    };
    let line = Line::new(grid_color, 1.0);
    grid.draw(&line, &Default::default(), IDENTITY, &mut buffer);

    for cell in grid.cells() {
        let (col, row) = cell;
        let cell_pos = grid.cell_position(cell);

        let center_y = cell_pos[0] + f64::from(cell_size) / 2.0;
        let center_x = cell_pos[1] + f64::from(cell_size) / 2.0;

        let stitch = chart.stitch(row as u16, col as u16)?;

        if let Stitch::Purl = stitch {
            let rectangle = [
            center_y - dot_size / 2.0,
            center_x - dot_size / 2.0,
            dot_size,
            dot_size];
            std::io::stdout().flush()?;  // TODO: what is this?
            graphics::ellipse([0.1, 0.1, 0.1, 1.0], rectangle, IDENTITY, &mut buffer);
        }
        print!("\r{:?}                ", cell);
    }
    print!("\r");

    // TODO: path buf from in generic function header.
    let outfile = PathBuf::from(filename).with_extension("png");
    dbg!("Writing buffer...");
    buffer.save(outfile)?;
    dbg!("Done!");
}