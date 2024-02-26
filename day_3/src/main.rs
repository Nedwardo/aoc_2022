use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
    vec,
};

fn main() {
    const FILENAME: &str = "data.txt";
    let file_contents = read_file_to_list(&FILENAME).unwrap();
    let result = solve(file_contents);

    println!("Result: {:#?}", result);
}

fn read_file_to_list<P>(filename: &P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let file_contents = Ok(io::BufReader::new(file).lines());

    return file_contents;
}

fn solve(file_contents: io::Lines<io::BufReader<File>>) -> u32 {
    let priority = file_contents
        .fold(vec![], |acc, item| split_into_3(acc, item.unwrap()))
        .iter()
        .map(get_duplicate_values)
        .sum();
    return priority;
}

fn get_duplicate_values(lists: &Vec<String>) -> u32 {
    let mut letters = vec![1; 52];
    const PRIMES: [u32; 3] = [2, 3, 5];
    const LCM: u32 = 30;
    for (i, list) in lists.iter().enumerate() {
        for item in list.chars() {
            let index = (get_value_of_char(item) - 1) as usize;
            letters[index] *= PRIMES[i];
            if letters[index] % LCM == 0 {
                return (index + 1) as u32;
            }
        }
    }
    return 0;
}

fn get_value_of_char(c: char) -> u32 {
    if !c.is_alphabetic() {
        return 0;
    }
    if c.is_uppercase() {
        return c as u32 - 38;
    }
    return c as u32 - 96;
}

fn split_into_3<'a>(acc: Vec<Vec<String>>, item: String) -> Vec<Vec<String>> {
    let mut new_acc = acc.clone();
    if acc.len() == 0 {
        new_acc.push(vec![item]);
        return new_acc;
    }
    if acc[acc.len() - 1].len() == 3 {
        new_acc.push(vec![item]);
        return new_acc;
    }
    let mut last = new_acc.pop().unwrap();
    last.push(item);
    new_acc.push(last);
    return new_acc;
}
