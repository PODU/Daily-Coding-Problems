// Cryptarithmetic solver: backtracking over distinct letters with leading-zero pruning.
// Time: O(10!/(10-k)!) over k<=10 distinct letters; Space: O(k).
package main

import (
	"fmt"
	"strings"
)

var w1, w2, w3 = "SEND", "MORE", "MONEY"
var order []byte
var assign map[byte]int
var used [10]bool
var leading map[byte]bool

func num(w string) int64 {
	var n int64 = 0
	for i := 0; i < len(w); i++ {
		n = n*10 + int64(assign[w[i]])
	}
	return n
}

func dfs(idx int) bool {
	if idx == len(order) {
		return num(w1)+num(w2) == num(w3)
	}
	for d := 0; d < 10; d++ {
		if used[d] {
			continue
		}
		if d == 0 && leading[order[idx]] {
			continue
		}
		used[d] = true
		assign[order[idx]] = d
		if dfs(idx + 1) {
			return true
		}
		used[d] = false
		delete(assign, order[idx])
	}
	return false
}

func main() {
	seen := map[byte]bool{}
	for _, w := range []string{w1, w2, w3} {
		for i := 0; i < len(w); i++ {
			if !seen[w[i]] {
				seen[w[i]] = true
				order = append(order, w[i])
			}
		}
	}
	leading = map[byte]bool{w1[0]: true, w2[0]: true, w3[0]: true}
	assign = map[byte]int{}

	dfs(0)

	var parts []string
	for _, c := range order {
		parts = append(parts, fmt.Sprintf("'%c': %d", c, assign[c]))
	}
	fmt.Println("{" + strings.Join(parts, ", ") + "}")
}
