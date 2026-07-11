// Peekable iterator wrapper: cache one element ahead. peek()/next()/hasNext() O(1) time, O(1) space.
package main

import "fmt"

type Peekable struct {
	data []int
	idx  int
}

func NewPeekable(v []int) *Peekable { return &Peekable{data: v} }
func (p *Peekable) HasNext() bool   { return p.idx < len(p.data) }
func (p *Peekable) Peek() int       { return p.data[p.idx] } // does not advance
func (p *Peekable) Next() int       { v := p.data[p.idx]; p.idx++; return v }

func main() {
	p := NewPeekable([]int{1, 2, 3})
	fmt.Println("peek()    ->", p.Peek())
	fmt.Println("next()    ->", p.Next())
	fmt.Println("peek()    ->", p.Peek())
	fmt.Println("hasNext() ->", p.HasNext())
	fmt.Println("next()    ->", p.Next())
	fmt.Println("next()    ->", p.Next())
	fmt.Println("hasNext() ->", p.HasNext())
}
