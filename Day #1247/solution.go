// Squares of a sorted array via two-pointer merge from both ends. O(n) time, O(n) space.
package main

import (
	"fmt"
	"strconv"
	"strings"
)

func sortedSquares(a []int) []int {
	n := len(a)
	res := make([]int, n)
	l, r := 0, n-1
	for i := n - 1; i >= 0; i-- {
		ls, rs := a[l]*a[l], a[r]*a[r]
		if ls > rs {
			res[i] = ls
			l++
		} else {
			res[i] = rs
			r--
		}
	}
	return res
}

func main() {
	input := []int{-9, -2, 0, 2, 3}
	res := sortedSquares(input)
	parts := make([]string, len(res))
	for i, v := range res {
		parts[i] = strconv.Itoa(v)
	}
	fmt.Println("[" + strings.Join(parts, ", ") + "]")
}
