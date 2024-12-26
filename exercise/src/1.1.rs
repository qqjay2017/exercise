// fn interproduct(a: i32, b: i32, c: i32) -> i32 {
//     return a * b + b * c + c * a;
// }

// fn fib(n:u32) -> u32 {
//     if n <=1 {
//         return n;
//     } else {
//         return fib(n-1) + fib(n-2);
//     }
// }

fn main1() {
    let mut x = 200;
    while x>=10 {
        x= x/2;
      
        
    }
    println!("finalx  :  {x}"  );

    for n in 1..5 {
        println!("n  :  {n}"  );
    }
    for elem in [1,2,3,4,5]{
        println!("elem  :  {elem}"  );
    }

    let mut  i =0;
    loop {
        i+=1;
        println!("i: {i}");
        if i>100{
            break;
        }
    }
    let mut i2=0;
    loop {
        i2+=1;
        if i2>5 {
            break;
        }
        if i2%2 ==0 {
            continue;
        }
        println!("i2: {}",i2);
    }
}


fn main2 (){
    let s =[[5, 6, 7], [8, 9, 10], [21, 15, 32]];
    let mut elements_searched = 0;
    let target_value=10;
    'outer: for i in 0..=2 {
        for j in 0..=2 {
            elements_searched+=1;
            if s[i][j] == target_value {
                println!("Target value found at index ({}, {})", i, j);
                break 'outer;
            }
        }
    }
    print!("Elements searched: {}", elements_searched);
}

fn main3(){
    let z = 13;
     let x = {
        let y =10;
        println!("y: {}",y);
        z-y
     };
     println!("x: {}",x);
}

fn main4 (){
    let a = 10;
    println!("before: {a}");
    {
        let a = "hello";
        println!("inner scope: {a}");
        let a = true;
        println!("shadowed in inner scope : {a}");

    }
    println!("after: {a}");
}


fn collatz_length(mut n:i32)->i32 {
    let mut length = 1;
    while n >1  {
        if n % 2 == 0 {
            n = n / 2;
        } else {
            n = 3 * n + 1;
        }
        length += 1;
    }
    length

}

fn main(){
    println!("collatz_length(13): {}",collatz_length(11));
}