// Day 1035: Smallest bonuses so each employee beats any lower-output neighbor.
// Two-pass greedy: left-to-right then right-to-left taking max. Time O(n), Space O(n).
package main

import (
	"fmt"
	"strings"
)

func bonuses(a []int) []int {
	n := len(a)
	b := make([]int, n)
	for i := range b {
		b[i] = 1
	}
	for i := 1; i < n; i++ {
		if a[i] > a[i-1] {
			b[i] = b[i-1] + 1
		}
	}
	for i := n - 2; i >= 0; i-- {
		if a[i] > a[i+1] && b[i] < b[i+1]+1 {
			b[i] = b[i+1] + 1
		}
	}
	return b
}

func main() {
	a := []int{10, 40, 200, 1000, 60, 30}
	parts := make([]string, len(a))
	for i, v := range bonuses(a) {
		parts[i] = fmt.Sprintf("%d", v)
	}
	fmt.Println("[" + strings.Join(parts, ", ") + "]")
}
