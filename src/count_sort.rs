use crate::core::Customer;

pub(crate) fn sort(slice: &mut [Customer], max: i32) -> Vec<Customer> {
    let num_items = slice.len();

    // Make a vector to hold counts. Initialize with 0s.
    let mut counts = vec![0i32; max as usize];

    // Count the values.
    for i in 0..num_items {
        counts[slice[i].num_purchases as usize] += 1;
    }

    // Convert counts into counts of values <=.
    for i in 1usize..max as usize {
        counts[i] += counts[i - 1];
    }

    // Count out the values.
    let mut result: Vec<Customer> = Vec::with_capacity(num_items as usize);
    result.resize(num_items, Customer::default());
    for i in (0..num_items).rev() {
        // Copy item i into position.
        let num = slice[i].num_purchases as usize;
        result[(counts[num] - 1) as usize] =
            Customer {
                id: slice[i].id.clone(),
                num_purchases: slice[i].num_purchases
            };
        counts[num] -= 1;
    }

    return result;
}