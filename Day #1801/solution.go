// Balance parentheses with min insertions/deletions via insertion-based scan.
// Time O(n), Space O(n).
package main

import (
	"fmt"
	"strings"
)

func balance(s string) string {
	var result strings.Builder
	open := 0
	for _, c := range s {
		if c == '(' {
			result.WriteByte('(')
			open++
		} else { // ')'
			if open == 0 {
				result.WriteByte('(')
				result.WriteByte(')')
			} else {
				result.WriteByte(')')
				open--
			}
		}
	}
	result.WriteString(strings.Repeat(")", open))
	return result.String()
}

func main() {
	fmt.Println(balance("(()"))
	fmt.Println(balance("))()("))
}
