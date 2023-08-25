use std::ops::{Add, Div, Mul, Rem, Sub};

fn main() {
    let circle = Circle { radius: 10.0 };

    println!("Circle: {:?}", circle.area());
    println!("Circle draw: {:?}", circle.draw());

    let mut counter = Counter { x: 0.0 };
    println!("Counter next: {:?}", counter.next());
    println!("Counter next: {:?}", counter.next());

    let mut counter_a = CounterAssociated { x: 0 };
    println!("Counter next: {:?}", counter_a.next());
    println!("Counter next: {:?}", counter_a.next());

    let mut vec: Vec<u32> = vec![1, 2, 3, 4];

    let res = vec.next().unwrap();
    // Ta sử dụng .next() được bởi vì nó là vector. Ta định nghĩa trait IteratorVec
    // đồng thời đã impl trait đó cho vector
    println!("res: {}", res);

    println!("Create shape: {:?}", create_shape());
    println!("Create shape 2: {:?}", create_shape_2());

    let circle1 = Circle { radius: 10.0 };
    let rec = Rectangle {
        width: 10.0,
        height: 10.0,
    };

    // let shapes = vec![circle1, rec]; // -> Lỗi vì không cùng kiểu dữ liệu

    // Vậy ta sẽ làm thế nào để lưu được 2 objects là `circle1` và `rec`

    // Cho vào Box? -> Không được
    // let shapes = vec![Box::new(circle1), Box::new(rec)];

    let test: Vec<u8> = vec![1, 23];
    // -> Biến trait thành 1 kiểu dữ liệu (object) <=> trait object
    let shapes: Vec<Box<dyn Shape>> = vec![Box::new(circle1), Box::new(rec)]; // -> Done

    let tri = Triangle {};
    for shape in shapes {
        let res = shape.area();
    }

    // Trait Object trong trường hợp này có nhiệm vụ như 1 kiểu dữ liệu, cung cấp
    // một cách làm việc với các đối tượng có kiểu trait chung

    // Static Dispatch
    // Static: Tĩnh
    // Dispatch: Execute/Thực thi 1 hàm


    // Cách sử dụng Static Dispatch
    draw_static(&circle);
    draw_dynamic(&circle);

    // trait bound 


    let res = String::from(ErrorCustom::NotFound);
    println!("res: {}", res);

    // From và Into ngược nhau
    // TryFrom: trả về thêm Error -> Chặt chẽ hơn
    let res2: String = ErrorCustom::NotFound.into();
    println!("res: {}", res2);

    let res3 = String::from_fake(Box::new(ErrorCustom::NotFound));
    println!("res:{}", res3);

    let student = Student {
        grade: 10,
        name: "VBI".to_string()
    };

    println!("student: {}",student);
}

// Static Dispatch
// Áp dụng Trait Bound
fn draw_static<T: Drawable>(shape: &T) {
    shape.draw();
}
// Compiler biết rằng trait Drawable có impl cho Circle và Rectangle
// Tĩnh (Static) có nghĩa là gì? Ở compile time, compiler đã biết được các
// object đã xác định như Circle, Rectangle

// Bản chất của Static Dispatch -> Có `imp` -> Bla bla
fn draw_static_other(shape: &impl Drawable) {
    shape.draw();
}

// fn draw_static_circle(shape: &Circle) {
//     shape.draw();
// }

// fn draw_static_rectangle(shape: &Rectangle) {
//     shape.draw();
// }

// Dynamic Dispatch
// Đối với Dynamic Dispatch, không có `impl`, compiler không rõ trait 
// Drawable có implement cho kiểu dữ liệu cụ thể nào hay không, nó chỉ biết 
// khi chạy run time
fn draw_dynamic(shape: &dyn Drawable) {
    shape.draw();
}

struct Triangle {}
#[derive(Debug)]
struct Circle {
    radius: f64,
}

#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

trait Drawable {
    fn draw(&self);
}

