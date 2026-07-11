// Uniform random in [0,n-1] excluding values in l: precompute sorted allowed array, pick random index. O(n) build, O(allowed) space.
package main

import (
	"fmt"
	"math/rand"
	"strings"
)

type RandomPicker struct {
	allowed []int
	rng     *rand.Rand
}

func NewRandomPicker(n int, l []int) *RandomPicker {
	ex := make(map[int]bool)
	for _, x := range l {
		ex[x] = true
	}
	var allowed []int
	for i := 0; i < n; i++ {
		if !ex[i] {
			allowed = append(allowed, i)
		}
	}
	return &RandomPicker{allowed: allowed, rng: rand.New(rand.NewSource(12345))}
}

func (r *RandomPicker) Pick() int { return r.allowed[r.rng.Intn(len(r.allowed))] }

func main() {
	rp := NewRandomPicker(10, []int{2, 3, 5, 7})
	aset := make(map[int]bool)
	parts := make([]string, len(rp.allowed))
	for i, v := range rp.allowed {
		aset[v] = true
		parts[i] = fmt.Sprintf("%d", v)
	}
	fmt.Println("allowed=[" + strings.Join(parts, ", ") + "]")
	ok := true
	for i := 0; i < 1000; i++ {
		if !aset[rp.Pick()] {
			ok = false
		}
	}
	fmt.Printf("sample in allowed: %v\n", ok)
}
