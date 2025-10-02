// Tree depth from nested-paren string: scan, track paren nesting, return max depth. O(N) time, O(1) space.
package main

import "fmt"

func treeDepth(s string) int {
	depth, maxDepth := 0, 0
	for _, c := range s {
		if c == '(' {
			depth++
			if depth > maxDepth {
				maxDepth = depth
			}
		} else if c == ')' {
			depth--
		}
	}
	return maxDepth
}

func main() {
	fmt.Println(treeDepth("((((00)0)0)0)"))
}
