// Day 574: Space-efficient bit array backed by 64-bit words.
// set/get are O(1); storage is ceil(size/64) words.
package main

import "fmt"

type BitArray struct {
	words []uint64
	n     int
}

func NewBitArray(size int) *BitArray {
	return &BitArray{words: make([]uint64, (size+63)/64), n: size}
}

func (b *BitArray) Set(i, val int) {
	if i < 0 || i >= b.n {
		panic("index out of range")
	}
	if val != 0 {
		b.words[i>>6] |= 1 << uint(i&63)
	} else {
		b.words[i>>6] &^= 1 << uint(i&63)
	}
}

func (b *BitArray) Get(i int) int {
	if i < 0 || i >= b.n {
		panic("index out of range")
	}
	return int((b.words[i>>6] >> uint(i&63)) & 1)
}

func main() {
	b := NewBitArray(8)
	b.Set(0, 1)
	b.Set(3, 1)
	fmt.Println("get(0) =", b.Get(0)) // 1
	fmt.Println("get(1) =", b.Get(1)) // 0
	fmt.Println("get(3) =", b.Get(3)) // 1
	b.Set(3, 0)
	fmt.Println("get(3) =", b.Get(3)) // 0
}
