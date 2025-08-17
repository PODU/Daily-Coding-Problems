// Space-efficient bit array: pack bits into uint64 words. set/get O(1), space O(size/64).
package main

import "fmt"

type BitArray struct {
	words []uint64
	n     int
}

func (b *BitArray) Init(size int) {
	b.n = size
	b.words = make([]uint64, (size+63)/64)
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
	var b BitArray
	b.Init(10)
	b.Set(2, 1)
	b.Set(7, 1)
	b.Set(7, 0)
	b.Set(9, 1)
	fmt.Printf("%d%d%d%d\n", b.Get(2), b.Get(7), b.Get(9), b.Get(0)) // 1010
}
