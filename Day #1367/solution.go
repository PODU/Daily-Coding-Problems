// Cryptarithmetic solver via backtracking over letter->digit assignments.
// Time O(10!/(10-L)!) worst with pruning, Space O(L). L = #distinct letters.
package main

import (
	"fmt"
	"strings"
)

var (
	wa, wb, wc string
	letters    []byte
	leading    = map[byte]bool{}
	assign     = map[byte]int{}
	used       [10]bool
)

func val(w string) int64 {
	var v int64
	for i := 0; i < len(w); i++ {
		v = v*10 + int64(assign[w[i]])
	}
	return v
}

func bt(i int) bool {
	if i == len(letters) {
		return val(wa)+val(wb) == val(wc)
	}
	ch := letters[i]
	for d := 0; d < 10; d++ {
		if used[d] || (d == 0 && leading[ch]) {
			continue
		}
		used[d] = true
		assign[ch] = d
		if bt(i + 1) {
			return true
		}
		used[d] = false
	}
	return false
}

func main() {
	wa, wb, wc = "SEND", "MORE", "MONEY"
	seen := map[byte]bool{}
	for _, w := range []string{wa, wb, wc} {
		leading[w[0]] = true
		for i := 0; i < len(w); i++ {
			if !seen[w[i]] {
				seen[w[i]] = true
				letters = append(letters, w[i])
			}
		}
	}
	if bt(0) {
		var parts []string
		for _, ch := range letters {
			parts = append(parts, fmt.Sprintf("'%c': %d", ch, assign[ch]))
		}
		fmt.Printf("{%s}\n", strings.Join(parts, ", "))
	} else {
		fmt.Println("No solution")
	}
}
