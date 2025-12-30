// Space-efficient bit array using 32-bit words; index>>5 picks word, 1<<(index&31) picks bit.
// Time: O(1) per op, Space: O(n/32 words).
package main

import "fmt"

type BitArray struct {
	words []uint32
}

func NewBitArray(size int) *BitArray {
	return &BitArray{words: make([]uint32, (size+31)>>5)}
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
	ba := NewBitArray(16)
	ba.Set(0, 1)
	ba.Set(5, 1)
	ba.Set(15, 1)
	fmt.Printf("get(0)=%d\n", ba.Get(0))
	fmt.Printf("get(1)=%d\n", ba.Get(1))
	fmt.Printf("get(5)=%d\n", ba.Get(5))
	fmt.Printf("get(15)=%d\n", ba.Get(15))
}
