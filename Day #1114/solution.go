// Day 1114 - Voting stream: top 3 candidates + fraud detection
// Approach: hash-map vote counts + set of seen voters (O(1) dup detection);
// top-3 via sort. Time: O(R + M log M), Space: O(V+M).
package main

import (
	"fmt"
	"sort"
	"strings"
)

type rec struct {
	voter int
	cand  string
}

func main() {
	stream := []rec{
		{1, "A"}, {2, "B"}, {3, "A"}, {4, "C"}, {5, "B"},
		{6, "A"}, {2, "C"}, {7, "D"}, {8, "A"},
	}
	counts := make(map[string]int)
	seen := make(map[int]bool)
	for _, r := range stream {
		if seen[r.voter] {
			fmt.Printf("Fraud detected: voter %d voted more than once\n", r.voter)
			continue
		}
		seen[r.voter] = true
		counts[r.cand]++
	}
	type kv struct {
		cand  string
		count int
	}
	var items []kv
	for c, n := range counts {
		items = append(items, kv{c, n})
	}
	sort.Slice(items, func(i, j int) bool {
		if items[i].count != items[j].count {
			return items[i].count > items[j].count
		}
		return items[i].cand < items[j].cand
	})
	var parts []string
	for i := 0; i < 3 && i < len(items); i++ {
		parts = append(parts, fmt.Sprintf("('%s', %d)", items[i].cand, items[i].count))
	}
	fmt.Printf("Top 3 candidates: [%s]\n", strings.Join(parts, ", "))
}
