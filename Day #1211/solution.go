// Day 1211: Interleave first half of a stack with the reversed second half, using one queue.
// Load stack bottom->top into the queue, then alternately push front/back (back via rotation). Time O(n^2), Space O(n).
package main

import "fmt"

type Stack struct{ d []int }

func (s *Stack) push(x int) { s.d = append(s.d, x) }
func (s *Stack) pop() int   { x := s.d[len(s.d)-1]; s.d = s.d[:len(s.d)-1]; return x }
func (s *Stack) empty() bool { return len(s.d) == 0 }

type Queue struct{ d []int }

func (q *Queue) enqueue(x int) { q.d = append(q.d, x) }
func (q *Queue) dequeue() int  { x := q.d[0]; q.d = q.d[1:]; return x }

func interleave(st *Stack) {
	q := &Queue{}
	n := 0
	for !st.empty() {
		q.enqueue(st.pop())
		n++
	}
	for i := 0; i < n; i++ {
		st.push(q.dequeue())
	}
	for i := 0; i < n; i++ {
		q.enqueue(st.pop())
	}
	remaining := n
	takeFront := true
	for remaining > 0 {
		if takeFront {
			st.push(q.dequeue())
		} else {
			for i := 0; i < remaining-1; i++ {
				q.enqueue(q.dequeue())
			}
			st.push(q.dequeue())
		}
		remaining--
		takeFront = !takeFront
	}
}

func main() {
	st := &Stack{}
	for _, x := range []int{1, 2, 3, 4, 5} {
		st.push(x)
	}
	interleave(st)
	fmt.Println(st.d) // [1 5 2 4 3]
}
