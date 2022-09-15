fn main() {
    let i = [12,34];
    let iter = IntoIterator::into_iter(i);
    let iter_mapped = iter.map(|x| x * 2);
    let out = iter_mapped.collect::<Vec<i32>>();
    println!("{:?}",out);
}