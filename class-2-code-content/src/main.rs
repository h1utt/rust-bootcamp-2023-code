fn main() {
    // Mutable
    let mut s1 = String::from("Hello world");
    // s1 đang sở hữu dữ liệu "Hello world"
    // s1 = s1 + " Hieu";
    // println!("{}", s1);

    // Immutable
    let s2 = &s1[..];

    // Garbage Collector
    let s3 = s1;
    // Gán giá trị của s1 cho s3 và đồng thời s3 sẽ sở hữu dữ liệu "Hello world"
    // xóa s1 -> Drop s1
    // Theo nguyên tắc Ownership của Rust, 1 biến chỉ sỡ hữu 1 giá trị tương ứng
    // khi chuyển giá trị đó cho 1 biến khác => chuyển chủ sở hữu cho biến khác và 
    // biến cũ sẽ không còn là chủ sở hữu của giá trị đó nữa
    // let s4 = s1;
    // Vi phạm nguyên tắc của Ownership
    // Ownership: sở hữu

    // Ownership này không áp dụng cho tất cả các kiểu dữ liệu
    // Rust Primitives: u32, u64, ... -> Đầu vào biết trước: 32 bit, 64 bit
    // Collections: vector, string, ... -> Đầu vào chưa biết trước -> Bộ nhớ động

    // Bộ nhớ động: Sau mỗi lần chạy sẽ cấp phát bộ nhớ khác nhau

    // Stack và Heap
    // Stack: Địa chỉ ô nhớ để lưu giá trị cố định
    // Heap: Ô nhớ động

    // Compile time: Thời gian biên dịch
    // Run time: Thời gian chạy

    // Rust Primitives: Default là stack -> biết size
    // Collections: Không bỏ vào stack được bởi vì không biết size ở compile time -> Lưu ở Heap
    // Xét -> việc truy xuất dữ liệu -> Stack nhanh hơn

    // Biến: là tên để lưu giá trị -> lưu ở 1 vùng nhớ nào đó
    // Memory Management

    // Pointer: trỏ tới địa chỉ của vùng nhớ quản lý (có dấu `&` - theo định nghĩa của C++)
    let x = 42;
    let y = 43;
    let var1 = &x; // var1 là 1 pointer đang trở tới biến x
    println!("var: {:p}", var1);
    // Rust sẽ print thẳng ra giá trị, vì vậy cần phải có `:p` -> print ra địa chỉ

    let s3 = String::from("Hello ");
    let s4 = &s3;

    println!("s3's address: {:?}", s3.as_ptr());
    println!("s4's address: {:?}", s4.as_ptr());
    println!("s3: {}", s3);
    // Tham chiếu. s4 đang trỏ cùng 1 địa chỉ tới s3 -> Không ảnh hưởng đến giá trị -> Không vi phạm Ownership

    let s3 = String::from("Hello ");
    println!("s3's address: {:?}", s3.as_ptr());

    let s4 = s3;
    println!("s4's address: {:?}", s4.as_ptr()); // -> s4 vẫn cùng địa chỉ với s3, vẫn cùng cái nhà, chỉ khác chủ

    let mut s6 = String::from("Hell");
    println!("len: {}, capacity: {}", s6.len(), s6.capacity());
    s6.push_str("o");
    println!("len: {}, capacity: {}", s6.len(), s6.capacity());
    s6.push_str("World");
    println!("len: {}, capacity: {}", s6.len(), s6.capacity());
    s6.push_str("ab");
    println!("len: {}, capacity: {}", s6.len(), s6.capacity());
    s6.push_str("beautiful");
    println!("len: {}, capacity: {}", s6.len(), s6.capacity());
    // Runtime tự động cung cấp `capacity` cho mình

    let s7 = String::from("Hell");
    let s8 = s7;
    // println!("s7: {}", s7); // Lỗi Ownership vì cả s7 và s8 cùng trỏ vào `"Hell"`
    // Vậy khi out of scope thì xóa s7 hay s8? -> Ownership -> Drop s7

    // Vậy khi muốn sử dụng lại s7 => Làm thế nào? 
    // Cách 1: Sử dụng clone() -> Nhược điểm: Tốn bộ nhớ
    let s7 = String::from("Hell");
    let s8 = s7.clone(); // clone() -> Tạo ra 1 phiên bản copy của s7 
    // -> s7 và s8 không còn liên quan gì đến nhau nữa
    println!("s7: {}", s7);
    println!("s7's address: {:?}", s7.as_ptr());
    println!("s8's address: {:?}", s8.as_ptr());

    // Cách 2: Sử dụng tham chiếu (Reference/Borrowing) (Thường được sử dụng)
    let mut s7 = String::from("Hell");
    let s9 = &s7; // s7 thay đổi -> s9 thay đổi
    s7.push_str("World"); // s7 có quyền thay đổi
    // Shared reference: s9 đang mượn s7 -> Immutable
    // s9.push_str("World"); // s9 không có quyền thay đổi
    // println!("s9: {}", s9); // -> False
    let s10 = &s7;
    let s11 = &s7;
    // Có thể shared reference nhiều lần

    // Mutable reference
    // Có thể thay đổi giá trị, nhưng owner phải share quyền thay đổi
    let s12 = &mut s7;
    let s15 = &mut s7; // -> False: Chỉ có 1 biến được mượn tại 1 thời điểm <=> Chỉ 1 mutable reference cho 1 giá trị nhất định trong 1 thời điểm
    // s12.push_str("World");
    // println!("s12: {}", s12);
    println!("s7: {}", s7);

    let s13 = &s7;
    let s14 = &s13;

    let s13 = &s7;
    println!("s13: {}", s13);
    let s14 = &s7;
    let s15 = &s7;
    println!("s14: {}", s14);
    // => Nhiều immutable reference được cho phép sử dụng (đọc)

    // Không thể có mutable reference trong khi có 1 immutable reference tham chiếu tới cùng 1 giá trị
    // let s16 = &mut s7;
    // let s17 = &s7; // -> Lỗi
    // println!("s16: {}", s16);

    // {}: Scope
    let x = 10;
    // x: Global
    {
        // y: Local
        let y = "Hello";
        println!("{}", y); // => True
    };
    // println!("{}", y); => False
    // Rust: Out of scope -> Value drop

    function_a();
    // println!("z: {}", z); // -> False

    let mut s = String::from("hello");
    change(&mut s);
    // //println!("s2:{}",s2);
    println!("s: {}",s); // // s không còn tồn tại vì đã chuyển Ownership cho some_string
}

fn change(mut some_string: &mut String) {
    some_string.push_str(", world");
}

fn function_a() {
    println!("Hello");
    let z = 10;
}

// borrow checker 




