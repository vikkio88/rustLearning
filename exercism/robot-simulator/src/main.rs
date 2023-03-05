use robot_simulator::{Direction, Robot};

fn main() {
    let r = Robot::new(0, 0, Direction::West);

    println!("{:?}", r.advance())
}
