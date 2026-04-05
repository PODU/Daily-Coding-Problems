// Day 1304: Uniformly sample an integer in [0, n) not in list l.
// Precompute sorted excluded; pick r in [0, n-|excl|) and map to the r-th allowed value.
// Preprocess O(m log m); each draw O(m). Uniform over all allowed values.
package main

import (
	"fmt"
	"math/rand"
	"sort"
)

type RandExcluder struct {
	n    int
	excl []int
	rng  *rand.Rand
}

func NewRandExcluder(n int, l []int, seed int64) *RandExcluder {
	seen := map[int]bool{}
	var excl []int
	for _, x := range l {
		if x >= 0 && x < n && !seen[x] {
			seen[x] = true
			excl = append(excl, x)
		}
	}
	sort.Ints(excl)
	return &RandExcluder{n: n, excl: excl, rng: rand.New(rand.NewSource(seed))}
}

func (r *RandExcluder) Next() int {
	count := r.n - len(r.excl)
	res := r.rng.Intn(count)
	for _, e := range r.excl {
		if e <= res {
			res++
		} else {
			break
		}
	}
	return res
}

func main() {
	r := NewRandExcluder(5, []int{1, 3}, 42)
	seen := map[int]bool{}
	for i := 0; i < 1000; i++ {
		seen[r.Next()] = true
	}
	var out []int
	for v := range seen {
		out = append(out, v)
	}
	sort.Ints(out)
	fmt.Println(out) // [0 2 4]
}
