struct Rectangle (u32, u32);

fn main(){
    let rectangle = Rectangle(30,50);
    let area = area(&rectangle);
    println!("{}",area)
}

fn area(&Rectangle(width, height): &Rectangle) -> u32 {
    width * height
}