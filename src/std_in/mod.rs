use std::process;
use std::io;

pub fn function() {
  println!("Hello, world!");
  var_mut();

  let first = "First".to_string();
  let last = "Last".to_string();

  /*
  Pass down the variables references and not themselves.
  */
  say_with_no_ownership(&first, &last);
  /*
  Pass down variables like this, we are passing down the ownership of them
  */
  say_with_ownership(first, last);
  // no more access to those variables
  // println!("{}", first); we have no more access to it

  let phrase = read_from_stdin(&"Enter a phrase".to_string());

  // exit a running script
  if phrase.trim() == "exit" {
    process::exit(1);
  }

  loop {
    let first = read_from_stdin(&"Enter a number".to_string());
    let second = read_from_stdin(&"Enter a number".to_string());

    // do not handle errors, if they happen the app crashes with unwrap
    // let a:u32 = number.trim().parse().unwrap();
    // let b:u32 = numberB.trim().parse().unwrap();

    // proper way of handling errors with useful messages
    //let a:u32 = number.trim().parse().expect("This should be a number");
    let a: u32 = safe_parse(first);
    let b: u32 = safe_parse(second);
    let result = sum(a, b);
    println!("{} + {} = {}", a, b, result);

    if a == 0 || b == 0 {
      process::exit(1);
    }
  }
}

/** Parse str to number with error handling */
fn safe_parse(val: String) -> u32 {
  match val.trim().parse() {
    Ok(r) => {
      return r;
    }
    // underscore variables indicate that we are not going to use it.
    Err(_err) => {
      println!("Not a valid number {}", val);
      return 0;
    }
  }
}

// -> returns something
fn sum(a: u32, b: u32) -> u32 {
  return a + b;
}

fn read_from_stdin(msg: &String) -> String {
  println!("{}:", msg);

  // mutable variable to listen to stdin
  let mut bla = String::new();

  // mutable reference passed to read_line
  io::stdin().read_line(&mut bla).unwrap();

  println!("received {}", bla);

  return bla.to_string();
}

fn say_with_no_ownership(first: &String, last: &String) {
  println!("{} {}", first, last);
}

fn say_with_ownership(first: String, last: String) {
  println!("{} and {}", first, last);
}

fn var_mut() {
  // Immutable variables
  let immut = "foo";

  /* Mutable variables `mut` */
  let mut name = "Franco";

  println!("{} something {}", name, immut);

  name = "bla";

  println!("{}", name);
  // var = "baz" Not allowed.
}
