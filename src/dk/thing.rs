use crate::dk::chart::{Chart, Stitch};
use anyhow::Error;
use fehler::throws;
use image::{Rgb, RgbImage};
use imageproc::drawing::{draw_filled_circle_mut, draw_filled_rect_mut, draw_line_segment_mut};
use imageproc::rect::Rect;
use std::path::Path;

#[throws]
pub fn the_thing(filename: impl AsRef<Path>, chart: &Chart) {
    let cell_size = 15;
    let dot_size = 10;
    let background_color = Rgb([255, 255, 255]);
    let grid_color = Rgb([0, 0, 0]);

    let rows = u32::from(chart.rows());
    let cols = u32::from(chart.cols());
    let mut img = RgbImage::new(cols * cell_size, rows * cell_size);

    draw_filled_rect_mut(
        &mut img,
        Rect::at(0, 0).of_size(cols * cell_size, rows * cell_size),
        background_color,
    );

    for row in 0..=rows {
        let row_offset = (row * cell_size) as f32;
        draw_line_segment_mut(
            &mut img,
            (0.0 - 0.5, row_offset - 0.5),
            ((cols * cell_size) as f32 - 0.5, row_offset - 0.5),
            grid_color,
        );
    }

    for col in 0..=cols {
        let col_offset = (col * cell_size) as f32;
        draw_line_segment_mut(
            &mut img,
            (col_offset - 0.5, 0.0 - 0.5),
            (col_offset - 0.5, (rows * cell_size) as f32 - 0.5),
            grid_color,
        );
    }

    for row in 0..rows {
        for col in 0..cols {
            if let Stitch::Purl = chart.stitch(row as u16, col as u16)? {
                let cell_x = col * cell_size;
                let cell_y = row * cell_size;

                draw_filled_circle_mut(
                    &mut img,
                    (
                        (cell_x + cell_size / 2) as i32,
                        (cell_y + cell_size / 2) as i32,
                    ),
                    dot_size / 2,
                    grid_color,
                );
            }
        }
    }

    img.save(filename)?;
}
