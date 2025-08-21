use rand::Rng;
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use std::f64;
use std::ops::{ Add, Div, Mul, Neg, Sub };

#[cfg(test)]
use assert_approx_eq::assert_approx_eq;


#[derive(Copy, Clone , Copy, Deserialize, Serialize )]
pub struct Point3D {
    x:f64,
    y:f64,
    z:f64,
}

impl Point3D {
    pub fb new(x: f64, y: f64, z: f64) -> Point3D {
    Point3D {x, y, z}
    }


}



