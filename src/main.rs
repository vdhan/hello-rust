use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    return type_name::<T>();
}

fn main() {
    let mut x = 5; // x có thể thay đổi giá trị
    x += 1;

    let y = "thế giới";
    let z = 'a';
    let a = 2u8;
    let b = a as i16;

    println!("Xin chào {y}\nBiến x: {x}");
    println!("Kiểu dữ liệu z: {}", type_of(z));
    println!("Biến khôna âm: {}", type_of(a));
    print!("b: {b} ");

    if i16::from(a) == b {
        println!("bằng a");
    } else {
        println!("không bằng a");
    }

    if x < 0 {
        println!("Nhỏ hơn 0");
    } else if x == 0 {
        println!("Bằng 0");
    } else {
        println!("Lớn hơn 0");
    }

    if a > 0 && a <= 10 {
        println!("a trong khoảng 0..10");
    } else {
        println!("a ngoài khoảng 0..10");
    }
}
