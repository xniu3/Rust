//use std::io::{self, Read};//prelude

fn main() {
    println!("Start guesssing!");
    // let foo = 9;
    // let bar = foo;// immutable variables 
    let mut guess = String::new(); // variables mutable
    // fixed 
    std::io::stdin().read_line(&mut guess).expect("Cannot read the line");
    //read_line返回的是一个io::Result类型的变量，取值包括Ok和Err
    //如果Expect接收到的是Err则expect方法会中断当前程序并显示字符串信息
    //如果接收到的是Ok，则将Ok中附加的值返回给用户。
    //ensure the usage of buffer to be safe. 
    println!("You guessed {}",guess);

}
