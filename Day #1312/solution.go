// rand7 from rand5 via rejection sampling: combine two rand5 into uniform
// 1..25, accept 1..21, map to 1..7. Expected O(1) calls, O(1) space.
package main

import (
	"fmt"
	"math/rand"
)

var rng = rand.New(rand.NewSource(42))

func rand5() int { return rng.Intn(5) + 1 } // uniform 1..5

func rand7() int {
	for {
		v := (rand5()-1)*5 + (rand5() - 1) // uniform 0..24
		if v < 21 {
			return v%7 + 1 // accept 0..20 -> 1..7
		}
	}
}

func main() {
	var counts [8]int
	for i := 0; i < 70000; i++ {
		counts[rand7()]++
	}
	for i := 1; i <= 7; i++ {
		fmt.Printf("%d: %d\n", i, counts[i])
	}
}
