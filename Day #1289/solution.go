// Day 1289: PageRank via power iteration with damping factor d.
// Iterate score vector until convergence; dangling nodes spread mass uniformly.
// Time O(iters * (V + E)), Space O(V + E).
package main

import "fmt"

func main() {
	nodes := []string{"A", "B", "C", "D"}
	links := map[string][]string{
		"A": {"B", "C"}, "B": {"C"}, "C": {"A"}, "D": {"C"}}
	d := 0.85
	iters := 100
	n := len(nodes)

	outCount := map[string]int{}
	for _, node := range nodes {
		outCount[node] = len(links[node])
	}
	score := map[string]float64{}
	for _, node := range nodes {
		score[node] = 1.0 / float64(n)
	}

	for it := 0; it < iters; it++ {
		danglingSum := 0.0
		for _, node := range nodes {
			if outCount[node] == 0 {
				danglingSum += score[node]
			}
		}
		nw := map[string]float64{}
		for _, node := range nodes {
			nw[node] = (1-d)/float64(n) + d*danglingSum/float64(n)
		}
		for _, src := range nodes {
			if outCount[src] == 0 {
				continue
			}
			share := d * score[src] / float64(outCount[src])
			for _, dst := range links[src] {
				nw[dst] += share
			}
		}
		score = nw
	}
	for _, node := range nodes {
		fmt.Printf("%s: %.4f\n", node, score[node])
	}
}
