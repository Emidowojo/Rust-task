// Compute the modular inverse of 42 modulo 2017 using Euclids Algorithm.
fn mod_inv(a: isize, module: isize) -> isize {
    //  To calculate the inverse we do as if we would calculate the GCD with the Euclid extended algorithm 
    let mut mn = (module, a); // Declare variable mn = (module, a);
    let mut xy = (0, 1); // Declare variable xy = (0, 1);
   
    while mn.1 != 0 { //  Using a while loop, while var mn multiplied by 1 is not equals to 0, 
      xy = (xy.1, xy.0 - (mn.0 / mn.1) * xy.1); // To find  var xy = xy multiplied by 1, xy multiplied by 0 subtracted from, var mn multiplied by 0 which is divied by mn multiplied by 1, an then multiplied by xy times 1
      mn = (mn.1, mn.0 % mn.1); // var mn = mn multiplied by 1, mn multiplioed by 0 modulo mn multiplied by 1
    }
   
    while xy.0 < 0 { // Using a while loop,  while xy multiplied by 0 is less than 0, xy multiplied by 0 is plus or equals the module or gcd 
      xy.0 += module;
    }
    xy.0 // else  if greater than 0, xy multiplies 0
  }
   
  fn main() {
    println!("{}", mod_inv(42, 2017)) // Input the parameters to calculate output or gcd
  }