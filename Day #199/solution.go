// Day 199: Balance parentheses with minimum insertions/deletions.
// Greedy: each unmatched paren costs exactly 1 op; inserting its partner is always optimal.
// Time: O(n), Space: O(n).
package main

import (
	"fmt"
	"strings"
)

func balance(s string) string {
	var res strings.Builder
	open := 0
	for _, c := range s {
		if c == '(' {
			res.WriteByte('(')
			open++
		} else {
			if open > 0 {
				res.WriteByte(')')
				open--
			} else {
				res.WriteString("()") // unmatched ')': insert a '(' before it
			}
		}
	}
	res.WriteString(strings.Repeat(")", open)) // close remaining opens
	return res.String()
}

func main() {
	fmt.Println(balance("(()"))   // (())
	fmt.Println(balance("))()(")) // ()()()()
}
