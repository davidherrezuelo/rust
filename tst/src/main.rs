fn main(){
  let sentence=String::from("Hello world :)");
  let fword=first_word(&sentence);
  print!("{fword}");

}

fn first_word(s: &String) -> usize {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
      if item == b' ' {
          return i;
      }
  }

  s.len()
}