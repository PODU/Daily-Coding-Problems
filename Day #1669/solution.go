// Two-pointer merge from both ends of sorted array; larger abs square goes to back. O(n) time, O(n) space.
package main

import (
	"fmt"
	"strings"
	"strconv"
)

func sortedSquares(a []int) []int {
	n := len(a); r := make([]int, n); l, h := 0, n-1
	for p := n - 1; p >= 0; p-- {
		lo, hi := a[l]*a[l], a[h]*a[h]
		if lo > hi { r[p] = lo; l++ } else { r[p] = hi; h-- }
	}
	return r
}
func main() {
	r := sortedSquares([]int{-9, -2, 0, 2, 3})
	s := make([]string, len(r))
	for i, v := range r { s[i] = strconv.Itoa(v) }
	fmt.Println("[" + strings.Join(s, ", ") + "]")
}
