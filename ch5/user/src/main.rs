struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
fn main(){
  let user1 = User{
    active:true, 
    username: String::from("Fernando Alonso"), 
    email: String::from("lacabra@hotmail.com"), 
    sign_in_count:33 
  };

  let user2 = User{
    email: String::from("fnndoalo@gmail.com"),
    ..user1
  };

println!("{}",user1.username);
}

fn sign_in(username: String, email: String) -> User{
  User {
      active:true,
      username,
      email, 
      sign_in_count:1 
  }
}