// Min parens to remove for validity: single pass counting unmatched.
// Time O(n), Space O(1).
package main

import "fmt"

func minRemovals(s string) int {
	open, removals := 0, 0
	for _, c := range s {
		if c == '(' {
			open++
		} else if c == ')' {
			if open > 0 {
				open--
			} else {
				removals++
			}
		}
	}
	return removals + open
}

func main() {
	fmt.Println(minRemovals("()())()"))
	fmt.Println(minRemovals(")("))
}
