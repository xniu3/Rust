use std::vec::Vec;

fn main(){
    let mut vec = Vec::new();
    println!("the result is {}",sum(map(vec)));
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
