// fn main() {
//     println!("Hello, world!");
//     exercise2();
// }

fn exercise1() {
    // Use as many approaches as you can to make it work
    let x = String::from("hello");
    let y = &x;
    // Ownership có 3 quy tắc chính
    // move
    // let z = x; // False -> x không còn tồn tại
    // Xử lý: 2 cách
    // Cách 1: .clone()
    // Cách 2: Reference/Borrowing `&`

    // let w = x;
    // *y: Dereference -> Lấy giá trị thay vì đang trỏ tới địa chỉ
    println!("*y: {}", *y);
}

fn exercise2() {
    let mut s1 = String::from("hello, world");
    let s2 = take_ownership(&mut s1);

    println!("{}", s2);
    println!("{}", s1);
}
// Only modify the code below! 
fn take_ownership(s: &mut String) -> String {
    println!("{}", s);
    s.push_str("VBI");
    s.to_owned()
}

fn exercise3() {
    let values: Vec<f64> = vec![
        2817.42, 2162.17, 3756.57, 2817.42, -2817.42, 946.9, 2817.42, 964.42, 795.43, 3756.57,
        139.34, 903.58, -3756.57, 939.14, 828.04, 1120.04, 604.03, 3354.74, 2748.06, 1470.8,
        4695.71, 71.11, 2391.48, 331.29, 1214.69, 863.52, 7810.01,
    ];

    let values_number = values.len();

    let additions: Vec<usize> = vec![0];

    println!("{:?}", values_number);

    while additions.len() > 0 {
        let mut addition: f64 = 0.0;

        // Sumar valores en additions
        // iter, into_iter(), iter_mut()
        for &element_index in &additions {
            let addition_aux = values[element_index];
            addition = addition_aux + addition;
        }
    }
}

// fn main() {
//     let reference_to_nothing = dangle();
// }
// fn dangle() -> &String {
//     let s = String::from("hello");
//     // s là local
//     &s
// }
// Ownership cũ đã bị drop, nhưng lại có 1 thằng khác trỏ tới thằng
// drop đó, thì Rust không cho phép, bởi vì Rust không có kiểu dạng Null
// Dangling reference (Mượn 1 cách lỏng lẻo) -> Trỏ tới 1 biến không còn tồn tại nữa

// fn main() {}
// fn exercise4(value: u32) -> &'static str {
//     let str_value = value.to_string(); // Convert u32 to String
//     let str_ref: &str = &str_value; // Obtain a reference to the String
//     str_ref // Return the reference to the String
// }

// fn exercise4(value: u32) -> &'static str {
//     let str_value = value.to_string(); // Convert u32 to String
//     let str_ref: &str = &str_value; // Obtain a reference to the String
//     // str_ref // Return the reference to the String
//     // => False: Lại là lỗi Dangling reference
//     // Thằng `str_value` mang tính Ownership
//     // Thằng `str_ref` trỏ tới thằng `str_value` mang tính Ownership
//     // Sau khi thực hiện exercise4 xong => Thằng Ownership bị drop 
// }

fn exercise4_solution_1(value: u32) -> String {
    let str_value = value.to_string(); // Convert u32 to String
                                       //let str_ref: &str = &str_value; // Obtain a reference to the String
                                       //str_ref // Return the reference to the String
    str_value
}

fn exercise4_solution_2(value: u32) -> &'static str {
    let str_value = value.to_string(); // Convert u32 to String
    let str_ref: &str = &str_value; // Obtain a reference to the String
                                    //str_ref // Return the reference to the String
    "Hello" // Không phụ thuộc vào thằng Ownership nào cả
}

fn exercise4_solution_3(value: u32) -> &'static str {
    match value {
        0 => "Zero", //&'static str
        // &str thì phải reference từ 1 String nào đó
        // Còn &'static str thì khác
        1 => "One",
        _ => "Unknown",
    }
}
use std::collections::HashMap;
fn exercise5() {
    let mut my_map = HashMap::from([(1, "1.0".to_string()), (2, "2.0".to_string())]);

    let key = 3;

    let res = match my_map.get(&key) {
        Some(child) => child,
        None => {
            let value = "3.0".to_string();
            my_map.insert(key, value);
            //&value // HERE IT FAILS
            my_map.get(&key).unwrap() // Thay vì lấy trực tiếp `value` -> Thông qua `my_map`
        }
    };

    println!("{}", res);
}

use std::io;

fn exercise6() {
    let mut prev_key = String::new();

    for line in io::stdin().lines() {
        let s = line.unwrap();

        let data: Vec<&str> = s.split("\t").collect();
        if prev_key.len() == 0 {
            prev_key = data[0].to_owned();
        }
    }
}

