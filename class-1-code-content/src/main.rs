fn main() {
    // println!("Hello, world!");
    // let x: u8 = 130;
    // let boo = false;

    // let y = 10.5f32;
    // let array: [f32; 4] = [10.0, 20.0, 10.0, 10.0];
    // println!("array 1: {}, {}", array[0], array[2]);
    // println!("array 4: {}", array[3]);

    // let tuple1 = (10, 10);
    // let tuple1 = (10, 10, 10.0, "s");
    // println!("tuple: {}", tuple1.0);
    // // println!("tuple: {}", tuple1.4); // Error

    // // let strage: u32;
    // // println!("result :{}",strage); // Error: Value haven't been assigned yet

    // type A = u32; // Alias

    // // Naming convention
    // let mut x = 10;
    // // Mutable
    // // Immutable
    // let ho_dung = "10";
    // x = x + 10;
    // println!("result: {}", x);

    // const PI: f64 = 3.14;
    // const NUMBER_CASE: u32 = 1;

    // let mut s = String::new();
    // println!("string is empty: {}", s.is_empty());
    // s.push('h');
    // println!("string is empty: {}", s);

    // let mut s = String::from("Hello world");
    // println!("string i: {}", s);

    // // std::mem::size_of::<char>();
    // // cast
    // // reference str
    // let s2 = String::from("Hello world1");
    // s = s + &s2;
    // println!("string is: {}", s);
    // println!("result is: {}", &s2[0..5]);

    // let mut s3 = "Hello World";
    // s3 = s3 + "Dung";
    // s3.push("ABC");

    // // Convert &str -> String
    // let conversion_string = "VBI".to_string();
    // // let conversion_str = &conversion_string; // Failed
    // let conversion_str = &*conversion_string; // True
    // let conversion_str = &&&&conversion_string.as_str();

    // let byte = conversion_string.as_bytes();
    // println!("result is  :{:?}", byte);

    // let mut s: String = String::new();
    // println!("s is: {}", s);

    // let mut s: &str = "Test";
    // println!("s is: {}", s);
    // let s_for = format!("{}", "Hello VBI");

    // let x = true;
    // if x {
    //     println!("Hello");
    // } else {
    //     println!("Bye");
    // }

    // Pattern matching
    // match: Switch Case / Case

    // match x {
    //     true => println!("Hello"),
    //     false => println!("Bye"),
    // }

    // // Default match
    // let number = 5;
    // match number {
    //     5 => println!("Hello: {}", number),
    //     10 => println!("Hello: {}", number),
    //     _ => println!("Hello the other world"),
    //     // 5 => println!("Hello Invalid"),
    // }

    // let tuple1 = (10, 10);
    // match tuple1 {
    //     (10, 10) | (20, 20) => match number {
    //         5 => println!("Hello: {}", number),
    //         10 => println!("Hello: {}", number),
    //         _ => println!("Hello Invalid"),
    //     },
    //     _ => todo!(),
    // }

    // let vec = vec![1, 23, 4, 5, 6];
    // // for value in vec {
    // //     println!("Value: {}",value);
    // // }
    // for value in vec.iter() {
    //     println!("Value: {}",value);
    // }
    
    // for index in 0..vec.len() {}
    // let max = vec.iter().max().unwrap();
    // println!("Max: {}", max);
    // Phân biệt giữa for bình thường và iter() hoặc iter_mut(), into_iter();
    // let k = function_x(&String::from("Veoquynhs"));
    // function_y(String::from("XYZ"));
    // let x = function_z(String::from("XYZ"));
    // println!("{}", x);
    // println!("{}", k);

    // Hàm bí danh, sử dụng closure
    // let x = || {
    //     println!("Hello");
    // };
    // x();

    // let x = |y: &str| -> String {
    //     y.to_string()
    // };
    // let res = x("VBI");
    // println!("Res: {}", res);

    // for, iter, into_iter, iter_mut
    let vec1 = vec![1,2,3,4,5];
    // for value in vec1 {
    //     println!("Value: {}", value);
    // }

    let vec2 = vec![1,2,3,4,5];
    for value in vec2.iter() {
        println!("Value: {}", value);
    }
    vec2.iter().enumerate().for_each(|x|{
        println!("Index: {}, value: {}", x.0, x.1);
    });
    let res: Vec<i32> = vec2.iter().map(|x|{
        x*2
    }).collect();
    println!("res: {:?}", res);

    let mut vec_new = vec![];
    for value in vec1 {
        vec_new.push(value*2);
    }
    println!("Cach 2: {:?}", vec_new);

    let vec3: Vec<i32>= vec![];
    let res = vec3.iter().max();
    println!("res: {:?}", res);

    let vec4 = vec![1,2,3,4,5];
    let res = vec4.iter().max();
    println!("res: {:?}", res);

    fn count_char_occurrences(string: &str, ch: char) -> usize {
        let res = string.chars().into_iter().filter(|c| c == &ch).count();
        res
    }
}

// fn function_x() {
//     println!("ABC");
// }
// fn function_y(s: String) {
//     println!("ABC:{}", s);
// }
// fn function_z(s: String) -> String {
//     s
// }

// fn function_y(x: &str) {
//     println!("ABC:{}", x);
// }

// fn function_x(x: &str) -> String  {
//     "ABC".to_string()
// }
