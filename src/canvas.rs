// Copyright 2022 Lukasz Janyst <lukasz@jany.st>
// Licensed under the MIT license, see the LICENSE file for details.

use crate::tuple::{color, Tuple};

use std::vec::Vec;

pub struct Canvas {
    width: usize,
    height: usize,
    data: Vec<Vec<Tuple>>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        let mut c = Canvas {
            width: width,
            height: height,
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
}
