// Day 265: Minimum bonuses. Two-pass scan (left-to-right then right-to-left),
// each worker gets max of the two passes. Time O(n), space O(n).
package main

import (
	"fmt"
	"strings"
)

func bonuses(lines []int) []int {
	n := len(lines)
	b := make([]int, n)
	for i := range b {
		b[i] = 1
	}
	for i := 1; i < n; i++ {
		if lines[i] > lines[i-1] {
			b[i] = b[i-1] + 1
		}
	}
	for i := n - 2; i >= 0; i-- {
		if lines[i] > lines[i+1] && b[i+1]+1 > b[i] {
			b[i] = b[i+1] + 1
		}
	}
	return b
}

func main() {
	lines := []int{10, 40, 200, 1000, 60, 30}
	b := bonuses(lines)
	parts := make([]string, len(b))
	for i, v := range b {
		parts[i] = fmt.Sprintf("%d", v)
	}
	fmt.Println("[" + strings.Join(parts, ", ") + "]")
}
