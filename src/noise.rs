// Copyright 2022 Lukasz Janyst <lukasz@jany.st>
// Licensed under the MIT license, see the LICENSE file for details.

use crate::tuple::Tuple;

#[derive(Debug, Clone)]
pub struct Noise {
    perms: Vec<u8>,
}

/// Perlin noise
/// Based on: http://adrianb.io/2014/08/09/perlinnoise.html
impl Noise {
    pub fn new() -> Noise {
        Noise {
            perms: vec![
                151, 160, 137, 91, 90, 15, 131, 13, 201, 95, 96, 53, 194, 233, 7, 225, 140, 36,
                103, 30, 69, 142, 8, 99, 37, 240, 21, 10, 23, 190, 6, 148, 247, 120, 234, 75, 0,
                26, 197, 62, 94, 252, 219, 203, 117, 35, 11, 32, 57, 177, 33, 88, 237, 149, 56, 87,
                174, 20, 125, 136, 171, 168, 68, 175, 74, 165, 71, 134, 139, 48, 27, 166, 77, 146,
                158, 231, 83, 111, 229, 122, 60, 211, 133, 230, 220, 105, 92, 41, 55, 46, 245, 40,
                244, 102, 143, 54, 65, 25, 63, 161, 1, 216, 80, 73, 209, 76, 132, 187, 208, 89, 18,
                169, 200, 196, 135, 130, 116, 188, 159, 86, 164, 100, 109, 198, 173, 186, 3, 64,
                52, 217, 226, 250, 124, 123, 5, 202, 38, 147, 118, 126, 255, 82, 85, 212, 207, 206,
                59, 227, 47, 16, 58, 17, 182, 189, 28, 42, 223, 183, 170, 213, 119, 248, 152, 2,
                44, 154, 163, 70, 221, 153, 101, 155, 167, 43, 172, 9, 129, 22, 39, 253, 19, 98,
                108, 110, 79, 113, 224, 232, 178, 185, 112, 104, 218, 246, 97, 228, 251, 34, 242,
                193, 238, 210, 144, 12, 191, 179, 162, 241, 81, 51, 145, 235, 249, 14, 239, 107,
                49, 192, 214, 31, 181, 199, 106, 157, 184, 84, 204, 176, 115, 121, 50, 45, 127, 4,
                150, 254, 138, 236, 205, 93, 222, 114, 67, 29, 24, 72, 243, 141, 128, 195, 78, 66,
                215, 61, 156, 180,
            ],
        }
    }

    fn p(&self, mut i: usize) -> usize {
        i = i % 255;
        self.perms[i] as usize
    }

    pub fn noise(&self, pt: Tuple) -> f64 {
        let xi = (pt.x().floor() as isize & 255) as usize;
        let yi = (pt.y().floor() as isize & 255) as usize;
        let zi = (pt.z().floor() as isize & 255) as usize;

        let aaa = self.p(self.p(self.p(xi) + yi) + zi) as u8;
        let aba = self.p(self.p(self.p(xi) + yi + 1) + zi) as u8;
        let aab = self.p(self.p(self.p(xi) + yi) + zi + 1) as u8;
        let abb = self.p(self.p(self.p(xi) + yi + 1) + zi + 1) as u8;
        let baa = self.p(self.p(self.p(xi + 1) + yi) + zi) as u8;
        let bba = self.p(self.p(self.p(xi + 1) + yi + 1) + zi) as u8;
        let bab = self.p(self.p(self.p(xi + 1) + yi) + zi + 1) as u8;
        let bbb = self.p(self.p(self.p(xi + 1) + yi + 1) + zi + 1) as u8;

        let xf = pt.x() - pt.x().floor();
        let yf = pt.y() - pt.y().floor();
        let zf = pt.z() - pt.z().floor();

        let u = fade(xf);
        let v = fade(yf);
        let w = fade(zf);

        let x1 = lerp(grad(aaa, xf, yf, zf), grad(baa, xf - 1.0, yf, zf), u);
        let x2 = lerp(
            grad(aba, xf, yf - 1.0, zf),
            grad(bba, xf - 1.0, yf - 1.0, zf),
            u,
        );
        let y1 = lerp(x1, x2, v);

        let x1 = lerp(
            grad(aab, xf, yf, zf - 1.0),
            grad(bab, xf - 1.0, yf, zf - 1.0),
            u,
        );
        let x2 = lerp(
            grad(abb, xf, yf - 1.0, zf - 1.0),
            grad(bbb, xf - 1.0, yf - 1.0, zf - 1.0),
            u,
        );
        let y2 = lerp(x1, x2, v);

        (lerp(y1, y2, w) + 1.0) / 2.0
    }

    pub fn octave_noise(&self, pt: Tuple, octaves: u8, persistence: f64) -> f64 {
        let mut total = 0.0_f64;
        let mut frequency = 1.0_f64;
        let mut amplitude = 1.0_f64;
        let mut max_value = 0.0_f64;
        let mut i = 0_u8;
        while i < octaves {
            total += self.noise(pt * frequency) * amplitude;
            max_value += amplitude;
            amplitude *= persistence;
            frequency *= 2.0;
            i += 1;
        }

        return total / max_value;
    }
}

fn fade(t: f64) -> f64 {
    return t * t * t * (t * (t * 6.0 - 15.0) + 10.0); // 6t^5 - 15t^4 + 10t^3
}

fn grad(hash: u8, x: f64, y: f64, z: f64) -> f64 {
    match hash & 0xf {
        0x0 => x + y,
        0x1 => -x + y,
        0x2 => x - y,
        0x3 => -x - y,
        0x4 => x + z,
        0x5 => -x + z,
        0x6 => x - z,
        0x7 => -x - z,
        0x8 => y + z,
        0x9 => -y + z,
        0xa => y - z,
        0xb => -y - z,
        0xc => y + x,
        0xd => -y + z,
        0xe => y - x,
        0xf => -y - z,
        _ => 0.0,
    }
}

fn lerp(a: f64, b: f64, x: f64) -> f64 {
    a + x * (b - a)
}
