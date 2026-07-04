// Day 1761: Interleave first half of a stack with the reversed second half,
// in-place using ONE auxiliary queue (only push/pop/enqueue/dequeue).
// Dump stack into queue (reverses), rebuild taking alternately back/front.
// Time O(n^2), Space O(n). Stack modeled as slice (end = top).
package main

import (
	"fmt"
	"strings"
)

func interleave(input []int) []int {
	stack := append([]int(nil), input...) // end = top
	q := []int{}
	for len(stack) > 0 {
		q = append(q, stack[len(stack)-1]) // pop top -> enqueue
		stack = stack[:len(stack)-1]
	}
	takeBack := true
	for len(q) > 0 {
		var v int
		if takeBack {
			r := len(q) - 1
			for i := 0; i < r; i++ {
				q = append(q, q[0])
				q = q[1:]
			}
			v = q[0]
			q = q[1:]
		} else {
			v = q[0]
			q = q[1:]
		}
		stack = append(stack, v)
		takeBack = !takeBack
	}
	return stack
}

func fmtSlice(v []int) string {
	parts := make([]string, len(v))
	for i, x := range v {
		parts[i] = fmt.Sprintf("%d", x)
	}
	return "[" + strings.Join(parts, ", ") + "]"
}

func main() {
	fmt.Println(fmtSlice(interleave([]int{1, 2, 3, 4, 5})))
	fmt.Println(fmtSlice(interleave([]int{1, 2, 3, 4})))
}
