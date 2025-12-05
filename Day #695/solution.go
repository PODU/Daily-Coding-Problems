// Day 695: Uniform random in [0, n-1] excluding values in list l.
// Approach: precompute the allowed values once, then pick a uniform index.
// Preprocess O(n), each draw O(1).
package main

import (
	"fmt"
	"math/rand"
)

type RandExcluder struct {
	allowed []int
	rng     *rand.Rand
}

func newExcluder(n int, l []int) *RandExcluder {
	bad := make(map[int]bool)
	for _, v := range l {
		bad[v] = true
	}
	var allowed []int
	for x := 0; x < n; x++ {
		if !bad[x] {
			allowed = append(allowed, x)
		}
	}
	return &RandExcluder{allowed, rand.New(rand.NewSource(42))}
}

func (r *RandExcluder) next() int {
	return r.allowed[r.rng.Intn(len(r.allowed))]
}

func main() {
	r := newExcluder(10, []int{2, 4, 6, 8})
	fmt.Print("sample: ")
	for i := 0; i < 8; i++ {
		fmt.Print(r.next(), " ")
	}
	fmt.Println("\n(all values are in [0,9] and never 2,4,6,8)")
}
