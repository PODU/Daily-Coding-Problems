// Space-efficient bit array packing 32 bits per word.
// init(size), set(i,val), get(i): each O(1); space O(size/32) words.
package main

import "fmt"

type BitArray struct {
	words []uint32
	n     int
}

func (b *BitArray) Init(size int) {
	b.n = size
	b.words = make([]uint32, (size+31)/32)
}

func (b *BitArray) Set(i, val int) {
	if val != 0 {
		b.words[i>>5] |= 1 << uint(i&31)
	} else {
		b.words[i>>5] &^= 1 << uint(i&31)
	}
}

func (b *BitArray) Get(i int) int {
	return int((b.words[i>>5] >> uint(i&31)) & 1)
}

func main() {
	var b BitArray
	b.Init(10)
	b.Set(1, 1)
	b.Set(4, 1)
	b.Set(4, 0)
	b.Set(9, 1)
	fmt.Println(b.Get(1), b.Get(4), b.Get(9), b.Get(0))
	// expected: 1 0 1 0
}
