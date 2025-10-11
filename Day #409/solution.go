// Day 409: PageRank via power iteration with damping d=0.85.
// Approach: iterate score(j)=(1-d)/N + d*sum(score(i)/Ci); dangling rank
// is spread over all nodes. Time: O(iters*(N+E)), Space: O(N+E).
package main

import (
	"fmt"
	"math"
	"sort"
)

func pageRank(graph map[string][]string, d float64, iters int, eps float64) map[string]float64 {
	n := float64(len(graph))
	rank := map[string]float64{}
	for node := range graph {
		rank[node] = 1.0 / n
	}
	for it := 0; it < iters; it++ {
		// Dangling nodes (no out-links) leak rank; redistribute it evenly.
		dangling := 0.0
		for node, outs := range graph {
			if len(outs) == 0 {
				dangling += rank[node]
			}
		}
		next := map[string]float64{}
		for node := range graph {
			next[node] = (1.0-d)/n + d*dangling/n
		}
		for node, outs := range graph {
			if len(outs) == 0 {
				continue
			}
			share := rank[node] / float64(len(outs))
			for _, nbr := range outs {
				next[nbr] += d * share
			}
		}
		diff := 0.0
		for node := range graph {
			diff += math.Abs(next[node] - rank[node])
		}
		rank = next
		if diff < eps {
			break
		}
	}
	return rank
}

func main() {
	graph := map[string][]string{
		"A": {"B", "C"},
		"B": {"C"},
		"C": {"A"},
	}
	rank := pageRank(graph, 0.85, 100, 1e-12)
	nodes := make([]string, 0, len(rank))
	for node := range rank {
		nodes = append(nodes, node)
	}
	sort.Strings(nodes)
	for _, node := range nodes {
		fmt.Printf("%s: %.4f\n", node, rank[node])
	}
}
