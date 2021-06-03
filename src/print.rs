pub fn run(){
  println!("Hello from print.rs file");

  println!("Number: {}",1);

  // Positional argument
  println!("{0} is from {1}", "Brad", "Mass");

  //Named arguments
  println!("{name} likes to play {activity}", name = "John", activity = "Baseball");

  //Placeholder traits
  println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

  //Placeholder for debug trait
  println!("{:?}",(12,true,"hello"));

  //Basic math
  println!("10 + 10 = {}", 10+10);
}