// Day 1116 - Quack: push/pop left, pull right, using three stacks.
// Two stacks L (top=leftmost) and R (top=rightmost); rebalance by splitting the
// other in half via scratch stack T. Amortized O(1) per op, O(1) extra memory.
package main

import "fmt"

type Quack struct {
	L, R, T []int
}

func (q *Quack) Push(x int) { q.L = append(q.L, x) }

func pop(s *[]int) int {
	n := len(*s)
	v := (*s)[n-1]
	*s = (*s)[:n-1]
	return v
}

func (q *Quack) rebalanceToL() {
	m := len(q.R)
	rightCount := m / 2
	for i := 0; i < rightCount; i++ {
		q.T = append(q.T, pop(&q.R))
	}
	for len(q.R) > 0 {
		q.L = append(q.L, pop(&q.R))
	}
	for len(q.T) > 0 {
		q.R = append(q.R, pop(&q.T))
	}
}

func (q *Quack) rebalanceToR() {
	m := len(q.L)
	leftCount := m / 2
	for i := 0; i < leftCount; i++ {
		q.T = append(q.T, pop(&q.L))
	}
	for len(q.L) > 0 {
		q.R = append(q.R, pop(&q.L))
	}
	for len(q.T) > 0 {
		q.L = append(q.L, pop(&q.T))
	}
}

func (q *Quack) Pop() int {
	if len(q.L) == 0 {
		q.rebalanceToL()
	}
	return pop(&q.L)
}

func (q *Quack) Pull() int {
	if len(q.R) == 0 {
		q.rebalanceToR()
	}
	return pop(&q.R)
}

func main() {
	q := &Quack{}
	for _, x := range []int{1, 2, 3} {
		q.Push(x)
	}
	fmt.Println("pop:", q.Pop())   // 3
	fmt.Println("pull:", q.Pull()) // 1
	for _, x := range []int{4, 5} {
		q.Push(x)
	}
	fmt.Println("pull:", q.Pull()) // 2
	fmt.Println("pop:", q.Pop())   // 5
	fmt.Println("pull:", q.Pull()) // 4
}
