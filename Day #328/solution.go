// Simplified Elo: expected = 1/(1+10^((Rb-Ra)/400)); delta = round(K*(s-expected)); zero-sum transfer.
// Time O(1), Space O(1).
package main

import (
	"fmt"
	"math"
)

func main() {
	Ra, Rb, K := 1200, 2000, 32
	expectedA := 1.0 / (1.0 + math.Pow(10.0, float64(Rb-Ra)/400.0))
	delta := int(math.Round(float64(K) * (1.0 - expectedA))) // A wins, s=1
	newA := Ra + delta
	newB := Rb - delta
	fmt.Printf("Winner (%d) -> %d, Loser (%d) -> %d\n", Ra, newA, Rb, newB)
}
