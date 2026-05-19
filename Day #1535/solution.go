// Two numbers summing to k via a single-pass hash set.
// Time O(n), Space O(n).
package main

import "fmt"

func hasPairSum(nums []int, k int) bool {
	seen := make(map[int]bool)
	for _, x := range nums {
		if seen[k-x] {
			return true
		}
		seen[x] = true
	}
	return false
}

func main() {
	fmt.Println(hasPairSum([]int{10, 15, 3, 7}, 17))
}
