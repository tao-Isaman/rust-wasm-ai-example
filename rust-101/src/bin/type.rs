fn main(){
    println!("This code will be learn about type and variable");
    println!("เมื่อประกาศค่าตัวแปรจะใช้คำสั่ง let เช่น let :f64 = 3.14159; ซึ่ง let เป็นตัวแปรที่เปลี่ยนแปลงได้");

    // type of rust
    println!("Bool -> true, false");
    println!("unsigned integers - u8 u16 u32 u64 u128");
    println!("signed integers - i8 i16 i32 i64 i128 สำหรับตัวเลขทั้งหมด");
    println!("pointer sized integers - usize isize สำหรับค่าดัชนี(index) และขนาดของ ของในหน่วยความจำ");
    println!("floating point - f32 f64");
    println!("tuple - (value, value, ...) สำหรับส่งของตามลำดับบน stack");
    println!("arrays - [value, value, ...] กลุ่มข้อมูลประเภทเดียวกันที่รู้ขนาดที่แน่นอนตั้งแต่ compile time");
    println!("slices - กลุ่มข้อมูลประเภทเดียวกันที่รู้ขนาดที่แน่นอนเมื่อตอน runtime");
    println!("str(string slice) - ข้อความ ที่รู้ขนาดที่แน่นอนเมื่อตอน runtime");

    // println with string
    const KEYWORD :&str = "this is ketword";

    println!("keyword should show on -> {}", KEYWORD);


}