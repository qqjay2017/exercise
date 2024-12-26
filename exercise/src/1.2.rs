// fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
//     let mut result = [[0; 3]; 3];
//     for i in 0..3 {
//         for j in 0..3 {
//             result[j][i] = matrix[i][j];
//         }
//     }
//     result
// }

// fn main1() {
//     let matrix = [[101, 102, 103], [201, 202, 203], [301, 302, 303]];
//     println!("matrix: {:#?}", matrix);
//     let transposed = transpose(matrix);
//     println!("transposed: {:#?}", transposed);
// }

// fn main2() {
//     let a = 'A';
//     let b = 'B';
//     let mut r: &char = &a;
//     println!("r: {}", *r);
//     r = &b;
//     println!("r: {}", *r);
//     println!("a: {}", a);
//     println!("b: {}", b);
// }

// fn main3() {
//     let mut point = (1, 2);
//     let x_coord = &mut point.0;
//     *x_coord = 20;
//     println!("point: {point:?}");
// }

// fn main4() {
//     let mut a: [i32; 6] = [10, 20, 30, 40, 50, 60];
//     println!("a: {a:?}");

//     let s: &[i32] = &a[2..4];
//     println!("s: {s:?}");
// }

// fn main5() {
//     let s1: &str = "World";
//     println!("s1: {s1}");
//     let mut s2: String = String::from("Hello ");
//     println!("s2: {s2}");
//     s2.push_str(s1);
//     println!("s2: {s2}");
//     let s3: &str = &s2[s2.len() - s1.len()..];
//     println!("s3: {s3}");
// }

// fn magnitude(v: &[f64]) -> f64 {
//     let mut sum = 0.0;
//     for coord in v {
//         sum += coord * coord;
//     }

//     sum.sqrt()
// }

// fn normalize(v: &mut [f64]) {
//     let mag = magnitude(v);
//     for item in v {
//         *item /= mag;
//     }
// }

// fn main6() {
//     println!(
//         "Magnitude of a unit vector: {}",
//         magnitude(&[0.0, 1.0, 0.0])
//     );
//     let mut v = [1.0, 2.0, 9.0];
//     normalize(&mut v);
//     println!("Magnitude of {v:?} after normalization: {}", magnitude(&v));
// }

// // 结构体的
// struct Person {
//     name: String,
//     age: u8,
// }

// fn describe(person: &Person) {
//     println!("{} is {} years old", person.name, person.age);
// }
// fn main7() {
//     let mut peter = Person {
//         name: String::from("Peter"),
//         age: 27,
//     };
//     describe(&peter);
//     peter.age = 28;
//     describe(&peter);

//     let name = String::from("Avery");
//     let age = 39;
//     let avery = Person { name, age };
//     describe(&avery);
//     let jackie = Person {
//         name: String::from("Jackie"),
//         ..avery
//     };
//     describe(&jackie);
// }

// struct Point(i32, i32);

// fn main8() {
//     let p = Point(10, 20);
//     println!("({},{})", p.0, p.1);
// }

// enum Direction {
//     Left,
//     Right,
// }

// enum PlayerMove {
//     Pass,                        // Simple variant
//     Run(Direction),              // Tuple variant
//     Teleport { x: u32, y: u32 }, // Struct variant
// }
// fn main9() {
//     let m: PlayerMove = PlayerMove::Run(Direction::Left);
//     println!("On this turn: {:?}", m);
// }

// static BANNER:&str = "Welcome to RustOs 3.14";

// fn main10(){
//     println!("{BANNER}",);
// }

// const DIGEST_SIZE:usize = 3;
// const ZERO:Option<u8> = Some(42);

// fn compute_digest(text:&str)->[u8;DIGEST_SIZE]{
//     let mut digest =[ZERO.unwrap_or(0);DIGEST_SIZE];
//     for (idx, &b) in text.as_bytes().iter().enumerate() {
//         digest[idx % DIGEST_SIZE] = digest[idx % DIGEST_SIZE].wrapping_add(b);
//         }
//         digest

// }
// fn main() {
//     let digest = compute_digest("Hello");
//     println!("digest: {digest:?}");
//     }

type Floor = i32;
/// An event in the elevator system that the controller must react to.
/// 
#[derive(Debug)]
enum Event {
    // TODO: add required variants
    ButtonPressed(Button),
    CarArrived(Floor),
    CarDoorOpened,
    CarDoorClosed
}

#[derive(Debug)]
enum Button {
    LobbyCall(Direction, Floor),
    /// A floor button within the car.
    CarFloor(Floor),
}
/// A direction of travel.
 #[derive(Debug)]
enum Direction {
    Up,
    Down,
}
/// The car has arrived on the given floor.
fn car_arrived(floor: i32) -> Event {
    Event::CarArrived(floor)
}
/// The car doors have opened.
fn car_door_opened() -> Event {
    Event::CarDoorOpened
}
/// The car doors have closed.
fn car_door_closed() -> Event {
   Event::CarDoorClosed
}


/// A directional button was pressed in an elevator lobby on the given floor.
fn lobby_call_button_pressed(floor: i32, dir: Direction) -> Event {
    Event::ButtonPressed(Button::LobbyCall(dir, floor))
}

/// A floor button was pressed in the elevator car.
fn car_floor_button_pressed(floor: i32) -> Event {
   Event::ButtonPressed(Button::CarFloor(floor))
}
fn main() {
    println!(
        "A ground floor passenger has pressed the up button: {:?}",
        lobby_call_button_pressed(0, Direction::Up)
    );
    println!(
        "The car has arrived on the ground floor: {:?}",
        car_arrived(0)
    );
    println!("The car door opened: {:?}", car_door_opened());
    println!(
        "A passenger has pressed the 3rd floor button: {:?}",
        car_floor_button_pressed(3)
    );
    println!("The car door closed: {:?}", car_door_closed());
    println!("The car has arrived on the 3rd floor: {:?}", car_arrived(3));
}
