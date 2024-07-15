use std::collections::HashMap;
use std::ops::Neg;
use cucumber::{given, then, World};
use futures::FutureExt;
use raytracer::linalg::tuple4::{point, Tuple4, vector};

#[derive(Debug, Default, World)]
pub struct RaytracerWorld {
    tuples: HashMap<String, Tuple4>,
}

impl RaytracerWorld {
    pub fn get_tuple(&self, variable: String) -> Tuple4 {
        return if variable.starts_with('-') {
            let var: String = variable.chars().skip(1).collect();
            self.tuples.get(&var).unwrap().clone().neg()
        } else {
            self.tuples.get(&variable).unwrap().clone()
        };
    }
}

#[given(expr = "{word} ← tuple\\({float}, {float}, {float}, {float})")]
fn given_a_tuple(world: &mut RaytracerWorld, variable: String, x: f64, y: f64, z: f64, w: f64) {
    let tuple = Tuple4(x, y, z, w);
    world.tuples.insert(variable, tuple);
}

#[given(expr = "{word} ← point\\({float}, {float}, {float})")]
fn given_a_point(world: &mut RaytracerWorld, variable: String, x: f64, y: f64, z: f64) {
    let point = point(x, y, z);
    world.tuples.insert(variable, point);
}

#[given(expr = "{word} ← vector\\({float}, {float}, {float})")]
fn given_a_vector(world: &mut RaytracerWorld, variable: String, x: f64, y: f64, z: f64) {
    let point = vector(x, y, z);
    world.tuples.insert(variable, point);
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

#[then(expr = "{word} = tuple\\({float}, {float}, {float}, {float})")]
fn is_equal_to_tuple(world: &mut RaytracerWorld, variable: String, x: f64, y: f64, z: f64, w: f64) {
    let tuple1 = world.get_tuple(variable);
    let tuple2 = Tuple4(x, y, z, w);
    assert_eq!(tuple1, tuple2);
}

#[then(expr = "{word} = point\\({float}, {float}, {float})")]
fn is_equal_to_point(world: &mut RaytracerWorld, variable: String, x: f64, y: f64, z: f64) {
    let tuple1 = world.get_tuple(variable);
    let tuple2 = point(x, y, z);
    assert_eq!(tuple1, tuple2);
}

#[then(expr = "{word} = vector\\({float}, {float}, {float})")]
fn is_equal_to_vector(world: &mut RaytracerWorld, variable: String, x: f64, y: f64, z: f64) {
    let tuple1 = world.get_tuple(variable);
    let tuple2 = vector(x, y, z);
    assert_eq!(tuple1, tuple2);
}

#[then(expr = "{word} + {word} = tuple\\({float}, {float}, {float}, {float})")]
fn sum_is_equal_to_tuple(world: &mut RaytracerWorld, variable1: String, variable2: String, x: f64, y: f64, z: f64, w: f64) {
    let tuple1 = world.get_tuple(variable1);
    let tuple2 = world.get_tuple(variable2);
    let expected = Tuple4(x, y, z, w);
    assert_eq!(tuple1.clone() + tuple2.clone(), expected);
}

#[then(expr = "{word} - {word} = tuple\\({float}, {float}, {float}, {float})")]
fn sub_is_equal_to_tuple(world: &mut RaytracerWorld, variable1: String, variable2: String, x: f64, y: f64, z: f64, w: f64) {
    let tuple1 = world.get_tuple(variable1);
    let tuple2 = world.get_tuple(variable2);
    let expected = Tuple4(x, y, z, w);
    assert_eq!(tuple1.clone() - tuple2.clone(), expected);
}

#[then(expr = "{word} - {word} = point\\({float}, {float}, {float})")]
fn sub_is_equal_to_point(world: &mut RaytracerWorld, variable1: String, variable2: String, x: f64, y: f64, z: f64) {
    let tuple1 = world.get_tuple(variable1);
    let tuple2 = world.get_tuple(variable2);
    let expected = point(x, y, z);
    assert_eq!(tuple1.clone() - tuple2.clone(), expected);
}

#[then(expr = "{word} - {word} = vector\\({float}, {float}, {float})")]
fn sub_is_equal_to_vector(world: &mut RaytracerWorld, variable1: String, variable2: String, x: f64, y: f64, z: f64) {
    let tuple1 = world.get_tuple(variable1);
    let tuple2 = world.get_tuple(variable2);
    let expected = vector(x, y, z);
    assert_eq!(tuple1.clone() - tuple2.clone(), expected);
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