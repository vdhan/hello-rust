use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn main() {
    let mut x = 5;
    x += 1;

    let y = "thế giới";
    println!("Xin chào {y}\nBiến x: {x}");
    println!("Kiểu dữ liệu x: {}", type_of(x));
}
