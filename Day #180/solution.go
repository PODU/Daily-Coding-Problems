// Interleave first half with reversed second half using ONE auxiliary queue.
// Result (bottom->top) = arr[0],arr[n-1],arr[1],arr[n-2],...  O(n^2) time, O(n) space.
package main

import (
	"fmt"
	"strconv"
	"strings"
)

func interleave(arr []int) []int {
	stack := append([]int{}, arr...) // top = end
	q := []int{}
	pop := func() int { v := stack[len(stack)-1]; stack = stack[:len(stack)-1]; return v }
	deq := func() int { v := q[0]; q = q[1:]; return v }

	for len(stack) > 0 { // A: stack -> queue
		q = append(q, pop())
	}
	for len(q) > 0 { // B: queue -> stack (reverse)
		stack = append(stack, deq())
	}
	for len(stack) > 0 { // C: stack -> queue (front..back = bottom..top)
		q = append(q, pop())
	}

	takeFront := true
	for len(q) > 0 {
		if takeFront {
			stack = append(stack, deq())
		} else {
			for k := len(q) - 1; k > 0; k-- { // rotate back to front
				q = append(q, deq())
			}
			stack = append(stack, deq())
		}
		takeFront = !takeFront
	}
	return stack
}

func fmtSlice(a []int) string {
	s := make([]string, len(a))
	for i, x := range a {
		s[i] = strconv.Itoa(x)
	}
	return "[" + strings.Join(s, ", ") + "]"
}

func main() {
	fmt.Println(fmtSlice(interleave([]int{1, 2, 3, 4, 5})))
	fmt.Println(fmtSlice(interleave([]int{1, 2, 3, 4})))
}
