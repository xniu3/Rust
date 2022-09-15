use rand::prelude::*;
use apint::Int;
use apint::ApInt;
use primes::is_prime;
use std::vec::Vec;


fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    /* 
    let mut rng = thread_rng();
    // rng.gen is a generic function randomly generate True or False
    // That's why sometimes the results are true, and sometimes false. 
    if rng.gen() {
        let x: f64 = rng.gen();
        let y = rng.gen_range(-10.0, 10.0);
        println!("x is: {}", x);
        println!("y is: {}", y);
        println!("Random Number between 0 and 9: {}", rng.gen_range(0, 10));
    }
    else{
        println!("rng.gen() is false");
    }
    */
}
    /*
    let n = 10;
    let mut candidate:Int = Int::from(ApInt::from_i32(rng.gen_range(0, n)));
    println!("candidate is {}",candidate.resize_to_i32());
    candidate.set_bit_at(0);
    let i = candidate.resize_to_i64()as u64;
    println!("candidate is {}",i);
    println!("candidate is {}",is_prime(i));
    //println!("isprime is {}",isprime);
    */
