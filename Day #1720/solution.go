// Min perfect squares: Legendre/Lagrange three-square theorem gives the count in
// O(sqrt N); decomposition found greedily largest-square-first. Time O(sqrt N), Space O(1).
package main

import (
	"fmt"
	"math"
	"sort"
	"strings"
)

func isSquare(n int) bool {
	r := int(math.Sqrt(float64(n)))
	for r*r > n {
		r--
	}
	for (r+1)*(r+1) <= n {
		r++
	}
	return r*r == n
}

func minSquaresCount(n int) int {
	if isSquare(n) {
		return 1
	}
	for a := 1; a*a <= n; a++ {
		if isSquare(n - a*a) {
			return 2
		}
	}
	m := n
	for m%4 == 0 {
		m /= 4
	}
	if m%8 == 7 {
		return 4
	}
	return 3
}

func find(n, count int, out *[]int) bool {
	if count == 1 {
		if isSquare(n) {
			*out = append(*out, n)
			return true
		}
		return false
	}
	for a := int(math.Sqrt(float64(n))) + 1; a >= 1; a-- {
		if a*a > n {
			continue
		}
		if find(n-a*a, count-1, out) {
			*out = append(*out, a*a)
			return true
		}
	}
	return false
}

func demo(n int) {
	count := minSquaresCount(n)
	parts := []int{}
	find(n, count, &parts)
	sort.Sort(sort.Reverse(sort.IntSlice(parts)))
	strs := make([]string, len(parts))
	for i, p := range parts {
		strs[i] = fmt.Sprintf("%d", p)
	}
	fmt.Printf("%d (%s)\n", count, strings.Join(strs, " + "))
}

func main() {
	demo(4)
	demo(17)
	demo(18)
}
