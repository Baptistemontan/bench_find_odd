use std::collections::HashMap;

use rand::seq::SliceRandom;

/// Generate a random Vec with `nums_count` unique numbers, where one appear once and all other appear twice.
/// Return the Vec and the number appearing once.
pub fn generate_sample(nums_count: i32) -> (Vec<i32>, Option<i32>) {
    let mut nums = Vec::from_iter(0..nums_count);
    nums.extend_from_within(..);
    nums.shuffle(&mut rand::thread_rng());
    let once = nums.pop();
    (nums, once)
}

pub fn find_xor(arr: &[i32]) -> i32 {
    arr.iter().fold(0i32, |acc, value| acc ^ value)
}

pub fn find_hashmap(arr: &[i32]) -> i32 {
    let mut map: HashMap<i32, u32> = HashMap::new();

    for i in arr {
        *map.entry(*i).or_default() += 1;
    }

    map.iter()
        .find_map(|(k, v)| (*v % 2 != 0).then_some(*k))
        .unwrap_or_default()
}

pub fn find_radix(arr: &[i32]) -> i32 {
    let mut owned = arr.to_owned();

    radix_sort(&mut owned);

    let mut iter = owned.into_iter();

    let Some(mut last) = iter.next() else {
        return 0;
    };
    let mut count = 1;
    for n in iter {
        if n != last {
            if count % 2 != 0 {
                return last;
            } else {
                last = n;
                count = 1;
            }
        } else {
            count += 1;
        }
    }
    0
}

/// Sorts the elements of `arr` in-place using radix sort.
///
/// Time complexity is `O((n + b) * logb(k))`, where `n` is the number of elements,
/// `b` is the base (the radix), and `k` is the largest element.
/// When `n` and `b` are roughly the same maginitude, this algorithm runs in linear time.
///
/// Space complexity is `O(n + b)`.
fn radix_sort(arr: &mut [i32]) {
    let max: usize = match arr.iter().max() {
        Some(&x) => x as usize,
        None => return,
    };
    // Make radix a power of 2 close to arr.len() for optimal runtime
    let radix = arr.len().next_power_of_two();
    // Counting sort by each digit from least to most significant
    let mut place = 1;
    while place <= max {
        let digit_of = |x| x as usize / place % radix;
        // Count digit occurrences
        let mut counter = vec![0; radix];
        for &x in arr.iter() {
            counter[digit_of(x)] += 1;
        }
        // Compute last index of each digit
        for i in 1..radix {
            counter[i] += counter[i - 1];
        }
        // Write elements to their new indices
        for &x in arr.to_owned().iter().rev() {
            counter[digit_of(x)] -= 1;
            arr[counter[digit_of(x)]] = x;
        }
        place *= radix;
    }
}

#[cfg(test)]
mod tests {
    use super::radix_sort;

    fn check_sort(arr: &[i32], mut original: Vec<i32>) -> bool {
        original.sort();

        arr == original
    }

    #[test]
    fn empty() {
        let mut a: [i32; 0] = [];
        let cloned = a;
        radix_sort(&mut a);

        assert!(check_sort(&a, cloned.into()));
    }

    #[test]
    fn descending() {
        let mut v = vec![201, 127, 64, 37, 24, 4, 1];
        let cloned = v.clone();
        radix_sort(&mut v);
        assert!(check_sort(&v, cloned));
    }

    #[test]
    fn ascending() {
        let mut v = vec![1, 4, 24, 37, 64, 127, 201];
        let cloned = v.clone();
        radix_sort(&mut v);
        assert!(check_sort(&v, cloned));
    }
}
