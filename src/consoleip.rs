use std::io;

fn main(){
    let mut word = String::new();

    io::stdin().read_line(&mut word).expect("error occured at line 6");

    println!("{}",word[0]);
}