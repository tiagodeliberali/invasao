use amethyst::{core::math::Vector3, core::Transform};

pub struct Entity3d<'a> {
    transform: &'a mut Transform,
}

#[allow(dead_code)]
impl<'a> Entity3d<'a> {
    pub fn new(transform: &mut Transform) -> Entity3d {
        Entity3d { transform }
    }

    pub fn up(&self) -> Vector3<f32> {
        self.transform.isometry().inverse().rotation * Vector3::y()
    }

    pub fn forward(&self) -> Vector3<f32> {
        self.transform.isometry().inverse().rotation * Vector3::z()
    }

    pub fn rigth(&self) -> Vector3<f32> {
        self.transform.isometry().inverse().rotation * Vector3::x()
    }

    pub fn walk_forward(&mut self, amount: f32) {
        self.transform
            .set_translation_z(self.transform.translation().z - self.forward().z * amount);
        self.transform
            .set_translation_x(self.transform.translation().x + self.forward().x * amount);
    }

    pub fn walk_right(&mut self, amount: f32) {
        self.transform
            .set_translation_x(self.transform.translation().x + self.rigth().x * amount);
        self.transform
            .set_translation_z(self.transform.translation().z - self.rigth().z * amount);
    }

    pub fn rotate_horizontal(&mut self, amount: f32) {
        self.transform.prepend_rotation_y_axis(-amount);
    }

    pub fn rotate_vertical(&mut self, amount: f32) {
        self.transform.prepend_rotation_x_axis(-amount);
    }
}
