// Day 1110: Detect a Pythagorean triplet a^2+b^2=c^2 in an array.
// Square all, sort; for each largest c use two-pointer scan. Time: O(N^2). Space: O(N).
package main

import (
	"fmt"
	"sort"
)

func hasTriplet(arr []int) bool {
	sq := make([]int, len(arr))
	for i, x := range arr {
		sq[i] = x * x
	}
	sort.Ints(sq)
	n := len(sq)
	for c := n - 1; c >= 2; c-- {
		l, r := 0, c-1
		for l < r {
			s := sq[l] + sq[r]
			if s == sq[c] {
				return true
			}
			if s < sq[c] {
				l++
			} else {
				r--
			}
		}
	}
	return false
}

func main() {
	fmt.Println(hasTriplet([]int{3, 1, 4, 6, 5}))   // true
	fmt.Println(hasTriplet([]int{10, 4, 6, 12, 5})) // false
}
