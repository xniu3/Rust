fn main(){
    let i = [10,42,93];
    let iter = IntoIterator::into_iter(i);
    let iter_cycled = iter.cycle();
    let out = iter_cycled.take(9).collect::<Vec<i32>>();
    println!("{:?}",out);

}