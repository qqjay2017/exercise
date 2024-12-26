// fn main1() {
//     let input = '0';
//     match input {
//         'q' => println!("Quitting"),
//         'a' | 's' | 'w' | 'd' => println!("Moving around"),
//         '0'..='9' => println!("Number input"),
//         key if key.is_lowercase() => println!("Lowercase  {key}",),
//         _ => println!("Something else"),
//     }
// }

// struct  Foo {
//     x:(u32,u32),
//     y:u32,
// }

// fn main(){
//  let foo = Foo{x:(33,2),y:44};
//  match foo {
//     Foo {x:(1,b),y} => println!("x.0 = 1, b = {b}, y = {y}"),
//     Foo {y:2,x:i}=>println!("y=2,x={i:?}"),
//     Foo {y,..}=>println!("y={y} ,  other fields were ignored"),

//  }
// }

// enum Result {
//     Ok(i32),
//     Err(String),
// }

// fn devide_in_two(n:i32)->Result {
//     if n%2 ==0 {
//         Result::Ok(n/2)
//     }else {
//         Result::Err(format!("cannot divide {n} into two equal parts"))
//     }
// }

// fn main(){
//     let n = 100;
//     match  devide_in_two(n) {
//         Result::Ok(half)=>println!("{n} divided in two os {half}"),
//         Result::Err(msg)=>println!("Sorry, an error hapened: {msg}"),

//     }
// }

use std::time::Duration;

fn sleep_for(secs: f32) {
    if let Ok(dur) = Duration::try_from_secs_f32(secs) {
        std::thread::sleep(dur);
        println!("slept for {:?}", dur);
    }
}

fn main() {
    sleep_for(1.2);
    sleep_for(-10.0);
    sleep_for(0.8);
}
