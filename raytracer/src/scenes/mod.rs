use crate::{
    basic::vec3::Vec3, camera::Camera, hittable::hittable_list::HittableList,
    pdf::lightable_list::LightableList,
};

pub mod book1_final_scene;
pub mod book2_final_scene;
pub mod cornell_box_sences;
pub mod final_scene;
pub mod obj_test_scene;

pub struct SceneOption {
    pub world: HittableList,
    pub lights: LightableList,
    pub cam: Camera,
    pub background: Vec3,
}
