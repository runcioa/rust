pub fn run() {
  let name = "Brad";
  let age = 37; //immutable

  //per farla mutable

  let mut age1 = 45;

  age1 = 55;

  println!("My name is {} and i am {}", name, age);

  //Define costant
  const ID: i32 = 001;

  println!("ID: {}", ID);

  let (my_name, my_age) = ("Brad", 37);
  println!("{} is {}", my_name, my_age);
}
