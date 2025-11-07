// Two-sum existence: one pass with a hash set of seen numbers.
// Time: O(N), Space: O(N).
package main

import "fmt"

func hasPairWithSum(nums []int, k int) bool {
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
	nums := []int{10, 15, 3, 7}
	k := 17
	fmt.Println(hasPairWithSum(nums, k))
}
