// Tree depth = max nesting level of '(' in the representation. O(n) time, O(1) space.
package main

import "fmt"

func treeDepth(s string) int {
	depth, best := 0, 0
	for _, c := range s {
		if c == '(' {
			depth++
			if depth > best {
				best = depth
			}
		} else if c == ')' {
			depth--
		}
	}
	return best
}

func main() {
	fmt.Println(treeDepth("((((00)0)0)0)")) // 4
}
