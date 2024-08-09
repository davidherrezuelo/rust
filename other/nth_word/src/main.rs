fn main(){
    let sentence = String::from("Hello world, how are you today?");
    let word = first_word(&sentence);
    let word2 = second_word(&sentence);
    let word4 = nth_word(&sentence, 4);
    println!("{sentence}");
    println!("{word}");
    println!("{word2}");
    println!("{word4}");
  }
  
  fn first_word(s:&String) -> &str{
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate(){
      if item == b' '{
        return &s[..i];
      }
    }
    &s[..]
  }
  
  fn second_word(s:&String) -> &str{
    let bytes = s.as_bytes();
    let mut word_index:i16=0;
    let mut last_space_index=0;
    for (i, &item) in bytes.iter().enumerate(){
      if item == b' '{
        word_index +=1;
        if word_index==2{
          return &s[last_space_index+1..i]
        }
        last_space_index=i;
      }
    }
    &s[..]
  }
  
  fn nth_word(s:&String, n:i16) -> &str{
    let bytes = s.as_bytes();
    let mut word_index:i16=0;
    let mut last_space_index=0;
    for (i, &item) in bytes.iter().enumerate(){
      if item == b' '{
        word_index +=1;
        if word_index==n{
          return &s[last_space_index+1..i]
        }
        last_space_index=i;
      }
    }
    &s[..]
  }