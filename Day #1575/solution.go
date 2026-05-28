// Cryptarithmetic solver via backtracking over distinct letters. O(10!/(10-k)!) worst, pruned.
package main

import (
	"fmt"
	"strings"
)

var (
	w1, w2, w3 = "SEND", "MORE", "MONEY"
	letters    []rune
	assign     = map[rune]int{}
	used       [10]bool
	leading    = map[rune]bool{}
)

func val(w string) int {
	v := 0
	for _, c := range w {
		v = v*10 + assign[c]
	}
	return v
}

func bt(idx int) bool {
	if idx == len(letters) {
		return val(w1)+val(w2) == val(w3)
	}
	c := letters[idx]
	for d := 0; d <= 9; d++ {
		if used[d] {
			continue
		}
		if d == 0 && leading[c] {
			continue
		}
		used[d] = true
		assign[c] = d
		if bt(idx + 1) {
			return true
		}
		used[d] = false
	}
	return false
}

func main() {
	for _, w := range []string{w1, w2, w3} {
		leading[[]rune(w)[0]] = true
	}
	seen := map[rune]bool{}
	for _, w := range []string{w1, w2, w3} {
		for _, c := range w {
			if !seen[c] {
				seen[c] = true
				letters = append(letters, c)
			}
		}
	}
	bt(0)
	parts := make([]string, len(letters))
	for i, c := range letters {
		parts[i] = fmt.Sprintf("'%c': %d", c, assign[c])
	}
	fmt.Println("{" + strings.Join(parts, ", ") + "}")
}
