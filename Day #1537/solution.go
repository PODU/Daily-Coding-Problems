// Simulate a Markov chain for num_steps and count visits per state.
// Time O(num_steps * outdegree), Space O(states + transitions).
package main

import (
	"fmt"
	"math/rand"
	"sort"
)

type edge struct {
	dst string
	p   float64
}

func runChain(start string, numSteps int, transitions [][3]interface{}, seed int64) map[string]int {
	trans := map[string][]edge{}
	for _, t := range transitions {
		src := t[0].(string)
		trans[src] = append(trans[src], edge{t[1].(string), t[2].(float64)})
	}
	rng := rand.New(rand.NewSource(seed))
	counts := map[string]int{}
	cur := start
	for s := 0; s < numSteps; s++ {
		counts[cur]++
		r, acc := rng.Float64(), 0.0
		for _, e := range trans[cur] {
			acc += e.p
			if r <= acc {
				cur = e.dst
				break
			}
		}
	}
	return counts
}

func main() {
	transitions := [][3]interface{}{
		{"a", "a", 0.9}, {"a", "b", 0.075}, {"a", "c", 0.025},
		{"b", "a", 0.15}, {"b", "b", 0.8}, {"b", "c", 0.05},
		{"c", "a", 0.25}, {"c", "b", 0.25}, {"c", "c", 0.5},
	}
	counts := runChain("a", 5000, transitions, 42)
	keys := make([]string, 0, len(counts))
	for k := range counts {
		keys = append(keys, k)
	}
	sort.Strings(keys)
	fmt.Print("{ ")
	for i, k := range keys {
		if i > 0 {
			fmt.Print(", ")
		}
		fmt.Printf("'%s': %d", k, counts[k])
	}
	fmt.Println(" }")
}
