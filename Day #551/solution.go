// Von Neumann fair-coin from biased coin: toss pairs, (0,1)->0, (1,0)->1, else retry.
// Expected O(1) tosses per fair toss; O(1) space.
package main

import "fmt"

var rngState uint64 = 88172645463325252

func nextUniform() float64 { // xorshift64 -> [0,1)
	rngState ^= rngState << 13
	rngState ^= rngState >> 7
	rngState ^= rngState << 17
	return float64(rngState>>11) * (1.0 / 9007199254740992.0)
}

func tossBiased() int { // p(1) = 0.3
	if nextUniform() < 0.3 {
		return 1
	}
	return 0
}

func fairToss() int {
	for {
		a := tossBiased()
		b := tossBiased()
		if a == 0 && b == 1 {
			return 0
		}
		if a == 1 && b == 0 {
			return 1
		}
	}
}

func main() {
	var heads, tails int
	for i := 0; i < 100000; i++ {
		if fairToss() == 1 {
			heads++
		} else {
			tails++
		}
	}
	fmt.Printf("heads: %d, tails: %d\n", heads, tails)
}
