use tobj::{load_obj, GPU_LOAD_OPTIONS};

use crate::{
    basic::vec3::Vec3,
    bvh::BvhNode,
    hittable::{hittable_list::HittableList, objects::triangle::Triangle, Hittable},
    material::Material,
};

pub struct LoadOption<'a> {
    pub path: &'a str,
    pub zoom_rate: f64,
    pub zoom_orig: Vec3,
    pub offset: Vec3,
    pub r_x: f64,
    pub r_y: f64,
    pub r_z: f64,
}

pub fn my_loader<TM: Material + Clone + 'static>(paras: LoadOption, mat: TM) -> Box<dyn Hittable> {
    let patrick = load_obj(paras.path, &GPU_LOAD_OPTIONS);
    #[allow(unused_variables)]
    let (models, materials) = patrick.unwrap();
    let mut tri_list = Vec::<Triangle<_>>::new();
    for md in models {
        let mut obj_p = Vec::<Vec3>::new();
        for p in md.mesh.positions.chunks(3) {
            obj_p.push(Vec3(p[0] as f64, p[1] as f64, p[2] as f64));
        }
        for id in md.mesh.indices.chunks(3) {
            let mut tri = Triangle::new(
                obj_p[id[0] as usize],
                obj_p[id[1] as usize],
                obj_p[id[2] as usize],
                mat.clone(),
            );
            tri.zoom(paras.zoom_orig, paras.zoom_rate);
            tri.trans(paras.offset);
            tri_list.push(tri);
        }
    }
    let mut center = Vec3::default();
    let tot_points: f64 = 3.0 * tri_list.len() as f64;
    for tri in tri_list.iter() {
        for i in 0..3 {
            center.0 += tri.p[i].0 / tot_points;
            center.1 += tri.p[i].1 / tot_points;
            center.2 += tri.p[i].2 / tot_points;
        }
    }
    let mut hit_list = HittableList::default();
    for tri in tri_list {
        let mut r_tri = tri;
        r_tri.rotate_xyz(center, paras.r_x, paras.r_y, paras.r_z);
        hit_list.add(Box::new(r_tri));
    }
    Box::new(BvhNode::new_from_list(hit_list, 0.0, 1.0))
}
