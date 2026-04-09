// Day 1327: Queue backed by a slice of fixed-length blocks (chunks).
// enqueue appends to the tail block (new block when full); dequeue pops from the head block. Amortized O(1).
package main

import "fmt"

const block = 4

type BlockQueue struct {
	blocks [][]int
	head   int // read index in the front block
	size   int
}

func (q *BlockQueue) enqueue(x int) {
	if len(q.blocks) == 0 || len(q.blocks[len(q.blocks)-1]) == block {
		q.blocks = append(q.blocks, make([]int, 0, block))
	}
	last := len(q.blocks) - 1
	q.blocks[last] = append(q.blocks[last], x)
	q.size++
}

func (q *BlockQueue) dequeue() int {
	if q.size == 0 {
		panic("empty")
	}
	x := q.blocks[0][q.head]
	q.head++
	q.size--
	if q.head == len(q.blocks[0]) {
		q.blocks = q.blocks[1:]
		q.head = 0
	}
	return x
}

func (q *BlockQueue) getSize() int { return q.size }

func main() {
	q := &BlockQueue{}
	for i := 1; i <= 5; i++ {
		q.enqueue(i)
	}
	fmt.Println(q.dequeue()) // 1
	fmt.Println(q.dequeue()) // 2
	fmt.Println(q.getSize()) // 3
}
