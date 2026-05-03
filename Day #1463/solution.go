// Pythagorean triplet: square all, sort, for each largest c^2 two-pointer for a^2+b^2.
// Time: O(n^2), Space: O(n).
package main

import (
	"fmt"
	"sort"
)

func hasTriplet(input []int) bool {
	n := len(input)
	arr := make([]int, n)
	for i, x := range input {
		arr[i] = x * x
	}
	sort.Ints(arr)
	for c := n - 1; c >= 2; c-- {
		l, r := 0, c-1
		for l < r {
			s := arr[l] + arr[r]
			if s == arr[c] {
				return true
			} else if s < arr[c] {
				l++
			} else {
				r--
			}
		}
	}
	return false
}

func main() {
	if hasTriplet([]int{3, 1, 4, 6, 5}) {
		fmt.Println("True")
	} else {
		fmt.Println("False")
	}
	if hasTriplet([]int{10, 4, 6, 12, 5}) {
		fmt.Println("True")
	} else {
		fmt.Println("False")
	}
}
