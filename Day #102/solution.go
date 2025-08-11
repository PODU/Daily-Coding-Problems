// Day 102: Contiguous subarray summing to K via prefix sums + hashmap. For each
// prefix p look for p-K seen earlier; earliest-ending match. O(n) time.
package main

import "fmt"

func subarraySum(nums []int, k int) []int {
	first := map[int]int{0: -1}
	prefix := 0
	for j, x := range nums {
		prefix += x
		if i, ok := first[prefix-k]; ok {
			return nums[i+1 : j+1]
		}
		if _, ok := first[prefix]; !ok {
			first[prefix] = j
		}
	}
	return nil
}

func main() {
	fmt.Println(subarraySum([]int{1, 2, 3, 4, 5}, 9)) // [2 3 4]
}
