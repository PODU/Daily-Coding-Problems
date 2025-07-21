// Two-sum existence: one pass with a hash set of complements.
// Time: O(n), Space: O(n).
package main

import "fmt"

func twoSum(nums []int, k int) bool {
	seen := make(map[int]struct{})
	for _, x := range nums {
		if _, ok := seen[k-x]; ok {
			return true
		}
		seen[x] = struct{}{}
	}
	return false
}

func main() {
	fmt.Println(twoSum([]int{10, 15, 3, 7}, 17))
}