// Super Trait
// std::fmt::Debug -> Cần thêm Trait Debug để compiler có thể hiểu
trait Shape: Drawable + std::fmt::Debug {
    fn area(&self) -> f64;
}
impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing a circle.");
    }

    // fn area(&self) -> f64 {
    //     std::f64::consts::PI * self.radius * self.radius
    // }
}

impl Drawable for Rectangle {
    fn draw(&self) {
        println!("Drawing a rectangle.");
    }

    // fn area(&self) -> f64 {
    //     self.width * self.height
    // }
}

// Bắt buộc phải có Implement cho Circle
impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

// Bắt buộc phải có Implement cho Rectangle
impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

// Work vì compiler biết Shape có impl cho Circle
fn create_shape() -> impl Shape {
    Circle { radius: 10.0 }
}

// Chẳng hạn mình quên `impl` -> Không có impl, chỉ có trait
// -> Compiler hiểu rằng mình đang trả về 1 trait object, giống như trả về struct, enum, ...
// Nhưng nó không rõ ràng, bởi vì đặc tính của trait đã là general - chung
// Khi đó cần thêm keyword `dyn` - dynamic

// Nếu dùng `impl Shape` -> Compiler đã biết size của object Circle
// Nhưng nếu không dùng -> Compiler sẽ không biết size ở thời gian compile

// Box: Smart Pointer
// Thay vì trả về 1 trait object, thì mình sẽ wrap nó trong 1 smart pointer
// Smart Pointer sẽ giúp allocate (phân bổ) memory ở trên heap, do kích cỡ chưa biết -> Dựa vào heap
// tương tự kiểu như String, Vec::new()

fn create_shape_2() -> Box<dyn Shape> {
    Box::new(Circle { radius: 10.0 })
}

fn foo() -> &'static str {
    "Hello"
}

// Cần xác nhận
// fn create_shape_3() -> &'static dyn Shape {
//     &Box::new(Circle {radius: 10.0})
// }


// Associated type: Thực chất vẫn là generic type, nhưng sẽ có 1 số lợi thế
// so với generic type
// Generic type: Kiểu dữ liệu chung

pub trait Iterator<T> {
    fn next(&mut self) -> T;
}

// Thay vì sử dụng x là u32 thì sử dụng Generic type
pub struct Counter<T> {
    x: T,
}

// std::ops::Add<f64, Output = T> // Thêm Trait có implement cho T mà có khả năng cộng (Add)
// Copy: Reference
impl<T: std::ops::Add<f64, Output = T> + Copy> Iterator<T> for Counter<T> {
    fn next(&mut self) -> T {
        self.x = self.x + 1.0;
        self.x
    }
}

trait Number:
    Copy
    + Clone
    + Eq
    + PartialEq
    + Ord
    + PartialOrd
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Rem<Output = Self>
{
}
// Associated Type
// :Cách sử dụng: Chuyển kiểu từ Generic sang Associated Type
pub struct CounterAssociated {
    x: u32,
}

pub trait IteratorA {
    type Item; // Item là 1 kiểu dữ liệu nào đó
    fn next(&mut self) -> Self::Item;
}

impl IteratorA for CounterAssociated {
    // Khi implement -> Cần cho cái cụ thể
    type Item = u32;
    fn next(&mut self) -> u32 {
        self.x = self.x + 1;
        self.x
    }
}

// Cần Discuss
trait IteratorVec {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

impl IteratorVec for Vec<u32> {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        Some(10)
    }
}

// Ta có 1 struct như bên dưới, sau đó ta sẽ định nghĩa 1 trait bao gồm 3
// behavior: `contains`, `first`, `last`
struct Container(i32, i32);

// Interface -> Shared behaviour cho các kiểu dữ liệu khác nhau
trait Contains {
    type A;
    type B;
    fn contains(&self, _: &Self::A, _: &Self::B) -> bool; // Explicitly requires `A` and `B`.
    fn first(&self) -> i32; // Doesn't explicitly require `A` or `B`.
    fn last(&self) -> i32; // Doesn't explicitly require `A` or `B`.
}

