
mod bubble_sort;
mod quick_sort;
mod core;
mod count_sort; // If you decide to use a separate module for core functions and structs

use core::{get_i32, make_random_vec, print_vec, check_sorted}; // Adjust path based on your structure

fn main() {
    // Get the number of items and max value.
    let num_items = get_i32("# Items: ");
    let max = get_i32("Max: ");

    // Make and display the unsorted vector.
    let mut customers = make_random_vec(num_items, max);
    print_vec(&customers, 20);
    println!();

    // Sort and display the result.
    customers = count_sort::sort(&mut customers[..], max);
    print_vec(&customers, 20);

    // Verify that it's sorted.
    check_sorted(&customers);
}

