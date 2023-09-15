use lazy_static::lazy_static;
pub use nalgebra_glm::*;


pub const WIDTH: usize = 800;
pub const HEIGHT: usize = 800;
lazy_static! {
    pub static ref LIGHT: Vec3 = normalize(&vec3(0.5, 0.5, 0.1));
}
lazy_static! {
    pub static ref BG_COL: Vec3 = vec3(0.2, 0.4, 0.8);
}