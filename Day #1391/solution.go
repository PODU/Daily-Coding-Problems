// Von Neumann fair coin from a biased toss: toss twice, (0,1)->0, (1,0)->1, else retry. O(1) expected per fair toss.
package main

import (
	"fmt"
	"math/rand"
)

var rng = rand.New(rand.NewSource(12345))

func tossBiased() int {
	if rng.Float64() < 0.3 {
		return 1
	}
	return 0
}

func fairToss() int {
	for {
		a, b := tossBiased(), tossBiased()
		if a == 0 && b == 1 {
			return 0
		}
		if a == 1 && b == 0 {
			return 1
		}
	}
}

func main() {
	n, heads := 100000, 0
	for i := 0; i < n; i++ {
		heads += fairToss()
	}
	fmt.Printf("heads fraction: %.2f\n", float64(heads)/float64(n))
}
