// Reservoir sampling (reservoir size 1): for the i-th element replace kept with prob 1/i.
// Distribution is uniform over the stream. Time O(n), Space O(1). Seeded RNG -> deterministic.
package main

import (
	"fmt"
	"math/rand"
)

func reservoirSample(stream []int, seed int64) int {
	rng := rand.New(rand.NewSource(seed))
	kept := 0
	for i := 0; i < len(stream); i++ {
		// for the (i+1)-th element keep with prob 1/(i+1)
		if rng.Intn(i+1) == 0 {
			kept = stream[i]
		}
	}
	return kept
}

func main() {
	stream := make([]int, 0, 10)
	for v := 1; v <= 10; v++ {
		stream = append(stream, v)
	}
	selected := reservoirSample(stream, 42)
	fmt.Printf("Selected: %d\n", selected)
}
