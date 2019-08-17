extern crate rand;
extern crate rstar;

use rand::Rng;

use rstar::RTree;

const DEFAULT_PARTICLE_SPACING: f64 = 1.;
const DEFAULT_ATTRACTION_DISTANCE: f64 = 3.;
const DEFAULT_MIN_MOVE_DISTANCE: f64 = 1.;
const DEFAULT_STUBBORNNESS: usize = 0;
const DEFAULT_STICKINESS: f64 = 1.;

pub mod structures;
use structures::{vec2d, vec3d, IndexValue2d, IndexValue3d, Vec2d, Vec3d};

mod raytracer;

// Particle_spacing defines the distance between particles that are
// joined together
//
// Attraction_distance defines how close together particles must be in
// order to join together
//
// Min_move_distanse defines the minimum distance that a particle will move
// during its random walk
//
// Stubbornness defines how many interactions must occur before a
// particle will allow another particle to join to it.
//
// Stickiness defines the probability that a particle will allow another
// particle to join to it.
//
// Bounding_radius defines the radius of the bounding sphere that bounds
// all of the particles
//
// Points stores the final particle positions
//
// Join_attempts tracks how many times other particles have attempted to
// join with each finalized particle
//
// Index is the spatial index used to accelerate nearest neighbor queries
pub struct Builder {}

impl Builder {
    pub fn flat() -> FlatAggregation {
        FlatAggregation::new()
    }

    pub fn convex() -> ConvexAggregation {
        ConvexAggregation::new()
    }
}

#[derive(Default)]
pub struct ConvexAggregation {
    particle_spacing: f64,
    attraction_distance: f64,
    min_move_distanse: f64,
    stubbornness: usize,
    stickiness: f64,
    bounding_radius: f64,
    join_attempts: Vec<usize>,
    pub points: Vec<Vec3d>,
    pub index: RTree<IndexValue3d>,
}

impl ConvexAggregation {
    pub fn new() -> ConvexAggregation {
        ConvexAggregation {
            particle_spacing: DEFAULT_PARTICLE_SPACING,
            attraction_distance: DEFAULT_ATTRACTION_DISTANCE,
            min_move_distanse: DEFAULT_MIN_MOVE_DISTANCE,
            stubbornness: DEFAULT_STUBBORNNESS,
            stickiness: DEFAULT_STICKINESS,
            bounding_radius: 0.,
            points: Vec::new(),
            join_attempts: Vec::new(),
            index: RTree::new(),
        }
    }

    pub fn set_particle_spacing(&mut self, a: f64) {
        self.particle_spacing = a;
    }

    pub fn set_attraction_distance(&mut self, a: f64) {
        self.attraction_distance = a;
    }

    pub fn set_min_move_distance(&mut self, a: f64) {
        self.min_move_distanse = a;
    }

    pub fn set_stubbornness(&mut self, a: usize) {
        self.stubbornness = a;
    }

    pub fn set_stickness(&mut self, a: f64) {
        self.stickiness = a;
    }

    pub fn random_starting_position(&self) -> Vec3d {
        let d = self.bounding_radius;
        vec3d::random_in_unit_sphere().normalized() * d
    }

    fn should_reset(&self, p: Vec3d) -> bool {
        p.length() > self.bounding_radius * 2.
    }

    fn should_join(&mut self, parent: usize) -> bool {
        self.join_attempts[parent] += 1;
        if self.join_attempts[parent] < self.stubbornness {
            return false;
        }
        rand::thread_rng().gen_range(0., 1.) <= self.stickiness
    }

    fn place_particle(&self, p: Vec3d, parent: usize) -> Vec3d {
        vec3d::lerp(self.points[parent], p, self.particle_spacing)
    }

    fn motion_vector(&self) -> Vec3d {
        vec3d::random_in_unit_sphere()
    }

    pub fn add(&mut self, p: Vec3d, parent: usize) {
        let id = self.points.len();
        self.index.insert(IndexValue3d { vec: p, index: id });
        self.points.push(p);
        self.join_attempts.push(0);
        self.bounding_radius = self
            .bounding_radius
            .max(p.length() + self.attraction_distance);
    }

    fn nearest(&mut self, p: &Vec3d) -> usize {
        let nearest = self
            .index
            .nearest_neighbor(&IndexValue3d { vec: *p, index: 0 })
            .expect("can't find nearest");
        nearest.index
    }

    pub fn add_particle(&mut self) {
        let mut p = self.random_starting_position();

        loop {
            let parent = self.nearest(&p);

            let d = p.distance(self.points[parent]);

            if d < self.attraction_distance {
                if !self.should_join(parent) {
                    // push particle away a bit
                    p = vec3d::lerp(
                        self.points[parent],
                        p,
                        self.attraction_distance + self.min_move_distanse,
                    );
                    continue;
                }

                // adjust particle position in relation to its parent
                p = self.place_particle(p, parent);

                // add the point
                self.add(p, parent);
                return;
            }

            // move randomly
            let m = self.min_move_distanse.max(d - self.attraction_distance);
            p += self.motion_vector().normalized() * m;

            // check if particle is too far away, reset if so
            if self.should_reset(p) {
                p = self.random_starting_position();
            }
        }
    }

