
fn fibonacci(num:i64) -> i64 
{
    println!("{num}");
    if num==0{
        0
    }
    else if num==1{
        1
    }
    else{
        fibonacci(num-1)+fibonacci(num-2)
    }

}
fn main() {
    println!("Hello, world!");
    let x= fibonacci(30);
    print!("fibonacci 5 = {x}");
}
