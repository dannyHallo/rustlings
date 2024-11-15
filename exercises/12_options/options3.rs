#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let optional_point = Some(Point { x: 100, y: 200 });

    if let Some(ref p) = optional_point {
        println!("Co-ordinates are {},{}", p.x, p.y);
    }

    println!("{optional_point:?}"); // Don't change this line.
}