    pub fn save_csv(&self, name: &str) -> std::io::Result<()> {
        println!("Saving to csv");
        use std::fs::File;
        use std::io::prelude::*;

        let mut file = File::create(name)?;
        file.write_all(b"index,parent,x,y,z\n")?;

        for (index, val) in self.index.iter().enumerate() {
            file.write_all(
                &format!(
                    "{},{},{:.04},{:.04},{:.04}\n",
                    index, val.index, val.vec.x, val.vec.y, val.vec.z
                )
                .as_bytes(),
            )
            .expect("second write");
        }

        Ok(())
    }

    pub fn render(&self, name: &str) {
        println!("Rendering");

        raytracer::render::render(self.index.iter().map(|x| x.vec).collect(), name);
    }
}

#[derive(Default)]
pub struct FlatAggregation {
    particle_spacing: f64,
    attraction_distance: f64,
    min_move_distanse: f64,
    stubbornness: usize,
    stickiness: f64,
    bounding_radius: f64,
    join_attempts: Vec<usize>,
    pub points: Vec<Vec2d>,
    pub index: RTree<IndexValue2d>,
}

impl FlatAggregation {
    pub fn new() -> FlatAggregation {
        FlatAggregation {
            particle_spacing: DEFAULT_PARTICLE_SPACING,
            attraction_distance: DEFAULT_ATTRACTION_DISTANCE,
            min_move_distanse: DEFAULT_MIN_MOVE_DISTANCE,
            stubbornness: DEFAULT_STUBBORNNESS,
            stickiness: DEFAULT_STICKINESS,
            bounding_radius: 0.,
            points: Vec::new(),
            join_attempts: Vec::new(),
            index: RTree::new(),
        }
    }

    pub fn set_particle_spacing(&mut self, a: f64) {
        self.particle_spacing = a;
    }

    pub fn set_attraction_distance(&mut self, a: f64) {
        self.attraction_distance = a;
    }

    pub fn set_min_move_distance(&mut self, a: f64) {
        self.min_move_distanse = a;
    }

    pub fn set_stubbornness(&mut self, a: usize) {
        self.stubbornness = a;
    }

    pub fn set_stickness(&mut self, a: f64) {
        self.stickiness = a;
    }

    pub fn random_starting_position(&self) -> Vec2d {
        let d = self.bounding_radius;
        vec2d::random_in_unit_sphere().normalized() * d
    }

    fn should_reset(&self, p: Vec2d) -> bool {
        p.length() > self.bounding_radius * 2.
    }

    fn should_join(&mut self, parent: usize) -> bool {
        self.join_attempts[parent] += 1;
        if self.join_attempts[parent] < self.stubbornness {
            return false;
        }
        rand::thread_rng().gen_range(0., 1.) <= self.stickiness
    }

    fn place_particle(&self, p: Vec2d, parent: usize) -> Vec2d {
        vec2d::lerp(self.points[parent], p, self.particle_spacing)
    }

    fn motion_vector(&self) -> Vec2d {
        vec2d::random_in_unit_sphere()
    }

    pub fn add(&mut self, p: Vec2d, parent: usize) {
        let id = self.points.len();
        self.index.insert(IndexValue2d { vec: p, index: id });
        self.points.push(p);
        self.join_attempts.push(0);
        self.bounding_radius = self
            .bounding_radius
            .max(p.length() + self.attraction_distance);
    }

    fn nearest(&mut self, p: &Vec2d) -> usize {
        let nearest = self
            .index
            .nearest_neighbor(&IndexValue2d { vec: *p, index: 0 })
            .expect("can't find nearest");
        nearest.index
    }

    pub fn add_particle(&mut self) {
        let mut p = self.random_starting_position();

        loop {
            let parent = self.nearest(&p);

            let d = p.distance(self.points[parent]);

            if d < self.attraction_distance {
                if !self.should_join(parent) {
                    // push particle away a bit
                    p = Vec2d::lerp(
                        self.points[parent],
                        p,
                        self.attraction_distance + self.min_move_distanse,
                    );
                    continue;
                }

                // adjust particle position in relation to its parent
                p = self.place_particle(p, parent);

                // add the point
                self.add(p, parent);
                return;
            }

            // move randomly
            let m = self.min_move_distanse.max(d - self.attraction_distance);
            p += self.motion_vector().normalized() * m;

            // check if particle is too far away, reset if so
            if self.should_reset(p) {
                p = self.random_starting_position();
            }
        }
    }

    pub fn save_csv(&self, name: &str) -> std::io::Result<()> {
        println!("Saving to csv");
        use std::fs::File;
        use std::io::prelude::*;

        let mut file = File::create(name)?;
        file.write_all(b"index,parent,x,y,z\n")?;

        for (index, val) in self.index.iter().enumerate() {
            file.write_all(
                &format!(
                    "{},{},{:.04},{:.04},{:.04}\n",
                    index, val.index, val.vec.x, val.vec.y, 0.
                )
                .as_bytes(),
            )
            .expect("second write");
        }

        Ok(())
    }

    pub fn render(&self, name: &str) {
        println!("Rendering");
        raytracer::render::render(
            self.index.iter().map(|x| Vec3d::from(x.vec)).collect(),
            name,
        );
    }
}
