// Day 937: Valid parenthesis string with '*' (= '(' , ')' or empty).
// Greedy: track [lo,hi] range of possible open counts. Valid iff lo can reach 0. O(n) time, O(1) space.
package main

import (
	"fmt"
	"strings"
)

func isValid(s string) bool {
	lo, hi := 0, 0
	for _, c := range s {
		switch c {
		case '(':
			lo++
			hi++
		case ')':
			lo--
			hi--
		default: // '*'
			lo--
			hi++
		}
		if hi < 0 {
			return false
		}
		if lo < 0 {
			lo = 0
		}
	}
	return lo == 0
}

func main() {
	inputs := []string{"(()*", "(*)", ")*("}
	var bal, notbal []string
	for _, s := range inputs {
		if isValid(s) {
			bal = append(bal, s)
		} else {
			notbal = append(notbal, s)
		}
	}
	fmt.Printf("%s are balanced. %s is not balanced.\n",
		strings.Join(bal, " and "), strings.Join(notbal, " and "))
	// (()* and (*) are balanced. )*( is not balanced.
}
