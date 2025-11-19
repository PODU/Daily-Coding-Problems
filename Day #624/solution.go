// Minimum parentheses to remove to make string valid: single pass counting unmatched
// open/close. Answer = removals (unmatched ')') + leftover open. Time O(n), Space O(1).
package main

import "fmt"

func minRemove(s string) int {
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
	fmt.Println(minRemove("()())()")) // 1
	fmt.Println(minRemove(")("))       // 2
}
