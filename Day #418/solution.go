// Day 418: Two-pass greedy. Each gets >= 1; more than any lower neighbor. Like candy distribution.
// Time O(n), Space O(n).
package main

import (
	"fmt"
	"strings"
)

func bonuses(lines []int) []int {
	n := len(lines)
	res := make([]int, n)
	for i := range res {
		res[i] = 1
	}
	for i := 1; i < n; i++ {
		if lines[i] > lines[i-1] {
			res[i] = res[i-1] + 1
		}
	}
	for i := n - 2; i >= 0; i-- {
		if lines[i] > lines[i+1] && res[i] < res[i+1]+1 {
			res[i] = res[i+1] + 1
		}
	}
	return res
}

func main() {
	res := bonuses([]int{10, 40, 200, 1000, 60, 30})
	parts := make([]string, len(res))
	for i, v := range res {
		parts[i] = fmt.Sprintf("%d", v)
	}
	fmt.Println("[" + strings.Join(parts, ", ") + "]")
}
