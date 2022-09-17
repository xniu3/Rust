use std::vec::Vec;
use std::f32::consts::PI;
/* 
fn main(){
    //let arr = [1,1,4,5,1,4];
    //println!("the result is {}",q7(&arr));
    let mut radius = 3;
    println!("the result is {}",sphere(radius));

}
*/
enum Colour {
    Red,
    Yellow,
    Blue,
    }
    fn say_colour(colour: &Colour) -> &'static str {
    match colour {
    Colour::Red => "red",
    Colour::Yellow => "yellow",
    //Colour::Blue => "yellow",
    }
    }
    fn main() {
        println!("{}", say_colour(&Colour::Blue));
    }
/* 
fn main(){
    let mut radius:f32 = 3.0;
    println!("the result is {}",sphere(radius));
}*/
fn sphere(mut radius:f32) -> f32{
    return 4.0 * 3.14 * cube(radius) / 3.0
}
fn cube(mut radius:f32) -> f32{
    return radius * radius * radius;
}

fn q7(mut arr:&[i32]) -> i32{
    let arr_iter = arr.into_iter();
    let arr_iter_mapped = arr_iter.map(|x| x * x + 2);
    let out:i32 = arr_iter_mapped.sum();
    return out;
}


fn sum(vec:Vec<i32>) -> i32{
    let mut total = 0;
    for n in vec.iter(){
        //println!("Prime {}: {}", ix, n);
        total += n;
    }
    return total;
}

fn map(mut vec:Vec<i32>) -> Vec<i32>{
    let mut vec1 = Vec::new();
    
    for n in vec.iter() {
        //println!("Prime {}: {}", ix, n);
        vec1.push(n * n + 2);
    }
    return vec1;
}
