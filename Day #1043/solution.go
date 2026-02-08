// PeekableIterator: wrap an iterator and cache one element ahead so peek() returns
// the next value without consuming it. O(1) per op, O(1) extra space.
package main

import "fmt"

type PeekableInterface struct {
	data      []int
	idx       int
	cached    int
	hasCached bool
}

func NewPeekable(data []int) *PeekableInterface {
	return &PeekableInterface{data: data}
}

func (p *PeekableInterface) HasNext() bool {
	return p.hasCached || p.idx < len(p.data)
}

func (p *PeekableInterface) Next() int {
	if p.hasCached {
		p.hasCached = false
		return p.cached
	}
	v := p.data[p.idx]
	p.idx++
	return v
}

func (p *PeekableInterface) Peek() int {
	if !p.hasCached {
		p.cached = p.data[p.idx]
		p.idx++
		p.hasCached = true
	}
	return p.cached
}

func main() {
	it := NewPeekable([]int{1, 2, 3})
	fmt.Printf("peek() -> %d\n", it.Peek())
	fmt.Printf("next() -> %d\n", it.Next())
	fmt.Printf("peek() -> %d\n", it.Peek())
	fmt.Printf("next() -> %d\n", it.Next())
	fmt.Printf("next() -> %d\n", it.Next())
	fmt.Printf("hasNext() -> %t\n", it.HasNext())
}
