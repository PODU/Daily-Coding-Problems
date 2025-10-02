// Queue built from a slice of fixed-capacity blocks (cap 4). Track head/tail block+offset and an O(1) size.
// enqueue/dequeue/get_size all amortized O(1) time; O(n) space.
package main

import "fmt"

const cap4 = 4

type BlockQueue struct {
	blocks    [][]int
	headBlock int
	headOff   int
	tailOff   int
	sz        int
}

func (q *BlockQueue) Enqueue(v int) {
	if len(q.blocks) == 0 || q.tailOff == cap4 {
		q.blocks = append(q.blocks, make([]int, cap4))
		q.tailOff = 0
	}
	q.blocks[len(q.blocks)-1][q.tailOff] = v
	q.tailOff++
	q.sz++
}

func (q *BlockQueue) Dequeue() int {
	v := q.blocks[q.headBlock][q.headOff]
	q.headOff++
	q.sz--
	if q.headOff == cap4 { // front block fully consumed
		q.headBlock++
		q.headOff = 0
	}
	return v
}

func (q *BlockQueue) GetSize() int {
	return q.sz
}

func main() {
	q := &BlockQueue{}
	for _, v := range []int{1, 2, 3, 4, 5} {
		q.Enqueue(v)
	}
	fmt.Printf("size=%d\n", q.GetSize())
	fmt.Println(q.Dequeue())
	fmt.Println(q.Dequeue())
	fmt.Println(q.Dequeue())
	fmt.Printf("size=%d\n", q.GetSize())
}
