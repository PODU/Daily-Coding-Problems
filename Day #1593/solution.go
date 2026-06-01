// rand5 from rand7 via rejection sampling: draw rand7, accept if <=5 else retry.
// Expected O(1) calls (acceptance prob 5/7). Output uniform on 1..5.
package main

import (
	"fmt"
	"math/rand"
	"strings"
)

var rng = rand.New(rand.NewSource(12345)) // deterministic seed

// uniform 1..7 using language RNG
func rand7() int {
	return rng.Intn(7) + 1
}

// uniform 1..5 via rejection sampling
func rand5() int {
	for {
		v := rand7()
		if v <= 5 {
			return v
		}
	}
}

func main() {
	const N = 100000
	var counts [6]int
	for i := 0; i < N; i++ {
		counts[rand5()]++
	}

	fmt.Printf("Distribution over %d samples:\n", N)
	for v := 1; v <= 5; v++ {
		fmt.Printf("  %d: %d\n", v, counts[v])
	}

	parts := make([]string, 10)
	for i := 0; i < 10; i++ {
		parts[i] = fmt.Sprintf("%d", rand5())
	}
	fmt.Println("First 10 samples: " + strings.Join(parts, " "))
}
