// PageRank: iterative power method with damping d=0.85, dangling-node mass redistributed evenly.
// Time O(iters*(N+E)), Space O(N+E).
package main

import (
	"fmt"
	"math"
	"sort"
)

func main() {
	nodes := []string{"A", "B", "C", "D"}
	edges := map[string][]string{
		"A": {"B", "C"}, "B": {"C"}, "C": {"A"}, "D": {"C"},
	}
	n := len(nodes)
	d := 0.85
	score := map[string]float64{}
	for _, nd := range nodes {
		score[nd] = 1.0 / float64(n)
	}

	for it := 0; it < 100; it++ {
		dangling := 0.0
		for _, nd := range nodes {
			if len(edges[nd]) == 0 {
				dangling += score[nd]
			}
		}
		next := map[string]float64{}
		for _, nd := range nodes {
			next[nd] = (1.0-d)/float64(n) + d*dangling/float64(n)
		}
		for _, nd := range nodes {
			outs := edges[nd]
			if len(outs) == 0 {
				continue
			}
			share := d * score[nd] / float64(len(outs))
			for _, t := range outs {
				next[t] += share
			}
		}
		diff := 0.0
		for _, nd := range nodes {
			diff += math.Abs(next[nd] - score[nd])
		}
		score = next
		if diff < 1e-9 {
			break
		}
	}

	sorted := append([]string{}, nodes...)
	sort.Strings(sorted)
	for _, nd := range sorted {
		fmt.Printf("%s %.6f\n", nd, score[nd])
	}
}
