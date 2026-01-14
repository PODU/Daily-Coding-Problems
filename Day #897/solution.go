// Queue via two stacks: enqueue->inStack; dequeue moves all to outStack when empty.
// Amortized O(1) per op, Space O(n).
package main

import "fmt"

type Queue struct {
	inStack  []int
	outStack []int
}

func (q *Queue) enqueue(x int) {
	q.inStack = append(q.inStack, x)
}

func (q *Queue) dequeue() int {
	if len(q.outStack) == 0 {
		for len(q.inStack) > 0 {
			n := len(q.inStack) - 1
			q.outStack = append(q.outStack, q.inStack[n])
			q.inStack = q.inStack[:n]
		}
	}
	n := len(q.outStack) - 1
	v := q.outStack[n]
	q.outStack = q.outStack[:n]
	return v
}

func main() {
	q := &Queue{}
	q.enqueue(1)
	q.enqueue(2)
	fmt.Println(q.dequeue())
	q.enqueue(3)
	fmt.Println(q.dequeue())
	fmt.Println(q.dequeue())
}
