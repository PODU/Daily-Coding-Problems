// Von Neumann debiasing: toss biased coin twice; (0,1)->0, (1,0)->1, else retry.
// Output is provably fair regardless of bias. O(1) expected tosses per fair bit.
package main

import (
	"fmt"
	"math/rand"
)

func tossBiased(r *rand.Rand) int { // P(1) = 0.3
	if r.Float64() < 0.3 {
		return 1
	}
	return 0
}

func fairCoin(r *rand.Rand) int {
	for {
		a := tossBiased(r)
		b := tossBiased(r)
		if a == 0 && b == 1 {
			return 0
		}
		if a == 1 && b == 0 {
			return 1
		}
	}
}

func main() {
	r := rand.New(rand.NewSource(42))
	const N = 100000
	ones := 0
	for i := 0; i < N; i++ {
		ones += fairCoin(r)
	}
	fmt.Printf("%.1f\n", float64(ones)/float64(N))
}
