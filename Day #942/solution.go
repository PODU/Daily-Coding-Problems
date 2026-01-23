// Day 942: Min parentheses to remove to make the string valid.
// One pass: count unmatched ')' immediately + leftover unmatched '('. O(n) time, O(1) space.
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
	fmt.Println(minRemovals("()())()")) // 1
	fmt.Println(minRemovals(")("))        // 2
}
