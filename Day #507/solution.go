// Streaming vote tally: set of seen voters (duplicate -> fraud, vote dropped),
// map candidate->count, top 3 computed on demand. Time O(n + k log k), Space O(n+k).
package main

import (
	"fmt"
	"sort"
	"strings"
)

func main() {
	stream := [][2]string{
		{"v1", "A"}, {"v2", "A"}, {"v3", "B"}, {"v4", "C"},
		{"v5", "B"}, {"v6", "B"}, {"v7", "C"}, {"v2", "D"},
	}

	seen := map[string]bool{}
	tally := map[string]int{}

	for _, rec := range stream {
		voter, cand := rec[0], rec[1]
		if seen[voter] {
			fmt.Printf("Fraud detected: voter %s voted more than once\n", voter)
			continue // do not count fraudulent vote
		}
		seen[voter] = true
		tally[cand]++
	}

	type pair struct {
		cand  string
		count int
	}
	var v []pair
	for c, n := range tally {
		v = append(v, pair{c, n})
	}
	sort.Slice(v, func(i, j int) bool {
		if v[i].count != v[j].count {
			return v[i].count > v[j].count // higher votes first
		}
		return v[i].cand < v[j].cand // tie: candidate id ascending
	})

	n := 3
	if len(v) < n {
		n = len(v)
	}
	var parts []string
	for i := 0; i < n; i++ {
		parts = append(parts, fmt.Sprintf("%s(%d)", v[i].cand, v[i].count))
	}
	fmt.Println("Top 3: " + strings.Join(parts, ", "))
}
