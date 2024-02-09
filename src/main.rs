
mod bubble_sort;
mod quick_sort;
mod core; // If you decide to use a separate module for core functions and structs

use core::{get_i32, make_random_vec, print_vec, check_sorted}; // Adjust path based on your structure

fn main() {
    let items = get_i32("# Items: ");
    let max = get_i32("max: ");

    let mut vec = make_random_vec(items, max);
    print_vec(&vec, 40);

    // Example of using bubble_sort
    quick_sort::sort(&mut vec); // Assuming your bubble_sort module exposes a function named `sort`
    print_vec(&vec, 40);
    check_sorted(&vec);
}

