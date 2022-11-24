use crate::{
    cameras::camera::Camera,
    common::{Real, NORMAL_BUMP},
    geometries::{area_light::AreaLight, intersectable::Intersectable, ray::Ray},
    lights::light::Light,
    materials::material::Material,
    math::color3::Color3,
    miss_shaders::miss_shader::MissShader,
};
use rand::RngCore;
use std::rc::Rc;

pub struct Scene {
    pub camera: Box<dyn Camera>,
    pub materials: Vec<Box<dyn Material>>,
    pub lights: Vec<Box<dyn Light>>,
    pub area_lights: Vec<Rc<dyn AreaLight>>,
    pub miss_shader: Box<dyn MissShader>,
    pub root_geometry: Rc<dyn Intersectable>,
}

impl Scene {
    pub fn new(
        camera: Box<dyn Camera>,
        materials: Vec<Box<dyn Material>>,
        lights: Vec<Box<dyn Light>>,
        area_lights: Vec<Rc<dyn AreaLight>>,
        miss_shader: Box<dyn MissShader>,
        root_geometry: Rc<dyn Intersectable>,
    ) -> Self {
        Self {
            camera,
            materials,
            lights,
            area_lights,
            miss_shader,
            root_geometry,
        }
    }

    pub fn cast_ray_color(&self, rng: &mut dyn RngCore, ray: &Ray, depth: u16) -> Color3 {
        if depth > 7 {
            return Color3::default();
        }

        let intersection = self.root_geometry.intersect(ray);

        match intersection {
            Some(intersection_some) => {
                let material = if intersection_some.material_index_override > 0 {
                    self.materials.get(intersection_some.material_index_override)
                } else {
                    self.materials.get(intersection_some.hit_geometry.material_index())
                };

                let mut hit_position = ray.position() + intersection_some.entrance_distance * ray.direction();

                let hit_normal = intersection_some.hit_geometry.calculate_normal(ray, &hit_position);

                hit_position += hit_normal * NORMAL_BUMP;

                match material {
                    Some(material_some) => material_some.calculate_rendering_equation(
                        rng,
                        self,
                        depth,
                        intersection_some.hit_geometry,
                        &hit_position,
                        &hit_normal,
                        ray.direction(),
                    ),
                    None => Color3::default(),
                }
            }
            None => self.miss_shader.calculate_color(ray),
        }
    }

    pub fn cast_ray_distance(&self, ray: &Ray) -> Option<Real> {
        let intersection = self.root_geometry.intersect(ray)?;
        Some(Real::max(0.0, intersection.entrance_distance))
    }
}
