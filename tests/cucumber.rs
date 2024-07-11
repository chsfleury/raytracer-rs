use cucumber::{given, then, World};
use raytracer::linalg::tuple4::Tuple4;

#[derive(Debug, Default, World)]
pub struct TupleWorld {
    tuple: Tuple4
}

#[given(expr = "a ← tuple\\({float}, {float}, {float}, {float})")]
fn given_a_tuple(world: &mut TupleWorld, x: f64, y: f64, z: f64, w: f64) {
    world.tuple = Tuple4(x, y, z, w);
}

#[then(expr = "a.x = {float}")]
fn x_is_equal_to(world: &mut TupleWorld, x: f64) {
    assert_eq!(world.tuple.0, x)
}

#[then(expr = "a.y = {float}")]
fn y_is_equal_to(world: &mut TupleWorld, y: f64) {
    assert_eq!(world.tuple.1, y)
}

#[then(expr = "a.z = {float}")]
fn z_is_equal_to(world: &mut TupleWorld, z: f64) {
    assert_eq!(world.tuple.2, z)
}

#[then(expr = "a.w = {float}")]
fn w_is_equal_to(world: &mut TupleWorld, w: f64) {
    assert_eq!(world.tuple.3, w)
}

#[then(expr = "a is a point")]
fn is_a_point(world: &mut TupleWorld) {
    assert!(world.tuple.is_point())
}

#[then(expr = "a is not a point")]
fn is_a_not_point(world: &mut TupleWorld) {
    assert!(!world.tuple.is_point())
}

#[then(expr = "a is a vector")]
fn is_a_vector(world: &mut TupleWorld) {
    assert!(world.tuple.is_vector())
}

#[then(expr = "a is not a vector")]
fn is_not_a_vector(world: &mut TupleWorld) {
    assert!(!world.tuple.is_vector())
}


fn main() {
    futures::executor::block_on(TupleWorld::run(
        "tests/features/tuples.feature"
    ));
}