// Day 1022: Single number where all others appear 3 times.
// Approach: ones/twos bitmask accumulators. Time O(N), Space O(1).
package main

import "fmt"

func singleNumber(nums []int) int {
	ones, twos := 0, 0
	for _, x := range nums {
		ones = (ones ^ x) &^ twos
		twos = (twos ^ x) &^ ones
	}
	return ones
}

func main() {
	tests := [][]int{{6, 1, 3, 3, 3, 6, 6}, {13, 19, 13, 13}}
	for _, t := range tests {
		fmt.Println(singleNumber(t))
	}
}
