// Day 597: Detect a Pythagorean triplet a^2 + b^2 = c^2 in an array.
// Approach: square values, sort, fix c as the largest, two-pointer. Time O(n^2), Space O(n).
package main

import (
	"fmt"
	"sort"
)

func hasPythagoreanTriplet(nums []int) bool {
	sq := make([]int, len(nums))
	for i, v := range nums {
		sq[i] = v * v
	}
	sort.Ints(sq)
	n := len(sq)
	for c := n - 1; c >= 2; c-- {
		lo, hi := 0, c-1
		for lo < hi {
			s := sq[lo] + sq[hi]
			if s == sq[c] {
				return true
			} else if s < sq[c] {
				lo++
			} else {
				hi--
			}
		}
	}
	return false
}

func main() {
	arr := []int{3, 1, 4, 6, 5} // contains 3,4,5
	fmt.Println(hasPythagoreanTriplet(arr))
}