impl Contains for Container {
    type A = i32;
    type B = i32;
    // True if the numbers stored are equal.
    fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }

    // Grab the first number.
    fn first(&self) -> i32 {
        self.0
    }

    // Grab the last number.
    fn last(&self) -> i32 {
        self.1
    }
}

// `C` contains `A` and `B`. In light of that, having to express `A` and
// `B` again is a nuisance.
// Ta đang sử dụng trait như 1 parameter, bằng cách thông qua Generic type
fn difference<C: Contains>(container: &C) -> i32 {
    container.last() - container.first()
}

// Static dispatch
// Dynamic dispatch

// Static: Trait bound
// Dynamic: `dyn`

// Chữa bài tập
trait Foo {
    fn method(&self) -> String;
}

impl Foo for u8 {
    fn method(&self) -> String {
        format!("u8: {}", *self)
    }
}

impl Foo for String {
    fn method(&self) -> String {
        format!("string: {}", *self)
    }
}

// IMPLEMENT below with generics and parameters
// Trait Bound
fn static_dispatch(x: impl Foo) {
    let _ = x.method();
}
// Implement below with trait objects and parameters
// dyn
fn dynamic_dispatch(x: Box<dyn Foo>) {
    todo!()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exercise4_should_work() {
        let x = 5u8;
        let y = "Hello".to_string();

        static_dispatch(x);
        dynamic_dispatch(Box::new(y));
    }
}

// #[derive(Debug)]
enum ErrorCustom {
    NotFound,
    FailToCreate,
}

// Làm sao có thể chuyển được kiểu dạng từng phần tử của Enum sang String
// -> Sử dụng Trait std::convert::From để convert từ thằn này sang thằng khác
// Như ví dụ ở đây là Enum -> String

impl std::error::Error for ErrorCustom {

}

impl std::fmt::Display for ErrorCustom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self)
    }
}

impl std::fmt::Debug for ErrorCustom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound => write!(f, "NotFound"),
            Self::FailToCreate => write!(f, "FailToCreate"),
        }
    }
}

impl From<ErrorCustom> for String {
    fn from(value: ErrorCustom) -> Self {
        match value {
            ErrorCustom::NotFound => "not found".to_string(),
            ErrorCustom::FailToCreate => "fail to create".to_string(),
        }
    }
}

// Thử FromFake
trait FromFake<T> {
    fn from_fake(value: T) -> Self;

}

impl FromFake<Box<dyn std::error::Error>> for String {
    fn from_fake(value: Box<dyn std::error::Error>) -> Self {
        value.to_string()
    }
}

struct Student {
    grade: u8,
    name: String
}

impl std::fmt::Display for Student {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //write!(f, "{},{}", self.grade, self.name)
        format!("{},{}", self.grade, self.name).fmt(f)
    }
}


trait Draw {
    fn draw(&self) -> String;
}

impl Draw for u8 {
    fn draw(&self) -> String {
        format!("u8: {}", *self)
    }
}

impl Draw for f64 {
    fn draw(&self) -> String {
        format!("f64: {}", *self)
    }
}

fn draw_with_box(x: Box<dyn Draw>) {
    x.draw();
}

fn draw_with_ref(x: &dyn Draw) {
    x.draw();
}


// Overview 
// Phân biệt Static Dispatch: Trait Bound; và Dynamic Dispatch: dyn
// Cách implement 1 trait nào đó (từ thư viện) hoặc tự customized
// Đã có ví dụ với From, To


// có 2 cách để mình đưa ra 1 bài tập tổng hợp đầy đủ 
// cách 1:  mình làm sẵn hết , mình đưa ra unit test -> mn viết logic
// cách 2 : chỉ đưa ra yêu cầu và gợi ý -> tự mn implement
