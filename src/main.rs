use diffus::edit;
use diffus::Diffable;
use diffus::Diffus;

#[derive(Diffus)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let left_point = Point { x: 1, y: 2 };
    let right_point = Point { x: 1, y: 3 };

    let diff = left_point.diff(&right_point);

    match diff {
        edit::Edit::Copy(_) => println!("point: no difference"),
        edit::Edit::Change(EditedPoint { x, y }) => {
            match x {
                edit::Edit::Copy(_) => println!("x: no difference"),
                edit::Edit::Change((left_x, right_x)) => println!("x: {} => {}", left_x, right_x),
            }
            match y {
                edit::Edit::Copy(_) => println!("y: no difference"),
                edit::Edit::Change((left_y, right_y)) => println!("y: {} => {}", left_y, right_y),
            }
        }
    }
}
