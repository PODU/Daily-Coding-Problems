// Wrap an iterator and buffer one element for peek(). next/hasNext/peek all O(1).
// Time O(1) per op; Space O(1).
package main

import "fmt"

type Iterator struct {
	data []int
	idx  int
}

func (it *Iterator) Next() int     { v := it.data[it.idx]; it.idx++; return v }
func (it *Iterator) HasNext() bool { return it.idx < len(it.data) }

type PeekableInterface struct {
	it       *Iterator
	buffer   int
	buffered bool
}

func NewPeekable(it *Iterator) *PeekableInterface {
	return &PeekableInterface{it: it}
}

func (p *PeekableInterface) Peek() int {
	if !p.buffered {
		p.buffer = p.it.Next()
		p.buffered = true
	}
	return p.buffer
}

func (p *PeekableInterface) Next() int {
	if p.buffered {
		p.buffered = false
		return p.buffer
	}
	return p.it.Next()
}

func (p *PeekableInterface) HasNext() bool {
	return p.buffered || p.it.HasNext()
}

func main() {
	p := NewPeekable(&Iterator{data: []int{1, 2, 3}})
	fmt.Printf("peek=%d next=%d peek=%d next=%d next=%d hasNext=%t\n",
		p.Peek(), p.Next(), p.Peek(), p.Next(), p.Next(), p.HasNext())
	// peek=1 next=1 peek=2 next=2 next=3 hasNext=false
}
