// Configs

// Extern crates

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate wasm_bindgen;

// Load modules

mod vector;

// Use statements

use vector::Vector;
use wasm_bindgen::prelude::*;

const SELF_INTERSECTION_THRESHOLD: f32 = 0.001;

#[derive(Serialize, Deserialize, Debug)]
pub struct Camera {
    pub point: Vector,
    pub vector: Vector,
    pub fov: f32,
}

struct Ray<'a> {
    point: &'a Vector,
    vector: Vector,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Object {
    Sphere(Sphere),
    Plane(Plane),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sphere {
    pub point: Vector,
    pub color: Vector,
    pub specular: f32,
    pub lambert: f32,
    pub ambient: f32,
    pub radius: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Plane {
    pub point: Vector,
    pub color: Vector,
    pub normal: Vector,
    pub specular: f32,
    pub lambert: f32,
    pub ambient: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DistanceFog {
    pub start_distance: f32,
    pub length: f32,
    pub color: Vector,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Scene {
    pub camera: Camera,
    pub objects: Vec<Object>,
    pub lights: Vec<Vector>,
    pub checker: Vec<Vector>,
    pub distance_fog: DistanceFog,
}

pub struct Intersection<'a> {
    pub distance: f32,
    pub collided_object: &'a Object,
}

#[wasm_bindgen]
pub fn binding(scene: String, width: f32, height: f32) -> Vec<f32> {
    let s: Scene = serde_json::from_str(&scene).unwrap();
    render(s, width, height)
}

pub fn render(scene: Scene, width: f32, height: f32) -> Vec<f32> {
    let mut result = Vec::new();
    let camera = &scene.camera;

    let eye_vector = camera.vector.subtract(&camera.point).unit();
    let vp_right = eye_vector.cross_product(&Vector::up()).unit();
    let vp_up = vp_right.cross_product(&eye_vector).unit();

    let fov_radians = std::f32::consts::PI * (camera.fov / 2.0) / 180.0;
    let height_width_ratio = height / width;
    let half_width = fov_radians.tan();
    let half_height = height_width_ratio * half_width;
    let camera_width = half_width * 2.0;
    let camera_height = half_height * 2.0;
    let pixel_width = camera_width / (width - 1.0);
    let pixel_height = camera_height / (height - 1.0);

    let mut ray = Ray {
        point: &camera.point,
        vector: Vector::up(),
    };

    for y in 0..height as u32 {
        for x in 0..width as u32 {
            let x_comp = vp_right.scale((x as f32 * pixel_width) - half_width);
            let y_comp = vp_up.scale((y as f32 * pixel_height) - half_height);

            ray.vector = eye_vector.add3(&x_comp, &y_comp).unit();

            let color = trace(&ray, &scene, 0).unwrap();

            result.push(color.x);
            result.push(color.y);
            result.push(color.z);
            result.push(255.0);
        }
    }

    result
}

fn trace(ray: &Ray, scene: &Scene, depth: usize) -> Option<Vector> {
    if depth > 2 {
        return None;
    }

    let dist_object = intersect_scene(ray, scene);

    match dist_object {
        None => Some(Vector::zero()),
        Some(dist_object) => {
            let point_in_time = ray.point.add(&ray.vector.scale(dist_object.distance));
            Some(surface(
                &ray,
                &scene,
                &dist_object.collided_object,
                &point_in_time,
                &normal(&dist_object.collided_object, &point_in_time),
                depth,
                dist_object.distance,
            ))
        }
    }
}

fn intersect_scene<'a>(ray: &Ray, scene: &'a Scene) -> (Option<Intersection<'a>>) {
    let mut closest = None;

    for object in &scene.objects {
        let distance: Option<f32> = object_intersection(object, ray);
        closest = match distance {
            None => closest,
            Some(some_distance) => {
                if some_distance > SELF_INTERSECTION_THRESHOLD {
                    match closest {
                        None => Some( Intersection { distance: some_distance, collided_object: object }),
                        Some(some_closest) => {
                            if some_distance < some_closest.distance {
                                Some( Intersection { distance: some_distance, collided_object: object })
                            }
                            else
                            {
                                Some(some_closest)
                            }
                        }
                    }
                }
                else
                {
                    closest
                }
            }
        }
    }

    closest
}

fn object_intersection(object: &Object, ray: &Ray) -> Option<f32> {
    match object {
        Object::Sphere(sphere) => {
            let eye_to_center = sphere.point.subtract(&ray.point);
            let v = eye_to_center.dot_product(&ray.vector);
            let eo_dot = eye_to_center.dot_product(&eye_to_center);
            let discriminant = (sphere.radius * sphere.radius) - eo_dot + (v * v);

            if discriminant < 0.0 {
                return None;
            }

            let distance = v - discriminant.sqrt();

            if distance > SELF_INTERSECTION_THRESHOLD {
                return Some(distance);
            }

            None
        }
        Object::Plane(plane) => {
            let neg_norm = plane.normal.negate();
            let denom = neg_norm.dot_product(&ray.vector);

            if denom <= 0.0 {
                return None;
            }

            let interm = plane.point.subtract(&ray.point);
            Some(interm.dot_product(&neg_norm) / denom)
        }
    }
}

fn normal(object: &Object, pos: &Vector) -> Vector {
    match object {
        Object::Sphere(sphere) => pos.subtract(&sphere.point).unit(),
        Object::Plane(plane) => plane.normal.unit(),
    }
}

fn plane_color_at(point_at_time: &Vector, _plane: &Plane, scene: &Scene) -> Vector {

    let width = 2;

    let checker_counter = ((point_at_time.x as i32) % width)
    + ((point_at_time.z as i32) % width)
    + if point_at_time.x < 0.0 { 0 } else { 1 }
    + if point_at_time.z < 0.0 { 0 } else { 1 };

    if checker_counter % 2 == 0 {
        return scene.checker[0].scale(1.0);
    }

    return scene.checker[1].scale(1.0);
}

fn surface(
    ray: &Ray,
    scene: &Scene,
    object: &Object,
    point_at_time: &Vector,
    normal: &Vector,
    depth: usize,
    time: f32
) -> Vector {
    let (object_color, lambert, specular, ambient) = match object {
        Object::Plane(obj) => (
            plane_color_at(point_at_time, obj, scene),
            obj.lambert,
            obj.specular,
            obj.ambient,
        ),
        Object::Sphere(obj) => (obj.color.scale(1.0), obj.lambert, obj.specular, obj.ambient),
    };

    let mut lambert_amount = 0.0;

    if lambert > 0.0 {
        for light in &scene.lights {
            if !is_light_visible(point_at_time, scene, light) {
                continue;
            }

            let contribution = light.subtract(point_at_time).unit().dot_product(normal);

            if contribution > 0.0 {
                lambert_amount += contribution;
            }
        }
    }

    let mut specular_reflection_color = Vector::zero();
    if specular > 0.0 {
        let reflected_ray = Ray {
            point: point_at_time,
            vector: ray.vector.reflect_through(normal),
        };
        let reflected_color = trace(&reflected_ray, scene, depth + 1);
        match reflected_color {
            Some(color) => {
                // Multiplying the reflected color by the object color is only valid for conductors (metals) and not dielectrics
                // Reference: https://www.scratchapixel.com/lessons/3d-basic-rendering/phong-shader-BRDF
                specular_reflection_color = specular_reflection_color.add(&color.multiply(&object_color).scale(specular / 256.0));
            }
            _ => {}
        }
    }

    lambert_amount = min(lambert_amount, 1.0);

    let output_color = specular_reflection_color.add3(&object_color.scale(lambert_amount * lambert), &object_color.scale(ambient));

    let distance_fog_lerp_scale: f32 = (time - scene.distance_fog.start_distance) / (scene.distance_fog.length);

    vector::lerp(&output_color, &scene.distance_fog.color, distance_fog_lerp_scale)
}

fn is_light_visible(point: &Vector, scene: &Scene, light: &Vector) -> bool {
    let point_to_light_vector = light.subtract(&point);
    let distance_to_light = point_to_light_vector.length();

    let ray = Ray {
        point,
        vector: point_to_light_vector.unit(),
    };
    let intersection = intersect_scene(&ray, scene);
    match intersection {
        None => true,
        Some(some_intersection) => some_intersection.distance > distance_to_light,
    }
}

fn min(a: f32, b: f32) -> f32 {
    if a > b {
        return b;
    }
    a
}
