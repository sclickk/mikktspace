use crate::vector::Vec3;

#[derive(Copy, Clone)]
pub struct TSpace {
    pub os: Vec3,
    pub mag_s: f32,
    pub ot: Vec3,
    pub mag_t: f32,

    pub counter: usize, // this is to average back into quads.
    pub orient: bool,
}

impl TSpace {
    #[inline]
    pub const fn zero() -> Self {
        Self {
            os: Vec3::zero(),
            mag_s: 0.0,
            ot: Vec3::zero(),
            mag_t: 0.0,
            counter: 0,
            orient: false,
        }
    }
}