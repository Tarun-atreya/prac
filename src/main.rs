use std::io::{stdin, Write, stdout};
use rand::Rng;

fn part1() -> Vec<Vec<i32>> {
    let mut value = String::new();
    print!("Enter 2 integers (space separated): ");
    stdout().flush().unwrap();
    stdin().read_line(&mut value).unwrap();
    

    let mut nums = value.trim().split_whitespace();
    let w = nums.next().unwrap().parse::<usize>().unwrap();
    let h = nums.next().unwrap().parse::<usize>().unwrap();

    let mut array2d: Vec<Vec<i32>> = vec!(vec!(0; w); h);

    for i in 0..array2d.len(){
        for j in 0..array2d[i].len(){
            array2d[i][j] = rand::thread_rng().gen_range(0..10);
        }
    }

    return array2d; 
}

fn method2(array2d: &Vec<Vec<i32>>) -> i32 {
    for i in 0..array2d.len(){
        for j in 1..array2d[i].len() - 1 {
            if array2d[i][j - 1] + array2d[i][j + 1] > 17 && array2d[i][j] % 2 == 0 {
                return array2d[i][j];
            }
        }
    }

    return -1; 
}

fn method1(array2d: &Vec<Vec<i32>>) -> i32 {
    for c in array2d {
        for num in c {
            if num % 2 == 0 {
                return *num; 
            }
        }
    }

    return -1; 
}

fn part2(array2d: Vec<Vec<i32>>) {
    println!("Method 1: {}", method1(&array2d));
}   

fn main() {
    let array2d = part1();
    part2(array2d);
}
