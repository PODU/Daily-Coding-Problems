// FIFO queue via two stacks (in/out); amortized O(1) per op, O(n) space.
package main

import "fmt"

type MyQueue struct {
	in  []int
	out []int
}

func (q *MyQueue) enqueue(x int) { q.in = append(q.in, x) }

func (q *MyQueue) dequeue() int {
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
	q.enqueue(1)
	q.enqueue(2)
	q.enqueue(3)
	fmt.Println(q.dequeue()) // 1
	q.enqueue(4)
	fmt.Println(q.dequeue()) // 2
	fmt.Println(q.dequeue()) // 3
	fmt.Println(q.dequeue()) // 4
}
