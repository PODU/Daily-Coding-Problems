// Scan parenthesis string, track open-paren depth, record maximum. Time O(n), Space O(1).
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
