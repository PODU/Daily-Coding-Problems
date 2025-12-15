// Bloom filter: fixed bit array, k hash functions via double hashing.
// check() may report false positives but never false negatives.
// Time: O(k) per add/check, Space: O(m bits).
package main

import "fmt"

type BloomFilter struct {
	bits []bool
	m, k int
}

func NewBloom(size, numHashes int) *BloomFilter {
	return &BloomFilter{bits: make([]bool, size), m: size, k: numHashes}
}

func (b *BloomFilter) hashes(s string) (uint64, uint64) {
	var h1 uint64 = 1469598103934665603 // FNV-1a
	for i := 0; i < len(s); i++ {
		h1 ^= uint64(s[i])
		h1 *= 1099511628211
	}
	var h2 uint64 = 5381 // djb2
	for i := 0; i < len(s); i++ {
		h2 = ((h2 << 5) + h2) + uint64(s[i])
	}
	return h1, h2
}

func (b *BloomFilter) Add(s string) {
	h1, h2 := b.hashes(s)
	for i := 0; i < b.k; i++ {
		b.bits[(h1+uint64(i)*h2)%uint64(b.m)] = true
	}
}

func (b *BloomFilter) Check(s string) bool {
	h1, h2 := b.hashes(s)
	for i := 0; i < b.k; i++ {
		if !b.bits[(h1+uint64(i)*h2)%uint64(b.m)] {
			return false
		}
	}
	return true
}

func main() {
	bf := NewBloom(1000, 4)
	bf.Add("apple")
	bf.Add("banana")
	fmt.Println("apple:", bf.Check("apple"))   // true
	fmt.Println("banana:", bf.Check("banana")) // true
	fmt.Println("cherry:", bf.Check("cherry")) // false
}
