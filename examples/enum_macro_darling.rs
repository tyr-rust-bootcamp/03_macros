use macros::EnumFromDarling;

#[allow(unused)]
#[derive(Debug, EnumFromDarling)]
enum Direction {
    Up(DirectionUp),
    Down,
    Left(u32),
    Right { a: u32 },
}

#[allow(unused)]
#[derive(Debug)]
struct DirectionUp {
    speed: i32,
}

fn main() {
    let up: Direction = DirectionUp::new(42).into();
    let left: Direction = 10.into();
    println!("{:?}, {:?}", up, left);
}

impl DirectionUp {
    fn new(speed: i32) -> Self {
        Self { speed }
    }
}

// impl<T> From<i32> for Direction<T> {
//     fn from(v: i32) -> Self {
//         Direction::Left(v as u32)
//     }
// }

// impl<T> From<DirectionUp<T>> for Direction<T> {
//     fn from(v: DirectionUp<T>) -> Self {
//         Direction::Up(v)
//     }
// }

// ident: Direction, var: Up, ty: DirectionUp
// impl From<DirectionUp> for Direction {
//     fn from(v: DirectionUp) -> Self {
//         Direction::Up(v)
//     }
// }
