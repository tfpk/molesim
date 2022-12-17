use std::f64::consts::{PI, TAU};

use rand::prelude::*;
use rand::thread_rng;

use rayon::prelude::*;

use glam::DVec2;

pub const WIDTH: f64 = 1920.0;
pub const HEIGHT: f64 = 1080.0;

pub const NUM_MOLECULES: usize = 200;
pub const MOLECULE_SPEED: f64 = 10.0;
pub const MOLECULE_RADIUS: f64 = 5.0;

pub const BOUNDARY_RATIO: f64 = 0.2;
pub const M: f64 = MOLECULE_SPEED * BOUNDARY_RATIO;

pub const BOUNDARY_ANGLE_RADS: f64 = (15.0 * PI) / 180.0;
#[derive(Debug, Clone)]
pub struct Molecule {
    pub position: DVec2,
    pub velocity: DVec2,
    pub id: usize,
}

impl Molecule {
    fn make_move(&mut self) {
        let new_position = self.position + self.velocity;

        let left_collision = new_position.x <= MOLECULE_RADIUS;
        let right_collision = new_position.x >= WIDTH - MOLECULE_RADIUS;
        let top_collision = new_position.y <= MOLECULE_RADIUS;
        let bottom_collision = new_position.y >= HEIGHT - MOLECULE_RADIUS;

        if left_collision {
            self.position.y = new_position.y;
            self.position.x = 2.0 * MOLECULE_RADIUS - new_position.x;
            self.velocity.x *= -1.0;
        }
        if right_collision {
            self.position.y = new_position.y;
            self.position.x = 2.0 * (WIDTH - MOLECULE_RADIUS) - new_position.x;
            self.velocity.x *= -1.0;
        }
        if top_collision {
            self.position.x = new_position.x;
            self.position.y = 2.0 * MOLECULE_RADIUS - new_position.y;
            self.velocity.y *= -1.0;
        }
        if bottom_collision {
            let new_velocity = if self.velocity.x - M > 0.0 {
                let b = DVec2::from_angle(BOUNDARY_ANGLE_RADS);
                let new_velocity = DVec2::new(
                    (b.x * b.x - b.y * b.y) * (self.velocity.x - M)
                        + 2.0 * b.y * b.x * self.velocity.y,
                    2.0 * b.y * b.y * (self.velocity.x - M)
                        - self.velocity.y * (b.x * b.x + b.y * b.y),
                );
                let mut new_velocity = new_velocity.normalize() * self.velocity.length();
                new_velocity.x += M;
                new_velocity
            } else {
                let mut b = DVec2::from_angle(BOUNDARY_ANGLE_RADS);
                b.x = -b.x;
                let new_velocity = DVec2::new(
                    (b.x * b.x - b.y * b.y) * (self.velocity.x + M)
                        + 2.0 * b.y * b.x * self.velocity.y,
                    2.0 * b.y * b.y * (self.velocity.y + M)
                        - self.velocity.y * (b.x * b.x + b.y * b.y),
                );
                let mut new_velocity = new_velocity.normalize() * self.velocity.length();
                new_velocity.x -= M;
                new_velocity
            };
            //this.vel[1] *=-1;
            //newVel = newVel.norm.scale(this.vel.len)
            let _original_len = self.velocity.length();
            let _new_len = new_velocity.length();

            self.velocity = new_velocity;
            self.position.x = new_position.x;
            self.position.y = 2.0 * (HEIGHT - MOLECULE_RADIUS) - self.position.y;
        }

        if !left_collision && !right_collision && !top_collision && !bottom_collision {
            self.position = new_position;
        }
    }

    fn create_random(id: usize) -> Molecule {
        let mut rng = thread_rng();
        let rand_angle = rng.gen_range(0.0..TAU);
        let rand_vector = DVec2::from_angle(rand_angle) * MOLECULE_SPEED.sqrt();
        Molecule {
            position: DVec2 {
                x: rng.gen_range(0.0..WIDTH),
                y: rng.gen_range(0.0..HEIGHT),
            },
            velocity: rand_vector,
            id,
        }
    }
}

#[derive(Debug)]
pub struct Molecules {
    pub molecules: Vec<Molecule>
}

impl Default for Molecules {
    fn default() -> Self {
        Self::new()
    }
}

impl Molecules {
    pub fn new() -> Molecules {
        Molecules {
            molecules: (0..NUM_MOLECULES).map(Molecule::create_random).collect()
        }
    }

    pub fn next(&mut self) {
        let molecules = &mut self.molecules;
        molecules.par_iter_mut().for_each(|b| b.make_move());
        let cloned_molecules = molecules.clone();

        molecules.par_iter_mut().for_each(|b| {
            for ob in &cloned_molecules {
                if ob.id == b.id { continue }
                let diff_vec = ob.position - b.position; //raw relative position vector.
                let dist_sq = diff_vec.length() * diff_vec.length(); // cenre spacing  scalar
                                                                     // You can't normalize a zero vector. The best we can do is skip this case.
                if dist_sq < 0.000001 {
                    continue;
                }
                let req_dist = 2.0 * MOLECULE_RADIUS; //ob.rad + b.rad; // 2*radius
                let req_dist_sq = req_dist * req_dist;
                if dist_sq > req_dist_sq {
                    continue;
                }; //not colliding
                //
                println!("old {}: p: {}, v: {}", b.id, b.position, b.velocity);

                let norm_unit = diff_vec.normalize(); // unit vector of relative position
                let tang_unit = DVec2::new(-norm_unit.y, norm_unit.x);

                let dist = dist_sq.sqrt();
                /*----move them back to the collision point----*/
                let half_overlap_len = (req_dist - dist) * 0.5;
                let half_overlap_vec = norm_unit * half_overlap_len;
                b.position -= half_overlap_vec;
                // ob.position += half_overlap_vec;

                /*--- b and ob collision position vectors */
                let new_moleculea_norm = norm_unit.dot(ob.velocity); //old_ob_norm;
                // let new_moleculeb_norm = norm_unit.dot(b.velocity); //old_b_norm;
                let new_moleculea_norm_vec = norm_unit * new_moleculea_norm;
                // let new_moleculeb_norm_vec = norm_unit * new_moleculea_norm;
                let moleculea_tang = tang_unit.dot(b.velocity);
                // let moleculeb_tang = tang_unit.dot(ob.velocity);
                let new_moleculea_tang_vec = tang_unit * moleculea_tang;
                // let new_moleculeb_tang_vec = tang_unit * moleculeb_tang;
                b.velocity = new_moleculea_norm_vec + new_moleculea_tang_vec;
                // ob.velocity = new_moleculeb_norm_vec + new_moleculeb_tang_vec;
                println!("new {}: p: {}, v: {}", b.id, b.position, b.velocity);
            }
        });

    }
}
