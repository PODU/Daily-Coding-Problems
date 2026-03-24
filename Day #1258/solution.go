// Peeking iterator: buffer one element ahead. peek/next/hasNext all O(1) time, O(1) extra space.
package main

import "fmt"

type Iterator struct {
	data []int
	idx  int
}

func (it *Iterator) Next() int  { v := it.data[it.idx]; it.idx++; return v }
func (it *Iterator) HasNext() bool { return it.idx < len(it.data) }

type PeekableInterface struct {
	it        *Iterator
	buffer    int
	hasBuffer bool
}

func (p *PeekableInterface) Peek() int {
	if !p.hasBuffer {
		p.buffer = p.it.Next()
		p.hasBuffer = true
	}
	return p.buffer
}

func (p *PeekableInterface) Next() int {
	if p.hasBuffer {
		p.hasBuffer = false
		return p.buffer
	}
	return p.it.Next()
}

func (p *PeekableInterface) HasNext() bool {
	return p.hasBuffer || p.it.HasNext()
}

func main() {
	p := &PeekableInterface{it: &Iterator{data: []int{1, 2, 3}}}
	fmt.Println(p.Peek())
	fmt.Println(p.Next())
	fmt.Println(p.Next())
	fmt.Println(p.Peek())
	fmt.Println(p.HasNext())
	fmt.Println(p.Next())
	fmt.Println(p.HasNext())
}
