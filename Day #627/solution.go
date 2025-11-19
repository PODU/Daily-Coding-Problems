// Peekable iterator: cache one element ahead so peek() returns next without advancing.
// next/peek/hasNext all O(1).
package main

import "fmt"

type Peekable struct {
	data []int
	idx  int
}

func NewPeekable(data []int) *Peekable { return &Peekable{data: data} }

func (p *Peekable) HasNext() bool { return p.idx < len(p.data) }

func (p *Peekable) Peek() int { return p.data[p.idx] } // next without advancing

func (p *Peekable) Next() int {
	v := p.data[p.idx]
	p.idx++
	return v
}

func main() {
	it := NewPeekable([]int{1, 2, 3, 4})
	fmt.Println(it.Peek())    // 1
	fmt.Println(it.Next())    // 1
	fmt.Println(it.Next())    // 2
	fmt.Println(it.Peek())    // 3
	fmt.Println(it.Next())    // 3
	fmt.Println(it.HasNext()) // true
	fmt.Println(it.Next())    // 4
	fmt.Println(it.HasNext()) // false
}
