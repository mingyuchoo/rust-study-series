pub fn if_conditions() {
  let number = 3;

  if number < 5 {
    println!("조건이 일치합니다.");
  } else {
    println!("조건이 일치하지 않습니다.");
  }

  if number != 0 {
    println!("변수에 저장된 값이 0이 아닙니다.");
  }
}

pub fn let_and_if() {
  let condition = true;
  let number = if condition { 5 } else { 6 };

  println!("number의 값: {}", number);
}

pub fn loops() {
  let mut counter = 0;
  let result = loop {
    counter += 1;

    if counter == 10 {
      break counter * 2;
    }
  };

  println!("result의 값: {}", result);
}

pub fn whiles() {
  let mut number = 3;
  while number != 0 {
    println!("{}!", number);
    number -= 1;
  }
  println!("발사!");
}

pub fn for_element() {
  let a = [10, 20, 30, 40, 50];
  for element in a.iter() {
    println!("요솟값: {}", element);
  }
}

pub fn for_rev() {
  for number in (1..4).rev() {
    println!("{}!", number);
  }
  println!("발사!");
}