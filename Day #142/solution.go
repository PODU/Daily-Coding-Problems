// Balanced parens with '*' wildcard: track range [lo,hi] of possible open counts. O(n) time, O(1) space.
package main

import "fmt"

func isBalanced(s string) bool {
	lo, hi := 0, 0
	for _, c := range s {
		switch c {
		case '(':
			lo++
			hi++
		case ')':
			lo--
			hi--
		default: // '*' as ')', '(' or empty
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
	a, b, c := "(()*", "(*)", ")*("
	sa, sb, sc := "", "", ""
	if isBalanced(a) {
		sa = "(()*"
	}
	if isBalanced(b) {
		sb = "(*)"
	}
	if !isBalanced(c) {
		sc = ")*("
	}
	fmt.Printf("%s and %s are balanced. %s is not balanced.\n", sa, sb, sc)
}
