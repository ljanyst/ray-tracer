// Copyright 2022 Lukasz Janyst <lukasz@jany.st>
// Licensed under the MIT license, see the LICENSE file for details.

use crate::tuple::{color, Tuple};

use std::fs;

pub struct Canvas {
    width: usize,
    height: usize,
    data: Vec<Vec<Tuple>>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        let mut c = Canvas {
            width,
            height,
            data: Vec::new(),
        };
        let mut col = Vec::new();
        col.resize(height, color(0.0, 0.0, 0.0));
        c.data.resize(width, col);
        c
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    fn check_bounds(&self, x: usize, y: usize) {
        if x >= self.width {
            panic!(
                "x is out of bounds: {} (canvas dims: {}x{})",
                x, self.width, self.height
            );
        }
        if y >= self.height {
            panic!(
                "y is out of bounds: {} (canvas dims: {}x{})",
                x, self.width, self.height
            );
        }
    }

    pub fn set(&mut self, x: usize, y: usize, color: &Tuple) {
        self.check_bounds(x, y);
        self.data[x][y] = *color;
    }

    pub fn at(&self, x: usize, y: usize) -> Tuple {
        self.check_bounds(x, y);
        self.data[x][y]
    }

    pub fn ppm(&self) -> String {
        let mut ppm = String::new();
        // Header
        ppm.push_str("P3\n");
        ppm.push_str(format!("{} {}\n", self.width, self.height).as_str());
        ppm.push_str("255\n");

        // Pixel data
        for j in 0..self.height {
            let mut row = String::new();
            for i in 0..self.width {
                let pixel = self.at(i, j);
                ppm = add_pixel_data(ppm, &mut row, pixel.r());
                ppm = add_pixel_data(ppm, &mut row, pixel.g());
                ppm = add_pixel_data(ppm, &mut row, pixel.b());
            }
            if !row.is_empty() {
                ppm.push_str(row.as_str());
                ppm.push('\n');
            }
        }
        ppm
    }

    pub fn save(&self, file_path: &str) -> std::io::Result<()> {
        fs::write(file_path, self.ppm().as_bytes())?;
        Ok(())
    }
}

fn add_pixel_data(mut ppm: String, row: &mut String, color: f64) -> String {
    let c = format!("{}", (color * 256.0) as u8);

    if row.len() + 1 + c.len() > 70 {
        ppm.push_str(row.as_str());
        ppm.push('\n');
        row.clear();
    }

    if !row.is_empty() {
        row.push(' ');
    }
    row.push_str(c.as_str());

    ppm
}
