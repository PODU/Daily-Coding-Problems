// Balanced parens with '*' wildcard: greedy track [lo,hi] of possible open counts.
// Time O(n), Space O(1).
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
		default:
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
	tests := []string{"(()*", "(*)", ")*("}
	var bal, notBal []string
	for _, s := range tests {
		if isValid(s) {
			bal = append(bal, s)
		} else {
			notBal = append(notBal, s)
		}
	}
	fmt.Println(strings.Join(bal, " and ") + " are balanced. " + strings.Join(notBal, " and ") + " is not balanced.")
}
