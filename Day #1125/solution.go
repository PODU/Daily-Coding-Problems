// Day 1125 - Contiguous sublist summing to K
// Prefix sums + hash map (handles negatives) to find a range with sum == K in
// one pass. Time: O(n), Space: O(n).
package main

import "fmt"

func subarraySum(nums []int, k int) []int {
	seen := map[int]int{0: -1}
	total := 0
	for j, x := range nums {
		total += x
		if i, ok := seen[total-k]; ok {
			return nums[i+1 : j+1]
		}
		if _, ok := seen[total]; !ok {
			seen[total] = j
		}
	}
	return nil
}

func main() {
	fmt.Println(subarraySum([]int{1, 2, 3, 4, 5}, 9)) // [2 3 4]
}
