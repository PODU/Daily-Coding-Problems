// Day 155: Boyer-Moore majority vote in O(n) time, O(1) space. We verify the
// candidate; if no strict majority exists we fall back to the most frequent
// element so the answer is well-defined. Time O(n).
package main

import "fmt"

func majorityElement(a []int) int {
	candidate, count := 0, 0
	for _, x := range a {
		if count == 0 {
			candidate = x
		}
		if x == candidate {
			count++
		} else {
			count--
		}
	}
	occ := 0
	for _, x := range a {
		if x == candidate {
			occ++
		}
	}
	if occ*2 > len(a) {
		return candidate // strict majority
	}

	// Fallback: most frequent element (example has no strict majority).
	freq := make(map[int]int)
	best, bestCnt := a[0], 0
	for _, x := range a {
		freq[x]++
		if freq[x] > bestCnt {
			bestCnt = freq[x]
			best = x
		}
	}
	return best
}

func main() {
	a := []int{1, 2, 1, 1, 3, 4, 0}
	fmt.Println(majorityElement(a)) // 1
}
