
//Control Flow
// use std::fmt::Display;
fn main() {
  // let number = 3;
  // let condition = if number < 5 {
  //   4
  // } else {
  //   44
  // };
  // println!("The value of number is: {condition:?}");

  // let mut counter = 0;

  // let result = loop {
  //   counter += 1;
    
  //   if counter == 10 {
  //     break counter * 2;
  //   }
  // };

  // println!("결과값은 {result}입니다.")

  let mut count = 0;
  'counting_up: loop {
    println!("count = {count}");
    let mut remaining = 10;

    loop {
      println!("remaining = {remaining}");
      if remaining == 9 {
        break;
      }
      if count == 2 {
        break 'counting_up;
      }
      remaining -= 1;
    }
    count += 1;
  }
  println!("End count = {count}");
}
