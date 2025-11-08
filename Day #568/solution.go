// Square a sorted array and return sorted. Two pointers from both ends pick larger
// absolute value into the back of the result. O(N) time, O(N) space.
package main

import (
	"fmt"
	"strings"
)

func sortedSquares(a []int) []int {
	n := len(a)
	l, r := 0, n-1
	res := make([]int, n)
	for k := n - 1; k >= 0; k-- {
		lv, rv := a[l]*a[l], a[r]*a[r]
		if lv > rv {
			res[k] = lv
			l++
		} else {
			res[k] = rv
			r--
		}
	}
	return res
}

func main() {
	res := sortedSquares([]int{-9, -2, 0, 2, 3})
	parts := make([]string, len(res))
	for i, v := range res {
		parts[i] = fmt.Sprintf("%d", v)
	}
	fmt.Println("[" + strings.Join(parts, ", ") + "]") // [0, 4, 4, 9, 81]
}
