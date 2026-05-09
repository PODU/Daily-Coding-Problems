// Detect a Pythagorean triplet a^2+b^2=c^2 in an array.
// Square + sort, then fix largest as c and two-pointer. Time O(n^2), Space O(1).
package main

import (
	"fmt"
	"sort"
)

func hasTriplet(nums []int) bool {
	sq := make([]int, len(nums))
	for i, x := range nums {
		sq[i] = x * x
	}
	sort.Ints(sq)
	n := len(sq)
	for c := n - 1; c >= 2; c-- {
		a, b := 0, c-1
		for a < b {
			s := sq[a] + sq[b]
			if s == sq[c] {
				return true
			}
			if s < sq[c] {
				a++
			} else {
				b--
			}
		}
	}
	return false
}

func main() {
	nums := []int{3, 1, 4, 6, 5}
	fmt.Println(hasTriplet(nums)) // expected: true
}
