// Bloom filter: fixed bit array (1000 bits) + k=3 hashes via double hashing.
// add/check are O(k); space O(m) bits. check has false positives, no false negatives.
package main

import (
	"fmt"
	"hash/fnv"
)

const (
	bfSize = 1000
	bfK    = 3
)

type BloomFilter struct {
	bits [bfSize]bool
}

func (b *BloomFilter) baseHashes(v string) (uint64, uint64) {
	h1 := fnv.New64a()
	h1.Write([]byte(v))
	h2 := fnv.New64()
	h2.Write([]byte(v + "salt"))
	return h1.Sum64(), h2.Sum64()
}

func (b *BloomFilter) Add(v string) {
	h1, h2 := b.baseHashes(v)
	for i := uint64(0); i < bfK; i++ {
		b.bits[(h1+i*h2)%bfSize] = true
	}
}

func (b *BloomFilter) Check(v string) bool {
	h1, h2 := b.baseHashes(v)
	for i := uint64(0); i < bfK; i++ {
		if !b.bits[(h1+i*h2)%bfSize] {
			return false
		}
	}
	return true
}

func main() {
	bf := &BloomFilter{}
	added := []string{"apple", "banana", "cherry"}
	for _, s := range added {
		bf.Add(s)
	}

	fmt.Println("Added values (expect all true):")
	for _, s := range added {
		fmt.Printf("  check(%s) = %v\n", s, bf.Check(s))
	}

	fmt.Println("Non-added values (expect mostly false):")
	for _, s := range []string{"date", "elderberry", "fig", "grape"} {
		fmt.Printf("  check(%s) = %v\n", s, bf.Check(s))
	}
}
