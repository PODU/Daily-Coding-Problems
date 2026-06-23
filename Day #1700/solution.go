// Deque ("quack") from three stacks: L (top=leftmost), R (top=rightmost), T temp.
// On empty side, move half the other stack across (T as transient buffer) => O(1) amortized.
package main

import "fmt"

type Quack struct {
	L, R, T []int // T only used transiently during rebalance
}

func (q *Quack) push(x int) { q.L = append(q.L, x) } // add to LEFT end

func pop(s *[]int) int {
	n := len(*s)
	v := (*s)[n-1]
	*s = (*s)[:n-1]
	return v
}

func (q *Quack) Pop() int { // remove LEFT end
	if len(q.L) == 0 {
		q.rebalanceLfromR()
	}
	return pop(&q.L)
}

func (q *Quack) Pull() int { // remove RIGHT end
	if len(q.R) == 0 {
		q.rebalanceRfromL()
	}
	return pop(&q.R)
}

func (q *Quack) rebalanceLfromR() {
	size := len(q.R)
	half := size / 2
	if half == 0 {
		half = 1
	}
	keep := size - half
	for k := 0; k < keep; k++ {
		q.T = append(q.T, pop(&q.R))
	}
	for len(q.R) > 0 {
		q.L = append(q.L, pop(&q.R))
	}
	for len(q.T) > 0 {
		q.R = append(q.R, pop(&q.T))
	}
}

func (q *Quack) rebalanceRfromL() {
	size := len(q.L)
	half := size / 2
	if half == 0 {
		half = 1
	}
	keep := size - half
	for k := 0; k < keep; k++ {
		q.T = append(q.T, pop(&q.L))
	}
	for len(q.L) > 0 {
		q.R = append(q.R, pop(&q.L))
	}
	for len(q.T) > 0 {
		q.L = append(q.L, pop(&q.T))
	}
}

func main() {
	q := &Quack{}
	q.push(1)
	q.push(2)
	q.push(3) // left-to-right: 3,2,1
	fmt.Println(q.Pop())  // 3
	fmt.Println(q.Pull()) // 1
	q.push(4)             // now 4,2
	fmt.Println(q.Pop())  // 4
	fmt.Println(q.Pull()) // 2
}
