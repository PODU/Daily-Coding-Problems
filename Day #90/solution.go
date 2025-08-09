// Day 90: Uniform random in [0,n) excluding l. Pick r in [0, n-k), then offset past
// sorted exclusions so every valid value is equally likely. Time O(k log k), Space O(k).
package main

import (
	"fmt"
	"math/rand"
	"sort"
)

func randomExcluding(n int, l []int, rng *rand.Rand) int {
	seen := map[int]bool{}
	ex := []int{}
	for _, v := range l {
		if v >= 0 && v < n && !seen[v] {
			seen[v] = true
			ex = append(ex, v)
		}
	}
	sort.Ints(ex)
	m := n - len(ex)
	if m <= 0 {
		panic("no valid number")
	}
	r := rng.Intn(m)
	for _, e := range ex {
		if e <= r {
			r++
		} else {
			break
		}
	}
	return r
}

func main() {
	rng := rand.New(rand.NewSource(42))
	fmt.Println(randomExcluding(5, []int{1, 3}, rng)) // sample from {0,2,4}
}
