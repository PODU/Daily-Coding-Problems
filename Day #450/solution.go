// Day 450: Balanced parentheses with '*' wildcards via greedy low/high open
// count. O(n) time, O(1) space.
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
		default: // '*'
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
	s := "(()*"
	if isBalanced(s) {
		fmt.Println("balanced")
	} else {
		fmt.Println("not balanced")
	}
	// balanced
}
