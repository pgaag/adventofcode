use std::{fs};

fn main() {
    // --snip--
    println!("In file {}", "../data/measurements.txt");

    let contents = fs::read_to_string("../data/measurements.txt")
        .expect("Something went wrong reading the file");
    let split = contents.split('\n');

    let mut vec = split.collect::<Vec<&str>>();

    vec.remove(vec.len() - 1);

    let mut vec_ints = Vec::new();

    for s in vec {
        vec_ints.push(s.parse::<i32>().unwrap());
    }

    let mut count = 0;
    let mut old_sum = 0;
    for num in 0..vec_ints.len(){
        if num + 3 < vec_ints.len() {
            let one:i32 = *vec_ints.get(num).unwrap();
            let two:i32 = *vec_ints.get(num + 1).unwrap();
            let three:i32 = *vec_ints.get(num + 2).unwrap();
            let sum = one + two + three;
            println!("{:#?}", sum);
            if sum > old_sum {
                count = count + 1;
                
            }
            old_sum = sum;
        }
    }

    println!("{:#?}", count);
}