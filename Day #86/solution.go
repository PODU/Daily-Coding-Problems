// Day 86: Min parentheses to remove for validity. Track unmatched '(' and ')'.
// Time O(n), Space O(1).
package main

import "fmt"

func minRemoval(s string) int {
	open, removals := 0, 0
	for _, c := range s {
		if c == '(' {
			open++
		} else if c == ')' {
			if open > 0 {
				open--
			} else {
				removals++ // unmatched ')'
			}
		}
	}
	return removals + open // leftover unmatched '('
}

func main() {
	fmt.Println(minRemoval("()())()")) // 1
	fmt.Println(minRemoval(")("))       // 2
}
