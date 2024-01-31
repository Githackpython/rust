use std::io;

fn main() {
    let mut a=String::new();
    let mut b=String::new();
    io::stdin().read_line(&mut a).expect("error");
    io::stdin().read_line(&mut b).expect("error");
    print!(" \n login: {} password: {}",a,b);
    println!("verfy ");
    let mut c=String::new();
    let mut d=String::new();
    io::stdin().read_line(&mut c).expect("error");
    io::stdin().read_line(&mut d).expect("error");
    if a==c && b==d{
      print!("suckefull ")
    }
}

