// Unrolled/paged queue: list of fixed-length blocks; head/tail indices. Amortized O(1) per op.
package main

import (
	"fmt"
	"strings"
)

const BS = 4

type BlockQueue struct {
	blocks [][]int
	head   int
	tail   int
	sz     int
}

func (q *BlockQueue) Enqueue(x int) {
	if len(q.blocks) == 0 || q.tail == BS {
		q.blocks = append(q.blocks, make([]int, BS))
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
	if q.head == BS || (len(q.blocks) == 1 && q.head == q.tail) {
		q.blocks = q.blocks[1:]
		q.head = 0
		if len(q.blocks) == 0 {
			q.tail = 0
		}
	}
	return x
}

func (q *BlockQueue) GetSize() int    { return q.sz }
func (q *BlockQueue) NumBlocks() int  { return len(q.blocks) }

func main() {
	q := &BlockQueue{}
	for i := 1; i <= 10; i++ {
		q.Enqueue(i)
	}
	fmt.Println("size after enqueue 1..10:", q.GetSize())
	fmt.Println("blocks allocated:", q.NumBlocks())
	fmt.Println("dequeue 3:", q.Dequeue(), q.Dequeue(), q.Dequeue())
	fmt.Println("size:", q.GetSize())
	q.Enqueue(11)
	fmt.Println("enqueue 11, size:", q.GetSize())
	var rest []string
	for q.GetSize() > 0 {
		rest = append(rest, fmt.Sprintf("%d", q.Dequeue()))
	}
	fmt.Println("dequeue rest:", strings.Join(rest, " "))
	fmt.Println("size:", q.GetSize())
}
