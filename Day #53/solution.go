// Day 53: FIFO queue from two stacks. Amortized O(1) per op.
// in-stack receives pushes; out-stack serves pops (refilled when empty).
package main

import "fmt"

type Queue struct {
	in, out []int
}

func (q *Queue) enqueue(x int) { q.in = append(q.in, x) }

func (q *Queue) dequeue() int {
	if len(q.out) == 0 {
		for len(q.in) > 0 {
			n := len(q.in) - 1
			q.out = append(q.out, q.in[n])
			q.in = q.in[:n]
		}
	}
	n := len(q.out) - 1
	v := q.out[n]
	q.out = q.out[:n]
	return v
}

func main() {
	q := &Queue{}
	q.enqueue(1)
	q.enqueue(2)
	q.enqueue(3)
	fmt.Println(q.dequeue()) // 1
	fmt.Println(q.dequeue()) // 2
	q.enqueue(4)
	fmt.Println(q.dequeue()) // 3
	fmt.Println(q.dequeue()) // 4
}
