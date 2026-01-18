// Queue via fixed-length array blocks (capacity 4). Slice of blocks; enqueue to tail, dequeue from head.
// Amortized O(1) per op; O(n) space.
package main

import "fmt"

const cap4 = 4

type BlockQueue struct {
	blocks    [][]int
	headIdx   int
	tailCount int
	size      int
}

func (q *BlockQueue) enqueue(v int) {
	if len(q.blocks) == 0 || q.tailCount == cap4 {
		q.blocks = append(q.blocks, make([]int, cap4))
		q.tailCount = 0
	}
	q.blocks[len(q.blocks)-1][q.tailCount] = v
	q.tailCount++
	q.size++
}

func (q *BlockQueue) dequeue() int {
	if q.size == 0 {
		panic("empty")
	}
	v := q.blocks[0][q.headIdx]
	q.headIdx++
	q.size--
	if q.headIdx == cap4 || (len(q.blocks) == 1 && q.headIdx == q.tailCount) {
		q.blocks = q.blocks[1:]
		q.headIdx = 0
		if len(q.blocks) == 0 {
			q.tailCount = 0
		}
	}
	return v
}

func (q *BlockQueue) getSize() int { return q.size }

func main() {
	q := &BlockQueue{}
	for i := 1; i <= 10; i++ {
		q.enqueue(i)
	}
	fmt.Print("Dequeued:")
	for i := 0; i < 3; i++ {
		fmt.Printf(" %d", q.dequeue())
	}
	fmt.Println()
	fmt.Printf("Size: %d\n", q.getSize())
}