fn exercise6_solution_2() {
    for line in io::stdin().lines() {
        let s = line.unwrap();
        let mut prev_key: &str = "";
        let data: Vec<&str> = s.split("\t").collect();
        if prev_key.len() == 0 {
            prev_key = data[0];
        }
    }
}

fn exercise6_solution_3() {
    for line in io::stdin().lines() {
        let s = line.unwrap();
        let mut prev_key: &str = "";
        let data: Vec<&str> = s.split("\t").collect();
        if prev_key.len() == 0 {
            prev_key = data[0];
        }
    }
}

fn main() {
    // Struct

    // Tuple
    let x = (10, "Hello", 16.0);
    // Nhược điểm: Không biết ý nghĩa thực sự của số 10 là gì, "Hello" là gì, 16.0 là gì

    let student_a = Students {
        class: 1,
        symbol: "Hi".to_string(),
        grade: 10.0,
    };
    let student_b = Students {
        class: 2,
        symbol: "hello".to_string(),
        grade: 5.0,
    };

    let class_1 = 1;
    let symbol = "World".to_string();
    let grade = 10.5;
    let student_c = Students {
        class: class_1,
        symbol,
        grade,
    }; // -> Tương tự như let ... = Students::new();

    let class_of_c = student_c.get_class();

    println!("Class of C: {}", class_of_c);

    let mut student_d = Students::new_student();

    // let class_of_d = student_d.get_class();
    // :: và .
    // :: dùng để gọi hàm từ struct, không nhận self <=> Định nghĩa hành vi không có self
    // . dùng khi đã có instance cụ thể <=> Định nghĩa hành vi có self

    let success = student_d.get_symbol();

    // Sử dụng Shared Reference
    println!("Student d: {:?}", student_d);

    // Mutable Reference
    student_d.set_grade(20.0);

    println!("Student d grade: {}", student_d.grade);

    let students = vec![student_a, student_b];

    let direction = Direction::West;
    let res = direction.convert_string();

    let shape = Shape::Circle (10.0);
    let rect = Shape::Rectangle { width: 10.0, length: 20.0 };

}

#[derive(Debug)]
// Struct
// Đối tượng: chung
// Hình thái cụ  thể
// Instance: 1 thực thể

struct Students {
    class: u8,
    symbol: String,
    grade: f64,
}

// Mô tả hành vi
// Implement
impl Students {
    // Constructor
    // Khởi tạo Instance
    fn new_student() -> Self {
        Self { class: 10, symbol: "YOLO".to_string(), grade: 6.0 }
    }
    // Method
    // self: thực thể
    fn get_class(self) -> u8 {
        self.class
    }

    fn get_symbol(&self) -> String {
        "Ok".to_string()
    }
    // Khi parameter không có self -> Chỉ cho object sử dụng, không cho thực thể

    fn set_grade(&mut self, new_grade: f64) {
        self.grade += new_grade;
    }

    // Hàm gọi hàm trong impl
    fn test(self) -> String {
        self.get_symbol()
    }

    // fn get_attr(&self, input: &str) -> String {
    //     match input {
    //         "class" => self.class.to_string(),
    //         "symbol" => self.symbol,
    //         "grade" => self.grade.to_string(),
    //         _ => todo!(),
    //     }
    // }
}

// enum dành cho việc định nghĩa các lựa chọn hữu hạn
enum Direction {
    West,
    East,
    South,
    North
}

enum Shape {
    Rectangle { width: f32, length: f32 },
    Circle (f32),
}

struct DirectionStr {
    direct: String,
}

impl Direction {
    fn convert_string(self) -> String {
        match self {
            Direction::East => "East".to_string(),
            Direction::West => "West".to_string(),
            Direction::South => "South".to_string(),
            Direction::North => "North".to_string(),
        }
    }
}

// Notes:
// Dangling Ref: Mượn biến đã bị drop
// self, &self, &mut self
// Lấy trường dữ liệu của object: instance.<tên trường dữ liệu>
// new: Hàm khởi tạo 
// Self vs self
// enum

// Câu hỏi discuss : 
// + get_attr
// + khi nào dùng enum , khi nào dùng struct 
// + partial move 
// khi match (sử dụng như là if else nhưng nó sẽ phù hợp khi mà dùng Option hoặc Result)

// enum có thể define struct hay ko và ngược lại 

// struct Student {
//     age: u32,
//     name: String,
//     friends: Vec<Student>,
//     best_friend: Student,
// } 

// struct Student {
//     age: u32,
//     name: String,
//     friends: Vec<Student>,
//     best_friend: Vec<Student>,
// } -> ổn

// enum UNDERSTANDING {
//     A_BIT,
//     MAKE_SENSE,
//     NO

// }

// struct Node {
//     value: i32,
//     next: & Node
// }
