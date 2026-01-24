// Day 951: interleave first half of a stack with the reversed second half,
// in place using only one auxiliary queue (push/pop, enqueue/dequeue).
// Time O(n^2) due to rotations, Space O(1) extra besides the queue.
package main

import "fmt"

func interleave(input []int) []int {
	st := append([]int(nil), input...) // top = end
	q := []int{}                       // queue: append to back, pop from front
	for len(st) > 0 {
		q = append(q, st[len(st)-1])
		st = st[:len(st)-1]
	}
	for len(q) > 0 {
		st = append(st, q[0])
		q = q[1:]
	}
	for len(st) > 0 {
		q = append(q, st[len(st)-1])
		st = st[:len(st)-1]
	}
	// q = a0..a_{n-1}
	for len(q) > 0 {
		st = append(st, q[0])
		q = q[1:]
		m := len(q)
		if m == 0 {
			break
		}
		for i := 0; i < m-1; i++ {
			q = append(q, q[0])
			q = q[1:]
		}
		st = append(st, q[0])
		q = q[1:]
	}
	return st
}

func main() {
	fmt.Println(interleave([]int{1, 2, 3, 4, 5})) // [1 5 2 4 3]
	fmt.Println(interleave([]int{1, 2, 3, 4}))    // [1 4 2 3]
}
