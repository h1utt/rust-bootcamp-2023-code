#![allow(dead_code)]

// fn main() {
//     print!("Hello, world!");
//     print_u8(10u64 as u8);// cast
//     print_u64(10u64);
//     print_generic(10u8);
//     print_generic(20u64);
//     print_generic("Hello");
//     print_generic("Hello".to_string());
//     // Monomorphization
//     //
//     returns_reference();
//     // pub enum Option<T> {
//     //     None,
//     //     Some(T),
//     // }
//     let x = Some(09082304);
//     let y: Option<i32> = None;

//     print_generic::<&str>("Hello"); // Tương tự như `print_generic("Hello");`. Compiler đủ thông minh để hiểu
//     // print_generic::<u64>(10u8); // -> Lỗi

//     let point = Point::<f64>{ x: 1.0, y: 2.0 };
//     // let point1: Point<u64> = Point::new(10, 20);
//     let point1 = Point::<i32>::new(10, 20);
//     // let point1 = Point::new::<i32>(10, 20); // -> False

//     let x = Some(5);
//     let y = Some(5.0f64);
// }

struct Point<T> {
    x: T,
    y: T,
}

impl<AB> Point<AB>{
    fn new(x: AB, y: AB) -> Self {
        Self { x, y }
    }
}

struct Point1 {
    x: u32,
    y: u32,
}

impl Point1{
    fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

// Generic Type
// Type:
// Primitive
// Collections
// Generic: chung, tổng hợp -> Generic type: Kiểu dữ liệu chung, tổng hợp
// Placeholder

fn print_u8(input: u8) {
    println!("Input: {}", input);
}

fn print_u64(input: u64) {
    println!("Input: {}", input);
}

// Turbofish
// Định nghĩa Generic type bằng ký tự in hoa
fn print_generic<T: std::fmt::Debug>(input: T) {
    println!("Input: {:?}", input);
}

struct PointTwo<T, U> {
    x: T,
    y: U,
}

fn returns_reference<'a>() -> &'a str {
    let my_string = String::from("I am a string");
    //&my_string // ⚠️ -> Dangling Reference
    "Hello"
}
// Đánh dấu Lifetime <'a> -> Cho phép đối tượng tồn tại tới cuối chương trình
// stack
// heap
// static: Vùng khởi tạo

// Trait: Đặc tính, dùng để định nghĩa những đặc tính, đặc điểm CHUNG
// chia sẻ giữa các kiểu dữ liệu khác nhau - share behaviour. Định nghĩa chung -> Biểu diễn riêng

// Interface

// pub struct Car {
//     category: String
// }

// impl Car {
//     fn get_category(&self){
//         println!("Category: {}", self.category);
//     }
// }

trait Car {
    fn get_category(&self) -> String;
    fn speed(&self) -> u32;

    // general
    //
    //
}

// Định nghĩa
struct Sedan {}
impl Sedan {

}

impl Car for Sedan {
    fn get_category(&self) -> String {
        "Bốn bánh".to_string()
    }

    fn speed(&self) -> u32 {
        100
    }
}

struct Coupe {}

impl Car for Coupe {
    fn get_category(&self) -> String {
        "Xe mui trần".to_string()
    }

    fn speed(&self) -> u32 {
        200
    }
}

// Nhận xét
// +) Sử dụng các đặc tính chung để mô tả cho 1 Object cụ thể nào đó
// +) Tính riêng biệt của mỗi Object dựa trên đặc tính chung

struct MPV {}

struct SUV {}

struct Student {}

fn main() {
    let vios = Sedan{};
    let speed_vios = vios.speed();
    println!("{}", speed_vios);

    let bmw = Coupe{};
    let speed_bmw = bmw.speed();
    println!("{}", speed_bmw);

    let circle = Circle { radius: 10.0 };
    let rec = Rectangle { width: 10.0, height: 20.0 };
    
    println!("Circle area: {}", circle.area());
    println!("Rectangle area: {}", rec.area());

    let a = get_area(circle);
    println!("Circle area with trait bound: {}", a);

    let b = get_area(rec);
    println!("Rectangle area with trait bound: {}", b);

    let tri = Triangle{};
    // let c = get_area(tri); // Không được vì chưa impl Drawable for Triangle

    // Cách dùng đối với Square<T>
    let square: Square<f64> = Square { x: 10.0 };
    println!("Square area: {}", square.area());

    // Sử dụng đối với struct Square
    let c = get_area(square);

}

struct Triangle {}

struct Circle {
    radius: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

trait Drawable {
    fn draw(&self);
    fn area(&self) -> f64;
}

impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing a circle.");
    }

    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

impl Drawable for Rectangle {
    fn draw(&self) {
        println!("Drawing a rectangle.");
    }

    fn area(&self) -> f64 {
        self.width * self.height
    }
}
// Thay vì dựa vào trực tiếp Circle và Rectangle, mình sẽ dựa vào trait Drawable
// T là kiểu dữ liệu, nhưng bị ràng buộc bởi trait Drawable
// nghĩa là, T là những object mà có implement (hay được bound) bởi trait đó (ở đây là Drawable)
// Trait Bound: Generic type đang ràng buộc một số trait

// fn get_area<T>(method: T) -> f64 where T: Drawable {
//     method.area()
// } -> Cách 2

// fn get_area<T: Drawable>(method: T) -> f64 {
//     method.area()
// } // -> Cách 1

// Thực chất
// Những input/method nào có được implement bởi trait Drawable thì có thể sử dụng được
// fn get_area(method: impl Drawable) -> f64 {
//     method.area()
// }

// Generic function sử dụng trait
fn get_area<T>(method: T) -> f64 where T: Drawable {
    method.area()
}

// Generic struct
struct Square<T> {
    x: T
}

// <T: std::ops::Mul<Output = f64>> Cho phép Generic type nhân lại với nhau, output trả về là f64
// Mục đích: Implement trait Drawable cho kiểu dữ liệu Square
impl<T: std::ops::Mul<Output = f64> + Copy> Drawable for Square<T>{
    fn draw(&self) {
        println!("Drawing a square.");
    }

    fn area(&self) -> f64 {
        self.x * self.x 
    }
}

fn create_shape() -> impl Drawable {
    // Circle { radius: 5.0 } -> Ok
    // Triangle {} -> Lỗi
    Square { x: 10f64 } // -> Ok
}

// fn create_shape_other<T: Drawable>() -> T {
//     T
// }

// // impl std::ops::Mul for Square<f64> {
// //     type Output = f64;

// //     fn mul(self, rhs: Self) -> f64 {
// //         let res = self.x * rhs.x;
// //         res
// //     }
// // }

// // fn create_shape() -> object mà trait Drawable có implement {
// //     Circle { radius: 5.0 }
// // }


// // Summary 
// // Generic type -> Kiểu dữ liệu chung, chữ in hoa, bao nhiêu chữ cái cũng được, <>
// // Life time -> Đánh dấu thời gian tồn tại của biến
// // Trait -> Interface, đặc tính chung
// // Định nghĩa trait
// // impl trait -> Implement cho các kiểu dữ liệu: primitive, collections, object: struct, enum
// // Cách sử dụng -> impl trait for type, chia sẽ hành vi, đặc tính đã được định nghĩa chung
// // Tại sao sử dụng trait 
// // Trait bound (function with generic type + trait , struct generic + trait)
// // return trait 


// // Chú ý 
// // Kiểu dữ liệu phải đc implement (vd như Triangle là không được)