pub mod treelight;
pub mod ufo;

use std::{collections::HashMap, sync::Arc};

use tobj::{load_obj, GPU_LOAD_OPTIONS};

use crate::{
    basic::vec3::Vec3,
    bvh::BvhNode,
    hittable::{hittable_list::HittableList, objects::triangle::Triangle, Hittable},
    material::lambertian::Lambertian,
    texture::{image_texture::ImageTexture, obj_texture::ObjTexture},
};

#[derive(Clone, Copy)]
pub struct LoadOption<'a> {
    pub path: &'a str,
    pub file_name: &'a str,
    pub zoom_rate: f64,
    pub zoom_orig: Vec3,
    pub offset: Vec3,
    pub set_flag: bool, // when true, the offset is the position of the object's center
    pub r_x: f64,
    pub r_y: f64,
    pub r_z: f64,
}

#[allow(dead_code)]
pub fn my_loader(id: u32, paras: LoadOption) -> Box<dyn Hittable> {
    let file_str = String::from(paras.path) + paras.file_name + ".obj";
    let obj = load_obj(file_str, &GPU_LOAD_OPTIONS);
    let (models, materials) = obj.unwrap();
    let materials = materials.unwrap();
    //let default_mat = Lambertian::<SolidColor>::new_by_solid_color(Vec3(0.73, 0.73, 0.73));
    let mut tri_list = Vec::<Triangle<_>>::new();
    let mut img_map = HashMap::<String, ObjTexture>::new();

    let mut cnt = 0;

    for md in models.iter() {
        let mut obj_pt = Vec::<_>::new();
        let mut obj_nm = Vec::<_>::new();
        let mut obj_tx = Vec::<_>::new();
        let mat_id = md.mesh.material_id.unwrap();
        let mat_file_name = String::from(paras.path) + materials[mat_id].diffuse_texture.as_str();
        let tex = if let Some(data) = img_map.get::<String>(&mat_file_name) {
            data.clone()
        } else {
            let data = ObjTexture {
                ptr: Arc::new(ImageTexture::load_image_file(&mat_file_name)),
            };
            img_map.insert(mat_file_name, data.clone());
            data
        };

        let mat = Lambertian::new_by_texture(tex.clone());

        for p in md.mesh.positions.chunks(3) {
            obj_pt.push(Vec3(p[0] as f64, p[1] as f64, p[2] as f64));
        }
        for p in md.mesh.texcoords.chunks(2) {
            obj_tx.push((p[0] as f64, p[1] as f64));
        }
        for p in md.mesh.normals.chunks(3) {
            obj_nm.push(Vec3(p[0] as f64, p[1] as f64, p[2] as f64));
        }

        cnt += 1;

        println!(
            "Thread #{}, Name : {}, loading image {} / {}",
            id,
            paras.file_name,
            cnt,
            models.len()
        );

        for id in md.mesh.indices.chunks(3) {
            let mut tri = Triangle::new_from_obj(
                &obj_pt,
                &obj_nm,
                &obj_tx,
                [id[0] as usize, id[1] as usize, id[2] as usize],
                mat.clone(),
            );
            tri.zoom(paras.zoom_orig, paras.zoom_rate);
            if !paras.set_flag {
                tri.trans(paras.offset);
            }
            tri_list.push(tri);
        }
    }
    let mut center_old = Vec3::default();
    let _center_new = paras.offset;
    let tot_points: f64 = 3.0 * tri_list.len() as f64;
    for tri in tri_list.iter() {
        for i in 0..3 {
            center_old.0 += tri.p[i].0 / tot_points;
            center_old.1 += tri.p[i].1 / tot_points;
            center_old.2 += tri.p[i].2 / tot_points;
        }
    }
    let mut hit_list = HittableList::default();
    for tri in tri_list {
        let mut r_tri = tri;
        r_tri.rotate_xyz(center_old, paras.r_x, paras.r_y, paras.r_z);
        if paras.set_flag {
            r_tri.set_position(center_old, _center_new);
        }
        hit_list.add(Box::new(r_tri));
    }
    Box::new(BvhNode::new_from_list(hit_list, 0.0, 1.0))
}
