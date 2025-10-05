
fn main() {
    // println!("Hello, world!");
    
    let a:Vec<u64> = vec![1, 3, 5];
    // let b:Vec<u32> = a.iter()
    let b = a.iter()
        .position(|&a| a > 10)
        // .enumerate()
        // .filter(|p| p.0 >= 1)
        // .map(|p| *p.1)
        // .map(|(p, &a)| a as u32 + 1)
        // .collect();
        .map(|p| p as u32)
        ;
    println!("{:#?}", b);
}
