// Day 1431: Majority element (appears > floor(n/2)).
// Approach: Boyer-Moore voting. Time: O(n), Space: O(1).

fn majority_element(nums: &[i32]) -> i32 {
    let mut count = 0;
    let mut candidate = 0;
    for &x in nums {
        if count == 0 {
            candidate = x;
        }
        if x == candidate {
            count += 1;
        } else {
            count -= 1;
        }
    }
    candidate
}

fn main() {
    println!("{}", majority_element(&[1, 2, 1, 1, 3, 4, 0])); // 1
}
