// Quack (push/pop left, pull right) via three stacks.
// On underflow of one side, rebalance by moving half from the other side using a temp stack.
// Amortized O(1) per operation, O(1) extra memory beyond the three stacks.
package main

import "fmt"

type Quack struct {
	front, back, temp []int // front top = leftmost, back top = rightmost
}

func pop(s *[]int) int { v := (*s)[len(*s)-1]; *s = (*s)[:len(*s)-1]; return v }

func (q *Quack) push(x int) { q.front = append(q.front, x) }

func (q *Quack) refillFront() {
	n := len(q.back)
	leftCount := (n + 1) / 2
	rightCount := n - leftCount
	for i := 0; i < rightCount; i++ {
		q.temp = append(q.temp, pop(&q.back))
	}
	for i := 0; i < leftCount; i++ {
		q.front = append(q.front, pop(&q.back))
	}
	for len(q.temp) > 0 {
		q.back = append(q.back, pop(&q.temp))
	}
}

func (q *Quack) refillBack() {
	n := len(q.front)
	rightCount := (n + 1) / 2
	leftCount := n - rightCount
	for i := 0; i < leftCount; i++ {
		q.temp = append(q.temp, pop(&q.front))
	}
	for i := 0; i < rightCount; i++ {
		q.back = append(q.back, pop(&q.front))
	}
	for len(q.temp) > 0 {
		q.front = append(q.front, pop(&q.temp))
	}
}

func (q *Quack) popLeft() int {
	if len(q.front) == 0 {
		q.refillFront()
	}
	if len(q.front) == 0 {
		panic("empty")
	}
	return pop(&q.front)
}

func (q *Quack) pull() int {
	if len(q.back) == 0 {
		q.refillBack()
	}
	if len(q.back) == 0 {
		panic("empty")
	}
	return pop(&q.back)
}

func main() {
	q := &Quack{}
	q.push(1)
	q.push(2)
	q.push(3)
	fmt.Println("pop:", q.popLeft()) // 3
	fmt.Println("pull:", q.pull())   // 1
	q.push(4)
	fmt.Println("pull:", q.pull())   // 2
	fmt.Println("pop:", q.popLeft()) // 4
}
