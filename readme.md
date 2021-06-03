# YOUTUBE

## <https://www.youtube.com/watch?v=zF34dRivLOw>

## Comandi

`rustup --version`
`rustc --version`
`cargo --version`

rustc hello.rs 

Compila il file in exe

## Se voglio la struttura

`cargo init`

## Se voglio compilare

`cargo run`

Compila in Running 

`target\debug\rust.exe`

Compila

`cargo build`

Compila in release

`cargo build --release`

`target\release\rust.exe`

## Usare funzioni in file separati

Creo un file `print.rs`

```rust
pub fn run(){
  println!("Hello from ")
}
```

nel file principale metto:

```rust
mod print;

fn main() {
    print::run();
}
```

## Funzione println

Devo mettere sempre un placeholder

```rust
// Basic formatting
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
  
  19:18
```

