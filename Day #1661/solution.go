// Bloom filter: fixed 1000-bit array, 3 hashes via double hashing. add/check.
// add O(k), check O(k); Space O(m bits). check may false-positive, never false-negative.
package main

import "fmt"

const M = 1000
const K = 3

type Bloom struct{ bits [M]bool }

func h1(s string) uint64 {
	var h uint64 = 5381
	for _, c := range s {
		h = h*33 + uint64(c)
	}
	return h
}
func h2(s string) uint64 {
	var h uint64 = 0
	for _, c := range s {
		h = h*131 + uint64(c)
	}
	return h
}
func (bf *Bloom) add(s string) {
	a, b := h1(s), h2(s)
	for i := uint64(0); i < K; i++ {
		bf.bits[(a+i*b)%M] = true
	}
}
func (bf *Bloom) check(s string) bool {
	a, b := h1(s), h2(s)
	for i := uint64(0); i < K; i++ {
		if !bf.bits[(a+i*b)%M] {
			return false
		}
	}
	return true
}
func main() {
	bf := &Bloom{}
	for _, w := range []string{"apple", "banana", "cat"} {
		bf.add(w)
	}
	for _, w := range []string{"apple", "banana", "cat", "dog"} {
		fmt.Printf("check %s: %t\n", w, bf.check(w))
	}
}
