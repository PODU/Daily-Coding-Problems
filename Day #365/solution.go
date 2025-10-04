// Day 365: "Quack" deque (push/pop left, pull right) from three stacks.
// l holds the left part (left end on top), r the right part (right end on top);
// tmp is a transient helper used only to re-split when one side empties.
// Rebalance moves k after k ops -> O(1) amortized, O(1) extra memory.
package main

import "fmt"

type Quack struct {
	l, r, tmp []int
}

func pop(s *[]int) int {
	v := (*s)[len(*s)-1]
	*s = (*s)[:len(*s)-1]
	return v
}

func (q *Quack) Push(x int) { q.l = append(q.l, x) }

func (q *Quack) rebalance(to, from *[]int, toCount int) {
	n := len(*from)
	for i := 0; i < n-toCount; i++ {
		q.tmp = append(q.tmp, pop(from))
	}
	for i := 0; i < toCount; i++ {
		*to = append(*to, pop(from))
	}
	for len(q.tmp) > 0 {
		*from = append(*from, pop(&q.tmp))
	}
}

func (q *Quack) Pop() int {
	if len(q.l) == 0 {
		q.rebalance(&q.l, &q.r, (len(q.r)+1)/2)
	}
	return pop(&q.l)
}

func (q *Quack) Pull() int {
	if len(q.r) == 0 {
		q.rebalance(&q.r, &q.l, (len(q.l)+1)/2)
	}
	return pop(&q.r)
}

func main() {
	q := &Quack{}
	q.Push(1); q.Push(2); q.Push(3)
	fmt.Println(q.Pop())  // 3
	fmt.Println(q.Pull()) // 1
	q.Push(4); q.Push(5)
	fmt.Println(q.Pull()) // 2
	fmt.Println(q.Pop())  // 5
	fmt.Println(q.Pop())  // 4
}
