// Word circle: backtracking to order all words so each last char == next first char,
// and the last wraps to the first. Time O(n!) worst, Space O(n). (n small)
package main

import (
	"fmt"
	"strings"
)

func lastByte(s string) byte { return s[len(s)-1] }

func circle(words []string) []int {
	n := len(words)
	used := make([]bool, n)
	order := []int{0}
	used[0] = true
	var bt func() bool
	bt = func() bool {
		if len(order) == n {
			return lastByte(words[order[len(order)-1]]) == words[order[0]][0]
		}
		need := lastByte(words[order[len(order)-1]])
		for i := 0; i < n; i++ {
			if !used[i] && words[i][0] == need {
				used[i] = true
				order = append(order, i)
				if bt() {
					return true
				}
				order = order[:len(order)-1]
				used[i] = false
			}
		}
		return false
	}
	if bt() {
		return order
	}
	return nil
}

func main() {
	words := []string{"chair", "height", "racket", "touch", "tunic"}
	order := circle(words)
	if order == nil {
		fmt.Println("Cannot form a circle")
		return
	}
	parts := make([]string, len(order))
	for i, idx := range order {
		parts[i] = words[idx]
	}
	fmt.Println(strings.Join(parts, " --> ") + " --> " + words[order[0]])
}
