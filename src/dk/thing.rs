use crate::dk::{
    chart::Chart,
    units::{Height, Width},
};
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

    let rows = chart.rows();
    let cols = chart.cols();
    let mut img = RgbImage::new(u32::from(Width::from(cols)) * cell_size,
                                u32::from(Height::from(rows)) * cell_size);

    draw_filled_rect_mut(
        &mut img,
        Rect::at(0, 0).of_size(u32::from(cols) * cell_size, u32::from(rows) * cell_size),
        background_color,
    );

    for row in rows + 1 {
        let row_offset = (row * cell_size) as f32;
        draw_line_segment_mut(
            &mut img,
            (0.0 - 0.5, row_offset - 0.5),
            ((cols * cell_size) as f32 - 0.5, row_offset - 0.5),
            grid_color,
        );
    }

    for col in cols + 1 {
        let col_offset = (col * cell_size) as f32;
        draw_line_segment_mut(
            &mut img,
            (col_offset - 0.5, 0.0 - 0.5),
            (col_offset - 0.5, (rows * cell_size) as f32 - 0.5),
            grid_color,
        );
    }

    for row in rows {
        for col in cols {
            if chart.stitch(row, col)?.symbol() == '*' {
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

    img.save(filename.as_ref())?;
}
