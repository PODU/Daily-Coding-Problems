// Day 488: Queue backed by a set of fixed-length arrays (blocks).
// Blocks of size B chained; head/tail indices roll over to next block.
// enqueue/dequeue amortized O(1), get_size O(1). Space O(n).
package main

import (
	"fmt"
	"strings"
)

const blockSize = 4 // fixed block length

type BlockQueue struct {
	blocks     [][blockSize]int // set of fixed-length arrays
	head, tail int
	sz         int
}

func (q *BlockQueue) Enqueue(x int) {
	if len(q.blocks) == 0 || q.tail == blockSize { // allocate a new fixed block
		q.blocks = append(q.blocks, [blockSize]int{})
		q.tail = 0
	}
	q.blocks[len(q.blocks)-1][q.tail] = x
	q.tail++
	q.sz++
}

func (q *BlockQueue) Dequeue() int {
	if q.sz == 0 {
		panic("empty")
	}
	x := q.blocks[0][q.head]
	q.head++
	q.sz--
	if q.head == blockSize { // front block exhausted -> free it
		q.blocks = q.blocks[1:]
		q.head = 0
	}
	if len(q.blocks) == 1 && q.head == q.tail { // single block consumed
		q.blocks = nil
		q.head, q.tail = 0, 0
	}
	return x
}

func (q *BlockQueue) GetSize() int { return q.sz }

func main() {
	q := &BlockQueue{}
	for i := 1; i <= 6; i++ {
		q.Enqueue(i) // 1..6
	}
	fmt.Printf("size=%d\n", q.GetSize()) // 6
	fmt.Printf("deq=%d\n", q.Dequeue())  // 1
	fmt.Printf("deq=%d\n", q.Dequeue())  // 2
	q.Enqueue(7)
	q.Enqueue(8)
	fmt.Printf("size=%d\n", q.GetSize()) // 6
	var out []string
	for q.GetSize() > 0 {
		out = append(out, fmt.Sprintf("%d", q.Dequeue())) // 3 4 5 6 7 8
	}
	fmt.Println(strings.Join(out, " "))
}
