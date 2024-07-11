use std::collections::HashMap;
use cucumber::{given, then, World};
use futures::FutureExt;
use raytracer::linalg::tuple4::Tuple4;

#[derive(Debug, Default, World)]
pub struct RaytracerWorld {
    tuples: HashMap<String, Tuple4>,
}

impl RaytracerWorld {
    pub fn get_tuple(&self, variable: String) -> &Tuple4 {
        return self.tuples.get(&variable).unwrap();
    }
}

#[given(expr = "{word} ← tuple\\({float}, {float}, {float}, {float})")]
fn given_a_tuple(world: &mut RaytracerWorld, variable: String, x: f64, y: f64, z: f64, w: f64) {
    let tuple = Tuple4(x, y, z, w);
    world.tuples.insert(variable, tuple);
}

#[then(expr = "{word}.x = {float}")]
fn x_is_equal_to(world: &mut RaytracerWorld, variable: String, x: f64) {
    let tuple = world.get_tuple(variable);
    assert_eq!(tuple.0, x)
}

#[then(expr = "{word}.y = {float}")]
fn y_is_equal_to(world: &mut RaytracerWorld, variable: String, y: f64) {
    let tuple = world.get_tuple(variable);
    assert_eq!(tuple.1, y)
}

#[then(expr = "{word}.z = {float}")]
fn z_is_equal_to(world: &mut RaytracerWorld, variable: String, z: f64) {
    let tuple = world.get_tuple(variable);
    assert_eq!(tuple.2, z)
}

#[then(expr = "{word}.w = {float}")]
fn w_is_equal_to(world: &mut RaytracerWorld, variable: String, w: f64) {
    let tuple = world.get_tuple(variable);
    assert_eq!(tuple.3, w)
}

#[then(expr = "{word} is a point")]
fn is_a_point(world: &mut RaytracerWorld, variable: String) {
    let tuple = world.get_tuple(variable);
    assert!(tuple.is_point())
}

#[then(expr = "{word} is not a point")]
fn is_a_not_point(world: &mut RaytracerWorld, variable: String) {
    let tuple = world.get_tuple(variable);
    assert!(!tuple.is_point())
}

#[then(expr = "{word} is a vector")]
fn is_a_vector(world: &mut RaytracerWorld, variable: String) {
    let tuple = world.get_tuple(variable);
    assert!(tuple.is_vector())
}

#[then(expr = "{word} is not a vector")]
fn is_not_a_vector(world: &mut RaytracerWorld, variable: String) {
    let tuple = world.get_tuple(variable);
    assert!(!tuple.is_vector())
}

fn main() {
    futures::executor::block_on(RaytracerWorld::cucumber()
        .before(|_feature, _rule, _scenario, world| {
            async {
                world.tuples = HashMap::new()
            }.boxed_local()
        })
        .run_and_exit("tests/features/tuples.feature"));
}