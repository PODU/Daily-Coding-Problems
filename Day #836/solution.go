// Day 836: Fisher-Yates shuffle using only a rand(k) RNG (uniform 1..k) and swaps.
// For i=n-1..1: pick j uniform in 0..i via rand(i+1)-1, swap a[i],a[j]. O(N) time, O(1) extra.
// Unbiased: each step picks uniformly among i+1 positions, so all n! permutations are equally likely.
package main

import (
	"fmt"
	"strings"
)

type RNG struct{ state uint64 }

func (r *RNG) next() uint64 {
	r.state = r.state*6364136223846793005 + 1442695040888963407
	return r.state >> 16
}

// rand returns a uniform int in [1, k] with rejection to avoid modulo bias.
func (r *RNG) rand(k uint64) uint64 {
	mask := uint64(1)<<48 - 1
	limit := (uint64(1) << 48) - ((uint64(1) << 48) % k)
	for {
		v := r.next() & mask
		if v < limit {
			return v%k + 1
		}
	}
}

func shuffle(a []int, rng *RNG) {
	for i := len(a) - 1; i > 0; i-- {
		j := int(rng.rand(uint64(i+1))) - 1 // uniform 0..i
		a[i], a[j] = a[j], a[i]
	}
}

func main() {
	deck := make([]int, 52)
	for i := range deck {
		deck[i] = i + 1
	}
	rng := &RNG{state: 12345}
	shuffle(deck, rng)
	parts := make([]string, len(deck))
	for i, v := range deck {
		parts[i] = fmt.Sprintf("%d", v)
	}
	fmt.Println(strings.Join(parts, " "))
}
