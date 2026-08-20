use super::{Vec3, Vec4};
use std::fmt;
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Mat4 {
    pub values: [f32; 16],
}
impl Mat4 {
    pub const fn new(diagonal: f32) -> Self {
        Self {
            values: [
                diagonal, 0., 0., 0., 0., diagonal, 0., 0., 0., 0., diagonal, 0., 0., 0., 0.,
                diagonal,
            ],
        }
    }
    pub const fn identity() -> Self {
        Self {
            values: [
                1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.,
            ],
        }
    }
    pub fn multiply(a: Self, b: Self) -> Self {
        let mut r = [0.; 16];
        for c in 0..4 {
            for row in 0..4 {
                r[c * 4 + row] = (0..4)
                    .map(|k| a.values[k * 4 + row] * b.values[c * 4 + k])
                    .sum()
            }
        }
        Self { values: r }
    }
    pub const fn get(&self, row: usize, column: usize) -> f32 {
        self.values[column * 4 + row]
    }
    pub fn set(&mut self, row: usize, column: usize, value: f32) {
        self.values[column * 4 + row] = value;
    }
    pub fn transpose_mut(&mut self) {
        *self = Self::transpose(*self);
    }
    pub fn transpose(matrix: Self) -> Self {
        let mut values = [0.; 16];
        for row in 0..4 {
            for column in 0..4 {
                values[column * 4 + row] = matrix.values[row * 4 + column];
            }
        }
        Self { values }
    }
    pub fn inverse(matrix: Self) -> Option<Self> {
        let m = matrix.values;
        let mut inv = [0_f32; 16];
        inv[0] = m[5] * m[10] * m[15] - m[5] * m[11] * m[14] - m[9] * m[6] * m[15]
            + m[9] * m[7] * m[14]
            + m[13] * m[6] * m[11]
            - m[13] * m[7] * m[10];
        inv[4] = -m[4] * m[10] * m[15] + m[4] * m[11] * m[14] + m[8] * m[6] * m[15]
            - m[8] * m[7] * m[14]
            - m[12] * m[6] * m[11]
            + m[12] * m[7] * m[10];
        inv[8] = m[4] * m[9] * m[15] - m[4] * m[11] * m[13] - m[8] * m[5] * m[15]
            + m[8] * m[7] * m[13]
            + m[12] * m[5] * m[11]
            - m[12] * m[7] * m[9];
        inv[12] = -m[4] * m[9] * m[14] + m[4] * m[10] * m[13] + m[8] * m[5] * m[14]
            - m[8] * m[6] * m[13]
            - m[12] * m[5] * m[10]
            + m[12] * m[6] * m[9];
        inv[1] = -m[1] * m[10] * m[15] + m[1] * m[11] * m[14] + m[9] * m[2] * m[15]
            - m[9] * m[3] * m[14]
            - m[13] * m[2] * m[11]
            + m[13] * m[3] * m[10];
        inv[5] = m[0] * m[10] * m[15] - m[0] * m[11] * m[14] - m[8] * m[2] * m[15]
            + m[8] * m[3] * m[14]
            + m[12] * m[2] * m[11]
            - m[12] * m[3] * m[10];
        inv[9] = -m[0] * m[9] * m[15] + m[0] * m[11] * m[13] + m[8] * m[1] * m[15]
            - m[8] * m[3] * m[13]
            - m[12] * m[1] * m[11]
            + m[12] * m[3] * m[9];
        inv[13] = m[0] * m[9] * m[14] - m[0] * m[10] * m[13] - m[8] * m[1] * m[14]
            + m[8] * m[2] * m[13]
            + m[12] * m[1] * m[10]
            - m[12] * m[2] * m[9];
        inv[2] = m[1] * m[6] * m[15] - m[1] * m[7] * m[14] - m[5] * m[2] * m[15]
            + m[5] * m[3] * m[14]
            + m[13] * m[2] * m[7]
            - m[13] * m[3] * m[6];
        inv[6] = -m[0] * m[6] * m[15] + m[0] * m[7] * m[14] + m[4] * m[2] * m[15]
            - m[4] * m[3] * m[14]
            - m[12] * m[2] * m[7]
            + m[12] * m[3] * m[6];
        inv[10] = m[0] * m[5] * m[15] - m[0] * m[7] * m[13] - m[4] * m[1] * m[15]
            + m[4] * m[3] * m[13]
            + m[12] * m[1] * m[7]
            - m[12] * m[3] * m[5];
        inv[14] = -m[0] * m[5] * m[14] + m[0] * m[6] * m[13] + m[4] * m[1] * m[14]
            - m[4] * m[2] * m[13]
            - m[12] * m[1] * m[6]
            + m[12] * m[2] * m[5];
        inv[3] = -m[1] * m[6] * m[11] + m[1] * m[7] * m[10] + m[5] * m[2] * m[11]
            - m[5] * m[3] * m[10]
            - m[9] * m[2] * m[7]
            + m[9] * m[3] * m[6];
        inv[7] = m[0] * m[6] * m[11] - m[0] * m[7] * m[10] - m[4] * m[2] * m[11]
            + m[4] * m[3] * m[10]
            + m[8] * m[2] * m[7]
            - m[8] * m[3] * m[6];
        inv[11] = -m[0] * m[5] * m[11] + m[0] * m[7] * m[9] + m[4] * m[1] * m[11]
            - m[4] * m[3] * m[9]
            - m[8] * m[1] * m[7]
            + m[8] * m[3] * m[5];
        inv[15] = m[0] * m[5] * m[10] - m[0] * m[6] * m[9] - m[4] * m[1] * m[10]
            + m[4] * m[2] * m[9]
            + m[8] * m[1] * m[6]
            - m[8] * m[2] * m[5];
        let determinant = m[0] * inv[0] + m[1] * inv[4] + m[2] * inv[8] + m[3] * inv[12];
        if determinant.abs() < f32::EPSILON {
            return None;
        }
        for value in &mut inv {
            *value /= determinant;
        }
        Some(Self { values: inv })
    }
    pub fn multiply_vec4(self, v: Vec4) -> Vec4 {
        let m = self.values;
        Vec4::new(
            m[0] * v.x + m[4] * v.y + m[8] * v.z + m[12] * v.w,
            m[1] * v.x + m[5] * v.y + m[9] * v.z + m[13] * v.w,
            m[2] * v.x + m[6] * v.y + m[10] * v.z + m[14] * v.w,
            m[3] * v.x + m[7] * v.y + m[11] * v.z + m[15] * v.w,
        )
    }
    pub fn to_gl_array(self) -> [f32; 16] {
        self.values
    }
    /// Java `Mat4.asFloatArrayForGLSL`, retained for shader/debug output.
    pub fn as_glsl_string(self) -> String {
        let mut output = String::from("{");
        for (index, value) in self.values.iter().enumerate() {
            output.push_str(&format!("{value:.2}"));
            if index != self.values.len() - 1 {
                output.push(',');
            }
        }
        output
    }
    pub fn translation(v: Vec3) -> Self {
        let mut m = Self::identity();
        m.values[12] = v.x;
        m.values[13] = v.y;
        m.values[14] = v.z;
        m
    }
}
impl Default for Mat4 {
    fn default() -> Self {
        Self::identity()
    }
}
impl fmt::Display for Mat4 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{{")?;
        for row in 0..4 {
            if row == 0 {
                write!(formatter, "{{")?;
            } else {
                write!(formatter, " {{")?;
            }
            for column in 0..4 {
                write!(formatter, "{:.2}", self.get(row, column))?;
                if column < 3 {
                    write!(formatter, ", ")?;
                }
            }
            if row == 3 {
                write!(formatter, "}}")?;
            } else {
                writeln!(formatter, "}},")?;
            }
        }
        write!(formatter, "}}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn inverse_multiplied_by_matrix_is_identity() {
        let matrix = Mat4::multiply(
            Mat4::translation(Vec3::new(3.0, -2.0, 5.0)),
            crate::gmaths::mat4_transform::rotate_y(37.0),
        );
        let product = Mat4::multiply(matrix, Mat4::inverse(matrix).expect("invertible"));
        for row in 0..4 {
            for column in 0..4 {
                let expected = if row == column { 1.0 } else { 0.0 };
                assert!((product.get(row, column) - expected).abs() < 0.0001);
            }
        }
    }
}
