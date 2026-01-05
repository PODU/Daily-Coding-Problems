// Day 857: Depth of tree from (lr) string representation.
// Approach: depth equals the maximum nesting level of parentheses.
// Time: O(n), Space: O(1).
package main

import "fmt"

func depth(s string) int {
	cur, mx := 0, 0
	for _, c := range s {
		if c == '(' {
			cur++
			if cur > mx {
				mx = cur
			}
		} else if c == ')' {
			cur--
		}
	}
	return mx
}

func main() {
	fmt.Println(depth("(00)"))          // 1
	fmt.Println(depth("((00)(00))"))    // 2
	fmt.Println(depth("((((00)0)0)0)")) // 4
}
