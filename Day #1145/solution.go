// Day 1145: Bloom filter - fixed-size bit array, k hashes.
// add/check O(k); check has false positives but never false negatives.
package main

import "fmt"

type BloomFilter struct {
	bits []bool
	m, k int
}

func NewBloomFilter(m, k int) *BloomFilter {
	return &BloomFilter{bits: make([]bool, m), m: m, k: k}
}

func (b *BloomFilter) hashI(s string, i int) int {
	h := uint64(1469598103934665603) ^ uint64(i+1)
	for _, c := range []byte(s) {
		h ^= uint64(c)
		h *= 1099511628211
	}
	return int(h % uint64(b.m))
}

func (b *BloomFilter) Add(v string) {
	for i := 0; i < b.k; i++ {
		b.bits[b.hashI(v, i)] = true
	}
}

func (b *BloomFilter) Check(v string) bool {
	for i := 0; i < b.k; i++ {
		if !b.bits[b.hashI(v, i)] {
			return false
		}
	}
	return true
}

func main() {
	bf := NewBloomFilter(1000, 4)
	bf.Add("apple")
	bf.Add("banana")
	fmt.Println(bf.Check("apple"), bf.Check("banana"), bf.Check("cherry"))
	// true true false (likely)
}
