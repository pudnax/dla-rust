use crate::raytracer;
use crate::raytracer::camera::Camera;
use crate::raytracer::color::Color;
use crate::raytracer::intersectable::sphere::Sphere;
use crate::raytracer::light::Light;
use crate::raytracer::light::LightType;
use crate::raytracer::material::Material;
use crate::raytracer::options::Options;
use crate::raytracer::scene::Scene;
use crate::raytracer::vector::Vec3;
use crate::structures::vec3d::{lerp, Vec3d};

extern crate rstar;
use rstar::RTree;

use std::time::Instant;

pub struct Raycaster {
    color: Color,
    points: Vec<Vec3d>,
    width: u32,
    height: u32,
}

impl Raycaster {
    pub fn flat(vec: RTree<crate::structures::IndexValue2d>) -> Raycaster {
        Raycaster {
            color: Color::white(),
            points: vec.iter().map(|x| Vec3d::from(x.vec)).collect(),
            width: 1920,
            height: 1080,
        }
    }
    pub fn convex(vec: RTree<crate::structures::IndexValue3d>) -> Raycaster {
        Raycaster {
            color: Color::white(),
            points: vec.iter().map(|x| x.vec).collect(),
            width: 1920,
            height: 1080,
        }
    }
    pub fn with_color(self, col: [f64; 3]) -> Raycaster {
        Raycaster {
            color: col.into(),
            points: self.points,
            ..self
        }
    }

    pub fn w_h(self, width: u32, height: u32) -> Raycaster {
        Raycaster {
            width,
            height,
            ..self
        }
    }

    pub fn render(&self, name: &str) {
        render(&self.points, name, self.color, self.width, self.height)
    }
}

pub fn render(vec: &[Vec3d], name: &str, col: Color, width: u32, height: u32) {
    println!("Rendering");
    let radius = vec.iter().fold(std::f64::EPSILON, |a, &b| {
        a.max((b.x * b.x + b.y * b.y + b.z * b.z).sqrt())
    });

    let options = Options {
        max_rays: 4,
        gamma: 0.85,
        diffuse: true,
        specular: true,
        shadows: true,
        reflections: true,
    };

    let fov = 90f64;
    let offset = (radius + 5.) / (fov * std::f64::consts::PI / 180. / 2.).tan();

    let aspect_ratio = f64::from(width) / f64::from(height);

    let mut plane: std::vec::Vec<
        std::boxed::Box<(dyn raytracer::intersectable::Intersectable + 'static)>,
    > = Vec::new();
    for coord in vec {
        plane.push(Box::new(Sphere {
            position: Vec3::new(coord.x, coord.y, coord.z),
            radius: 0.5,
            material: Material {
                color: col,
                diffuse: 0.6,
                specular: 50.0,
                specular_exponent: 100.0,
                reflectiveness: 0.0,
            },
        }));
    }

    let scene = Scene {
        width,
        height,
        camera: Camera::new(
            Vec3::new(0., 0., offset),
            Vec3::new(0., 0., 0.),
            fov,
            aspect_ratio,
            0.,
        ),
        objects: plane,
        lights: vec![
            Light {
                light_type: LightType::Point,
                position: Vec3::new(-40.0, 20.0, 20.0),
                intensity: 1.0,
                color: Color::white(),
            },
            Light {
                light_type: LightType::Point,
                position: Vec3::new(40.0, 20.0, 20.0),
                intensity: 0.8,
                color: Color::white(),
            },
            // Light {
            //     light_type: LightType::Point,
            //     position: Vec3::new(00.0, 50.0, 0.0),
            //     intensity: 0.8,
            //     color: Color::from_u8(0xa6, 0x7c, 0x00),
            // },
            Light {
                light_type: LightType::Ambient,
                position: Vec3::zero(),
                intensity: 0.25,
                color: Color::white(),
            },
        ],
        bg_color: Color::black(),
        options,
    };

    let now = Instant::now();

    scene.render(name.to_string());

    let duration = now.elapsed();

    println!(
        "{} milliseconds elapsed.",
        duration.as_secs() * 1000 + u64::from(duration.subsec_millis())
    );

    // println!("{}:{}:{}", duration.);
}
