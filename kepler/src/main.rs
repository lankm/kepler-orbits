#![allow(unused)]
#![allow(non_snake_case)]

use kepler::*;
use std::f32::consts::PI;

fn main() {
    test();
}
fn test() {
    let orbit = Orbit::new(0.8, 2.0, PI/4.0, 0.0-PI/2.0, PI, 0.0);

    const COUNT: i32 = 10;
    const STEP: f32 = (2.0*PI/(COUNT as f32));

    for i in 0..COUNT {
        let M = STEP*(i as f32);
        let pos = orbit.pos(M);
        println!("({}, {}, {})", pos.0, pos.1, pos.2);
    }
}