// Cryptarithmetic solver via backtracking over distinct letters with column-sum check.
// Time: O(10!) worst case over distinct letters (<=10), Space: O(#letters).
package main

import (
	"fmt"
	"strings"
)

var (
	w1, w2, w3 string
	letters    []rune
	leading    map[rune]bool
	assign     map[rune]int
	used       [10]bool
)

func value(w string) int {
	v := 0
	for _, c := range w {
		v = v*10 + assign[c]
	}
	return v
}

func solve(idx int) bool {
	if idx == len(letters) {
		return value(w1)+value(w2) == value(w3)
	}
	ch := letters[idx]
	for d := 0; d <= 9; d++ {
		if used[d] {
			continue
		}
		if d == 0 && leading[ch] {
			continue
		}
		used[d] = true
		assign[ch] = d
		if solve(idx + 1) {
			return true
		}
		used[d] = false
	}
	return false
}

func main() {
	w1, w2, w3 = "SEND", "MORE", "MONEY"
	leading = map[rune]bool{rune(w1[0]): true, rune(w2[0]): true, rune(w3[0]): true}
	assign = map[rune]int{}

	seen := []rune{}
	inSeen := map[rune]bool{}
	for _, c := range w1 + w2 + w3 {
		if !inSeen[c] {
			inSeen[c] = true
			seen = append(seen, c)
		}
	}
	letters = seen

	if solve(0) {
		parts := make([]string, 0, len(seen))
		for _, c := range seen {
			parts = append(parts, fmt.Sprintf("'%c': %d", c, assign[c]))
		}
		fmt.Printf("{%s}\n", strings.Join(parts, ", "))
	} else {
		fmt.Println("No solution")
	}
}
