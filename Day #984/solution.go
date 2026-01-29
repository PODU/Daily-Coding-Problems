// Balance a parentheses string with minimum insertions+deletions (insertion-only
// greedy is provably optimal: each unmatched paren needs exactly one edit).
// Time: O(n), Space: O(n).
package main

import (
	"fmt"
	"strings"
)

func balance(s string) string {
	var res strings.Builder
	bal := 0
	for _, c := range s {
		if c == '(' {
			res.WriteByte('(')
			bal++
		} else { // ')'
			if bal > 0 {
				res.WriteByte(')')
				bal--
			} else {
				res.WriteString("()") // insert '(' to match this ')'
			}
		}
	}
	res.WriteString(strings.Repeat(")", bal)) // close any still-open '('
	return res.String()
}

func main() {
	fmt.Println(balance("(()"))
	fmt.Println(balance("))()("))
}
