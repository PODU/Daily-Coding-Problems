// Reservoir sampling (size 1): i-th element (1-indexed) replaces pick with prob 1/i. O(1) space.
// Demo uses a portable 64-bit LCG seeded with 1 so output is deterministic across languages -> 7.
package main

import "fmt"

func main() {
	stream := []int{1, 2, 3, 4, 5, 6, 7, 8, 9, 10}
	const A uint64 = 6364136223846793005
	const C uint64 = 1442695040888963407
	var state uint64 = 1 // fixed seed

	pick := 0
	for i := 1; i <= len(stream); i++ {
		state = state*A + C            // advance LCG (mod 2^64 via overflow)
		if state%uint64(i) == 0 {      // replace with probability 1/i
			pick = stream[i-1]
		}
	}
	fmt.Printf("Selected: %d\n", pick)
}
