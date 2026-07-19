// Day 1842: Majority / most-frequent element via a frequency count.
// (Equals the strict majority element whenever one exists.) Time O(N), Space O(N).
package main

import "fmt"

func majority(a []int) int {
	freq := map[int]int{}
	best, bestCount := a[0], 0
	for _, x := range a {
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
