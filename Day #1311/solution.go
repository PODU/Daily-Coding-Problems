// Quack (deque) via three stacks. On underflow of one end, rebalance by moving
// half the elements through the third stack -> O(1) amortized, O(1) extra memory.
package main

import "fmt"

type Quack struct {
	L, R, T []int // left (top=leftmost), right (top=rightmost), temp
}

func pop(s *[]int) int {
	n := len(*s)
	v := (*s)[n-1]
	*s = (*s)[:n-1]
	return v
}

func rebalance(src, dst, tmp *[]int) {
	n := len(*src)
	k := n / 2 // elements that stay in src
	for i := 0; i < k; i++ {
		*tmp = append(*tmp, pop(src))
	}
	for i := 0; i < n-k; i++ {
		*dst = append(*dst, pop(src))
	}
	for i := 0; i < k; i++ {
		*src = append(*src, pop(tmp))
	}
}

func (q *Quack) Push(x int) { q.L = append(q.L, x) }
func (q *Quack) Pop() int {
	if len(q.L) == 0 {
		rebalance(&q.R, &q.L, &q.T)
	}
	return pop(&q.L)
}
func (q *Quack) Pull() int {
	if len(q.R) == 0 {
		rebalance(&q.L, &q.R, &q.T)
	}
	return pop(&q.R)
}

func main() {
	q := &Quack{}
	q.Push(1)
	q.Push(2)
	q.Push(3)
	fmt.Println(q.Pop())  // 3
	fmt.Println(q.Pull()) // 1
	q.Push(4)
	fmt.Println(q.Pull()) // 2
	fmt.Println(q.Pop())  // 4
}
