// Greedy: track low/high possible open-paren counts in one pass.
// Time O(n), Space O(1). Balanced iff low reaches 0 at end and high never < 0.
package main

import "fmt"

func isBalanced(s string) bool {
	low, high := 0, 0
	for _, c := range s {
		switch c {
		case '(':
			low++
			high++
		case ')':
			low--
			high--
		default:
			low--
			high++
		}
		if high < 0 {
			return false
		}
		if low < 0 {
			low = 0
		}
	}
	return low == 0
}

func main() {
	a, b, c := "(()*", "(*)", ")*("
	ra, rb, rc := isBalanced(a), isBalanced(b), isBalanced(c)
	first := "not balanced"
	if ra && rb {
		first = "balanced"
	}
	second := "not balanced"
	if rc {
		second = "balanced"
	}
	fmt.Printf("%s and %s are %s. %s is %s.\n", a, b, first, c, second)
}
