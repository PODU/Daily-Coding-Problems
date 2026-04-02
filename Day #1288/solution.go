// Day 1288: Balance parentheses with minimum insertions+deletions (insertion-only is minimal).
// Single scan: pair each ')' with an open, else insert '(' before it; close leftover opens. O(n).
package main

import (
	"fmt"
	"strings"
)

func balance(s string) string {
	var res strings.Builder
	open := 0
	for _, ch := range s {
		if ch == '(' {
			res.WriteByte('(')
			open++
		} else {
			if open > 0 {
				res.WriteByte(')')
				open--
			} else {
				res.WriteString("()") // insert matching '(' before unmatched ')'
			}
		}
	}
	res.WriteString(strings.Repeat(")", open))
	return res.String()
}

func main() {
	fmt.Println(balance("(()"))   // (())
	fmt.Println(balance("))()(")) // ()()()()
}
