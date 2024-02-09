pub fn sort(slice: &mut [i32]) {
    if slice.len() < 2 {
        return;
    }
    let pivot = partition(slice);
    sort(&mut slice[0..pivot]);
    sort(&mut slice[pivot + 1..]);
}

fn partition(slice: &mut [i32]) -> usize {
    let lo = 0usize;
    let hi = slice.len() - 1usize;
    let pivot = slice[hi];
    let mut i = lo as i32 - 1;

    for j in lo..hi {
        if slice[j] <= pivot {
            i += 1;
            (slice[i as usize], slice[j]) = (slice[j], slice[i as usize]);
        }
    }

    i += 1;
    (slice[i as usize], slice[hi]) = (slice[hi], slice[i as usize]);
    return i as usize;
}
