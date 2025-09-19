// Day 301: Bloom filter - fixed-size probabilistic set. No false negatives.
// add/check O(k); space O(m) bits.
package main

import (
	"fmt"
	"hash/fnv"
)

type BloomFilter struct {
	bits []bool
	k    int
}

func NewBloom(m, k int) *BloomFilter { return &BloomFilter{make([]bool, m), k} }

func (b *BloomFilter) h(s string, i int) int {
	f := fnv.New64a()
	f.Write([]byte(s))
	h1 := f.Sum64()
	f.Reset()
	f.Write([]byte(s + "#salt"))
	h2 := f.Sum64()
	return int((h1 + uint64(i)*h2) % uint64(len(b.bits)))
}

func (b *BloomFilter) add(v string) {
	for i := 0; i < b.k; i++ {
		b.bits[b.h(v, i)] = true
	}
}

func (b *BloomFilter) check(v string) bool {
	for i := 0; i < b.k; i++ {
		if !b.bits[b.h(v, i)] {
			return false
		}
	}
	return true
}

func main() {
	bf := NewBloom(1000, 4)
	bf.add("apple")
	bf.add("banana")
	fmt.Println(bf.check("apple"))
	fmt.Println(bf.check("banana"))
	fmt.Println(bf.check("cherry"))
}
