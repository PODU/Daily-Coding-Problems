// Day 1212: Space-efficient bit array backed by 64-bit words.
// Pack bits into words; set/get use word index and bit offset. Time O(1) per op, Space O(size/64).
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
	if val != 0 {
		b.words[i>>6] |= 1 << uint(i&63)
	} else {
		b.words[i>>6] &^= 1 << uint(i&63)
	}
}

func (b *BitArray) Get(i int) int {
	return int((b.words[i>>6] >> uint(i&63)) & 1)
}

func main() {
	b := NewBitArray(10)
	b.Set(2, 1)
	b.Set(7, 1)
	b.Set(2, 0)
	fmt.Println(b.Get(2), b.Get(7), b.Get(0)) // 0 1 0
}
