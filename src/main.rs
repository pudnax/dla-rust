extern crate rand;
extern crate rstar;

use rand::Rng;

use rstar::RTree;

const D: f64 = 3.;

const DEFAULT_PARTICLE_SPACING: f64 = 1.;
const DEFAULT_ATTRACTION_DISTANCE: f64 = 3.;
const DEFAULT_MIN_MOVE_DISTANCE: f64 = 1.;
const DEFAULT_STUBBORNNESS: usize = 0;
const DEFAULT_STICKINESS: f64 = 1.;

mod vectors;
use vectors::{vec2d, vec3d, IndexValue2d, IndexValue3d, Vec2d, Vec3d};

struct Builder {
    DIMENSIONS: usize,
    // m_ParticleSpacing defines the distance between particles that are
    // joined together
    m_ParticleSpacing: f64,

    // m_AttractionDistance defines how close together particles must be in
    // order to join together
    m_AttractionDistance: f64,

    // m_MinMoveDistance defines the minimum distance that a particle will move
    // during its random walk
    m_MinMoveDistance: f64,

    // m_Stubbornness defines how many interactions must occur before a
    // particle will allow another particle to join to it.
    m_Stubbornness: usize,

    // m_Stickiness defines the probability that a particle will allow another
    // particle to join to it.
    m_Stickiness: f64,

    // m_BoundingRadius defines the radius of the bounding sphere that bounds
    // all of the particles
    m_BoundingRadius: f64,

    // m_Points stores the final particle positions
    m_Point2: Vec<Vec3d>,
    m_Point3: Vec<Vec3d>,

    // m_JoinAttempts tracks how many times other particles have attempted to
    // join with each finalized particle
    m_JoinAttempts: Vec<usize>,

    // m_Index is the spatial index used to accelerate nearest neighbor queries
    m_Index2: RTree<IndexValue3d>,
    m_Index3: RTree<IndexValue3d>,
}

impl Builder {
    pub fn flat() -> Model2 {
        Model2::new()
    }

    pub fn convex() -> Model3 {
        Model3::new()
    }
}

struct Model3 {
    m_ParticleSpacing: f64,
    m_AttractionDistance: f64,
    m_MinMoveDistance: f64,
    m_Stubbornness: usize,
    m_Stickiness: f64,
    m_BoundingRadius: f64,
    m_Points: Vec<Vec3d>,
    m_JoinAttempts: Vec<usize>,
    m_Index: RTree<IndexValue3d>,
}

impl Model3 {
    pub fn new() -> Model3 {
        Model3 {
            m_ParticleSpacing: DEFAULT_PARTICLE_SPACING,
            m_AttractionDistance: DEFAULT_ATTRACTION_DISTANCE,
            m_MinMoveDistance: DEFAULT_MIN_MOVE_DISTANCE,
            m_Stubbornness: DEFAULT_STUBBORNNESS,
            m_Stickiness: DEFAULT_STICKINESS,
            m_BoundingRadius: 0.,
            m_Points: Vec::new(),
            m_JoinAttempts: Vec::new(),
            m_Index: RTree::new(),
        }
    }

    fn SetParticleSpacing(&mut self, a: f64) {
        self.m_ParticleSpacing = a;
    }

    fn SetAttractionDistance(&mut self, a: f64) {
        self.m_AttractionDistance = a;
    }

    fn SetMinMoveDistance(&mut self, a: f64) {
        self.m_MinMoveDistance = a;
    }

    fn SetStubbornness(&mut self, a: usize) {
        self.m_Stubbornness = a;
    }

    fn SetStickiness(&mut self, a: f64) {
        self.m_Stickiness = a;
    }

    pub fn random_starting_position(&self) -> Vec3d {
        let d = self.m_BoundingRadius;
        vec3d::random_in_unit_sphere().normalized() * d
    }

    pub fn should_reset(&self, p: Vec3d) -> bool {
        p.length() > self.m_BoundingRadius * 2.
    }

    pub fn should_join(&mut self, p: Vec3d, parent: usize) -> bool {
        self.m_JoinAttempts[parent] += 1;
        if self.m_JoinAttempts[parent] < self.m_Stubbornness {
            return false;
        }
        rand::thread_rng().gen_range(0., 1.) <= self.m_Stickiness
    }

    pub fn place_particle(&self, p: Vec3d, parent: usize) -> Vec3d {
        vec3d::lerp(self.m_Points[parent], p, self.m_ParticleSpacing)
    }

    pub fn motion_vector(&self) -> Vec3d {
        vec3d::random_in_unit_sphere()
    }

    pub fn add(&mut self, p: Vec3d, parent: usize) {
        let id = self.m_Points.len();
        self.m_Index.insert(IndexValue3d { vec: p, index: id });
        self.m_Points.push(p);
        self.m_JoinAttempts.push(0);
        self.m_BoundingRadius = self
            .m_BoundingRadius
            .max(p.length() + self.m_AttractionDistance);
        println!("{},{},{},{},{}", id, parent, p.x, p.y, p.z);
    }

