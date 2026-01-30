// Day 993: Majority element (the value occurring more than floor(n/2) times).
// Count occurrences in a hash map and return the most frequent value. This also
// yields the expected answer for the README's (non-strict) example. O(n) time/space.
package main

import "fmt"

func majority(nums []int) int {
	freq := make(map[int]int)
	best, bestCount := nums[0], 0
	for _, x := range nums {
		freq[x]++
		if freq[x] > bestCount {
			bestCount = freq[x]
			best = x
		}
	}
	return best
}

func main() {
	fmt.Println(majority([]int{1, 2, 1, 1, 3, 4, 0})) // 1
}
