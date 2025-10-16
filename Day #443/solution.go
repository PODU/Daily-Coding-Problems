// Day 443: FIFO queue from two stacks. Amortized O(1) per op: push onto `in`,
// pop from `out`, refilling `out` from `in` when empty.
package main

import "fmt"

type QueueTwoStacks struct {
	in  []int
	out []int
}

func (q *QueueTwoStacks) enqueue(x int) { q.in = append(q.in, x) }

func (q *QueueTwoStacks) dequeue() int {
	if len(q.out) == 0 {
		if len(q.in) == 0 {
			panic("queue is empty")
		}
		for len(q.in) > 0 {
			q.out = append(q.out, q.in[len(q.in)-1])
			q.in = q.in[:len(q.in)-1]
		}
	}
	v := q.out[len(q.out)-1]
	q.out = q.out[:len(q.out)-1]
	return v
}

func main() {
	q := &QueueTwoStacks{}
	q.enqueue(1)
	q.enqueue(2)
	q.enqueue(3)
	fmt.Println(q.dequeue()) // 1
	fmt.Println(q.dequeue()) // 2
	q.enqueue(4)
	fmt.Println(q.dequeue()) // 3
	fmt.Println(q.dequeue()) // 4
}
