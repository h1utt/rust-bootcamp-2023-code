// // fn main() -> Result<(), ThisError> {
// fn main() -> Result<(), Box<dyn std::error::Error>> {
// // Box<dyn std::error::Error -> Đây là 1 cách để trả về lỗi mà không cần 1 kiểu dữ liệu cụ thể nào đó
//     // panic!("Lỗi"); // Dừng chương trình ngay lập tức
//     println!("Hello, world!");
//     // let res = foo(-10).unwrap();
//     // let res = foo(-10)?; 

//     // Ok(())
//     // Dấu "?" thay bằng .unwrap()
//     // Dấu "?" sử dụng cho `Result` và `Option`
//     // Sử dụng khi hàm Foo có Result, và nằm trong hàm Main cũng có Result
//     // Dùng "?" -> In ra Error: "Wrong input"
//     // Dùng .unwrap() -> In ra lỗi

//     // let res = foo(-10).map_err(|_| "Wrong")?;
//     // let res = foo(-10).map_err(|e| e)?;
//     // let res = foo(-10);
//     // match res {
//     //     Ok(r) => println!("Result: {}", r),
//     //     Err(e) => println!("Error: {}", e),
//     // }

//     // Enum
//     // match res {
//     //     Ok(r) => println!("Result: {}", r),
//     //     Err(e) => println!("Error: {:?}", e),
//     // }

//     // if let

//     // if let Ok(r) = res {
//     //     println!("Result: {}", r);
//     // }
//     // else {
//     //     println!("Error");
//     // }

//     // hoặc

//     // if let Err(e) = res {
//     //     println!("Error: {}", e);
//     // }
//     // else {
//     //     let r = res.unwrap();
//     //     println!("Result: {}", r);
//     // }
    
//     // let res1 = divide(10, 0)?;
//     // let res2 = divide_2(10, 0)?;
//     call_foo_2()?;
//     Ok(())
    
//     // println!("{:?}", res);

//     // for i in 0..10 {
//     //     dbg!(i); 
//     // }

//     // pub enum Result<T, E> {
//     //     Ok(T), // Không có lỗi, được wrap bởi enum Ok
//     //     Err(E), // Có lỗi
//     // }

//     // pub enum Option<T> {
//     //     None,
//     //     Some(T),
//     // }
// }

// // fn foo(input: i32) -> Result<i32, &'static str> {
// //     if input > 0 {
// //         return Ok(input);
// //     }
// //     else {
// //         return Err("Wrong input");
// //     }
// // }

// // fn foo(input: i32) -> Result<i32, ThisError> {
// //     if input > 0 {
// //         return Ok(input);
// //     }
// //     else {
// //         return Err(ThisError::WrongInput);
// //     }
// // }

// // Đây là Super Trait, mình đang impl Super Trait cho 1 kiểu dữ liệu là ThisError
// // Super Trait có yêu cầu là Debug + Display
// // Cho nên phải impl 2 trait Debug & Display cho kiểu dữ liệu ThisError
// // Rust có thư viện sẵn #[derive(Debug)] -> Không cần manual impl nữa
// // Còn đối với Display -> Không có -> Cần impl
// impl std::error::Error for ThisError {

// }

// impl std::fmt::Display for ThisError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f,"{}",self)
//     }
// }

// #[derive(Debug)] // hoặc
// // impl std::fmt::Debug for ThisError {
// //     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
// //         match self {
// //             Self::WrongInput => write!(f, "WrongInput"),
// //             Self::NotFound => write!(f, "NotFound"),
// //         }
// //     }
// // }
// pub enum ThisError {
//     WrongInput,
//     NotFound,
// }

// // thiserror -> Không cần impl, chỉ cần thêm Debug
// // +) lib đã impl sẵn trait std::error::Error cho mình
// // +) convert enum sang &str
// #[derive(thiserror::Error, Debug)]
// pub enum ErrorWithThisError {
//     #[error("Wrong answer")] // Convert enum sang &str, bản chất chính là
//     // impl From<ThisError> for &str {
//     // }
//     WrongAnswer, // Thay vì trả về enum như thế này => Trả về thông tin chi tiết hơn
//     // #[error("A little bit more")]
//     // More,
//     // #[error("A little bit less")]
//     // Less,
// }

// fn foo_2(input: i32) -> Result<i32, Box<dyn std::error::Error>> {
//     if input > 0 {
//         return Ok(input);
//     }
//     else {
//         return Err(Box::new(ErrorWithThisError::WrongAnswer));
//     }
// }

// fn call_foo_2() -> Result<(), Box<dyn std::error::Error>> {
//     foo_2(-10)?;
//     Ok(())
// }

// #[derive(Debug)]
// struct CustomError {
//     message: String
// }

// impl std::error::Error for CustomError {

// }

// impl std::fmt::Display for CustomError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f,"{}",self.message)
//     }
// }
// fn divide(a: u32, b: u32) -> Result<u32, Box<dyn std::error::Error>> {
//     // Đối với trường hợp b = 0
//     if b == 0 {
//         return Err(Box::new(CustomError{ message: "Can not divide".to_string()}));
//     }
//     Ok(a/b)
// }


// // 2 library thường dùng cho Error Handling
// // anyhow
// // thiserror

// use anyhow::{Result, anyhow};
// fn divide_2(a: u32, b: u32) -> Result<u32> { // -> anyhow lo việc trả về lỗi
//     if b == 0 {
//         return Err(anyhow!("Can not divide")); 
//     }
//     Ok(a/b)
// }

// Macro
// Bản chất của Marco là code trong code
// Nó sẽ tuân theo các quy tắc
// Có nhiều kiểu marco
// marco_rule!
fn main() {
    println!("Hello, world!");
    let vec = vec![1,2,3,4];
    test!();
    let x = {
        let y = 10;
        let x = 20;
    };
    print_result!(x);

    let bad = Level::default();
}

// Deckaratuve macro
#[macro_export]
macro_rules! test {
    () => {
        println!("Ok")
    };
}

#[macro_export]
macro_rules! print_result {
    // This macro takes an expression of type `expr` and prints
    // it as a string along with its result.
    // The `expr` designator is used for expressions
    ($expression:expr) => {
        // `stringify` will convert the expression *as it is* into a string
        println!("{:?} = {:?}",
                stringify!($expression),
                $expression);
    };
}

#[derive(Default)] // Dùng derive Default được bởi vì u8 và Vec được impl trait Default
struct Student {
    grade: u8,
    girl_friend: Vec<String>
}

enum Level {
    GOOD,
    BAD
}

impl Default for Level {
    fn default() -> Self {
        Self::BAD
    }
}

use std::num::ParseIntError;

fn parse_number(s: &str) -> Result<i32, ParseIntError> {
    match s.parse::<i32>() {
        Ok(int) => Ok(int),
        Err(e) => return Err(e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    /// Test for exercise 2
    #[test]
    fn exercise2_should_work() {
        assert_eq!(parse_number("42"), Ok(42));
        assert_eq!(
            parse_number("invalid").map_err(|e| e.to_string()),
            Err("invalid digit found in string".parse().unwrap())
        );
    }
}