fn main(){
    let i = [13,24,35];
    let iter = IntoIterator::into_iter(i);
    let iter_mapped = iter.inspect(|&x|
    println!("Pre:\t{}",x))
    .map(|x| x * 10)
    .inspect(|&x|println!("First:\t{}",x))
    .map(|x| x + 5)
    .inspect(|&x|println!("Second:\t{}",x));
    iter_mapped.collect::<Vec<i32>>();

}