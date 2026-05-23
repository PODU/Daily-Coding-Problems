// Candy problem: init bonuses to 1, left-to-right then right-to-left passes enforce strict ordering.
// Time O(n), Space O(n).
package main

import (
	"fmt"
	"strconv"
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
	b := bonuses(a)
	parts := make([]string, len(b))
	for i, x := range b {
		parts[i] = strconv.Itoa(x)
	}
	fmt.Println("[" + strings.Join(parts, ", ") + "]")
}
