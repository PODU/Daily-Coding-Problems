// Day 640: Simulate a Markov chain and tally state visits.
// Approach: build outgoing-edge table, draw next state by cumulative prob.
// Time: O(num_steps * avg_out_degree), Space: O(states + edges).
// Note: output is stochastic; counts approximate the README sample (sum 5000).
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

func runMarkov(start string, trans [][3]interface{}, steps int, seed int64) map[string]int {
	adj := map[string][]edge{}
	for _, t := range trans {
		src := t[0].(string)
		adj[src] = append(adj[src], edge{t[1].(string), t[2].(float64)})
	}
	rng := rand.New(rand.NewSource(seed))
	counts := map[string]int{}
	cur := start
	for i := 0; i < steps; i++ {
		counts[cur]++
		r := rng.Float64()
		acc := 0.0
		for _, e := range adj[cur] {
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
	trans := [][3]interface{}{
		{"a", "a", 0.9}, {"a", "b", 0.075}, {"a", "c", 0.025},
		{"b", "a", 0.15}, {"b", "b", 0.8}, {"b", "c", 0.05},
		{"c", "a", 0.25}, {"c", "b", 0.25}, {"c", "c", 0.5},
	}
	counts := runMarkov("a", trans, 5000, 42)
	keys := []string{}
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
	fmt.Println(" }") // e.g. { 'a': 3012, 'b': 1656, 'c': 332 }
}
