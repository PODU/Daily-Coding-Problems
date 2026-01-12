// rand5 from rand7 via rejection sampling: keep rand7 values in 1..5. Expected O(1) calls (7/5).
package main

import (
	"fmt"
	"math/rand"
)

var rng = rand.New(rand.NewSource(12345))

func rand7() int { return rng.Intn(7) + 1 }

func rand5() int {
	x := rand7()
	for x > 5 {
		x = rand7()
	}
	return x
}

func main() {
	counts := make([]int, 6)
	trials := 100000
	for i := 0; i < trials; i++ {
		counts[rand5()]++
	}
	fmt.Printf("Distribution over %d samples:\n", trials)
	for v := 1; v <= 5; v++ {
		fmt.Printf("%d: %d\n", v, counts[v])
	}
}
