// Day 430: Balance parentheses with the minimum number of insertions + deletions.
// One pass: keep matched pairs; unmatched ')' -> "()", leftover '(' gets a ')'. Time O(n), Space O(n).
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
			open++
			res.WriteByte('(')
		} else { // ')'
			if open > 0 {
				open--
				res.WriteByte(')')
			} else {
				res.WriteString("()")
			}
		}
	}
	for ; open > 0; open-- {
		res.WriteByte(')')
	}
	return res.String()
}

func main() {
	fmt.Println(balance("(()"))
	fmt.Println(balance("))()("))
}