    pub fn nearest(&mut self, p: &Vec3d) -> usize {
        let nearest = self
            .m_Index
            .nearest_neighbor(&IndexValue3d { vec: *p, index: 0 })
            .expect("can't find nearest");
        nearest.index
    }

    pub fn add_particle(&mut self) {
        let mut p = self.random_starting_position();

        loop {
            let parent = self.nearest(&p);

            let d = p.distance(self.m_Points[parent]);

            if d < self.m_AttractionDistance {
                if !self.should_join(p, parent) {
                    // push particle away a bit
                    p = vec3d::lerp(
                        self.m_Points[parent],
                        p,
                        self.m_AttractionDistance + self.m_MinMoveDistance,
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
            let m = self.m_MinMoveDistance.max(d - self.m_AttractionDistance);
            p += self.motion_vector().normalized() * m;

            // check if particle is too far away, reset if so
            if self.should_reset(p) {
                p = self.random_starting_position();
            }
        }
    }
}

struct Model2 {
    m_ParticleSpacing: f64,
    m_AttractionDistance: f64,
    m_MinMoveDistance: f64,
    m_Stubbornness: usize,
    m_Stickiness: f64,
    m_BoundingRadius: f64,
    m_Points: Vec<Vec2d>,
    m_JoinAttempts: Vec<usize>,
    m_Index: RTree<IndexValue2d>,
}

impl Model2 {
    pub fn new() -> Model2 {
        Model2 {
            m_ParticleSpacing: DEFAULT_PARTICLE_SPACING,
            m_AttractionDistance: DEFAULT_ATTRACTION_DISTANCE,
            m_MinMoveDistance: DEFAULT_MIN_MOVE_DISTANCE,
            m_Stubbornness: DEFAULT_STUBBORNNESS,
            m_Stickiness: DEFAULT_STICKINESS,
            m_BoundingRadius: 0.,
            m_Points: Vec::new(),
            m_JoinAttempts: Vec::new(),
            m_Index: RTree::new(),
        }
    }

    fn SetParticleSpacing(&mut self, a: f64) {
        self.m_ParticleSpacing = a;
    }

    fn SetAttractionDistance(&mut self, a: f64) {
        self.m_AttractionDistance = a;
    }

    fn SetMinMoveDistance(&mut self, a: f64) {
        self.m_MinMoveDistance = a;
    }

    fn SetStubbornness(&mut self, a: usize) {
        self.m_Stubbornness = a;
    }

    fn SetStickiness(&mut self, a: f64) {
        self.m_Stickiness = a;
    }

    pub fn random_starting_position(&self) -> Vec2d {
        let d = self.m_BoundingRadius;
        vec2d::random_in_unit_sphere().normalized() * d
    }

    pub fn should_reset(&self, p: Vec2d) -> bool {
        p.length() > self.m_BoundingRadius * 2.
    }

    pub fn should_join(&mut self, p: Vec2d, parent: usize) -> bool {
        self.m_JoinAttempts[parent] += 1;
        if self.m_JoinAttempts[parent] < self.m_Stubbornness {
            return false;
        }
        rand::thread_rng().gen_range(0., 1.) <= self.m_Stickiness
    }

    pub fn place_particle(&self, p: Vec2d, parent: usize) -> Vec2d {
        vec2d::lerp(self.m_Points[parent], p, self.m_ParticleSpacing)
    }

    pub fn motion_vector(&self) -> Vec2d {
        vec2d::random_in_unit_sphere()
    }

    pub fn add(&mut self, p: Vec2d, parent: usize) {
        let id = self.m_Points.len();
        self.m_Index.insert(IndexValue2d { vec: p, index: id });
        self.m_Points.push(p);
        self.m_JoinAttempts.push(0);
        self.m_BoundingRadius = self
            .m_BoundingRadius
            .max(p.length() + self.m_AttractionDistance);
        println!("{},{},{},{},{}", id, parent, p.x, p.y, 0);
    }

    pub fn nearest(&mut self, p: &Vec2d) -> usize {
        let nearest = self
            .m_Index
            .nearest_neighbor(&IndexValue2d { vec: *p, index: 0 })
            .expect("can't find nearest");
        nearest.index
    }

    pub fn add_particle(&mut self) {
        let mut p = self.random_starting_position();

        loop {
            let parent = self.nearest(&p);

            let d = p.distance(self.m_Points[parent]);

            if d < self.m_AttractionDistance {
                if !self.should_join(p, parent) {
                    // push particle away a bit
                    p = Vec2d::lerp(
                        self.m_Points[parent],
                        p,
                        self.m_AttractionDistance + self.m_MinMoveDistance,
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
            let m = self.m_MinMoveDistance.max(d - self.m_AttractionDistance);
            p += self.motion_vector().normalized() * m;

            // check if particle is too far away, reset if so
            if self.should_reset(p) {
                p = self.random_starting_position();
            }
        }
    }
}

fn main() {
    let mut model = Builder::flat();

    model.add(Vec2d::new(0., 0.), 0);

    for _ in 0..100_000 {
        model.add_particle();
    }
}
