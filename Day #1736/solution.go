// FIFO queue via two stacks (in/out); dequeue moves in->out when out empty. Amortized O(1) per op, O(n) space.
package main

import "fmt"

type MyQueue struct {
	in  []int
	out []int
}

func (q *MyQueue) Enqueue(x int) {
	q.in = append(q.in, x)
}

func (q *MyQueue) Dequeue() int {
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
	q := &MyQueue{}
	q.Enqueue(1)
	q.Enqueue(2)
	q.Enqueue(3)
	fmt.Println(q.Dequeue())
	q.Enqueue(4)
	fmt.Println(q.Dequeue())
	fmt.Println(q.Dequeue())
	fmt.Println(q.Dequeue())
}
