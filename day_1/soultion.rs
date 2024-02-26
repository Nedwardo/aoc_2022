use std::fs;
fn main() {
    // Statements here are executed when the compiled binary is called.

    // Print text to the console.
    let result_ = read_file("data.txt");
    println!("{:#?}", result_);
}

fn read_file(filename: &str) -> i32 {
    let file_contents =
        fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut results = file_contents.lines().fold(vec![0; 1], reducer);
    results.sort_by(|a, b| b.cmp(a));
    let result = results[0..3].iter().sum();
    return result;
}

fn reducer(acc: Vec<i32>, x: &str) -> Vec<i32> {
    let mut mut_acc = acc.clone();
    if x == "" {
        mut_acc.push(0);
    } else {
        let num: i32 = x.parse::<i32>().unwrap();
        let last = mut_acc.len() - 1;
        mut_acc[last] += num;
    }
    return mut_acc;
}
