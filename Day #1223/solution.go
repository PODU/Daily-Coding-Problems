// Single linear scan; depth = max paren nesting ('(' +1, ')' -1).
// O(n) time, O(1) space.
package main

import "fmt"

func treeDepth(s string) int {
	depth, cur := 0, 0
	for _, c := range s {
		if c == '(' {
			cur++
			if cur > depth {
				depth = cur
			}
		} else if c == ')' {
			cur--
		}
	}
	return depth
}

func main() {
	fmt.Println(treeDepth("((((00)0)0)0)"))
}
