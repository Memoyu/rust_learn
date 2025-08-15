use crate::sort::*;

pub mod sort;

fn main() {
    // let mut nums = [4, 1, 3, 1, 5, 2];
    // selection_sort(&mut nums);
    // println!("{:?}", nums);

    // let mut nums = [4, 1, 3, 1, 5, 2];
    // bubble_sort(&mut nums);
    // println!("{:?}", nums);

    // let mut nums = [
    //     21, 41, 80, 28, 57, 4, 35, 96, 51, 95, 45, 37, 25, 44, 39, 79, 97, 27, 5, 84,
    // ];
    // insertion_sort(&mut nums);
    // println!("{:?}", nums);

    let mut nums = [2, 4, 1, 0, 3, 5];
    // let mut nums = [6, 5, 4, 3, 2, 1, 0];
    quick_sort(&mut nums);
    println!("{:?}", nums);
}
