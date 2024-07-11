mod linalg;

use linalg::tuple4::Tuple4;

fn main() {
    let v = Tuple4(1.0, 2.3, 3.0, 4.0);
    println!("v = ({}, {}, {}, {})", v.0, v.1, v.2, v.3);
}
