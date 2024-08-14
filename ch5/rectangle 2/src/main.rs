struct Rectangle {
    width:u32,
    height:u32
}
impl Rectangle{
    fn can_hold (&self, rctngl:&Rectangle)-> bool{
        if self.width >= rctngl.width && self.height >= rctngl.height{
            return true
        }
        false
    }
    fn area(&self) -> u32 {
        self.height * self.width
    }
}

fn main(){
    let rectangle1 = Rectangle{width:40, height:30};
    let rectangle2 = Rectangle{width:30, height:20};
    let area = rectangle1.area();
    println!("{}",area);
    if rectangle1.can_hold(&rectangle2){
        println!("rectangle1 can hold rectangle2");
    }
}

