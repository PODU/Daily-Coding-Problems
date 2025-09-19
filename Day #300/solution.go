// Stream voting: track seen voters + candidate counts; report top-3 (count desc, id asc) or FRAUD on repeat.
// Time: O(n * c log c) over stream, Space: O(voters + candidates).
package main

import (
	"fmt"
	"sort"
	"strings"
)

type record struct {
	voter int
	cand  string
}

func main() {
	stream := []record{
		{1, "A"}, {2, "B"}, {3, "A"}, {4, "C"}, {5, "B"}, {1, "A"}, {6, "A"},
	}

	seen := map[int]bool{}
	counts := map[string]int{}

	for _, rec := range stream {
		if seen[rec.voter] {
			fmt.Printf("Fraud: voter %d voted more than once\n", rec.voter)
			continue
		}
		seen[rec.voter] = true
		counts[rec.cand]++

		type kv struct {
			id    string
			count int
		}
		var ranked []kv
		for id, c := range counts {
			ranked = append(ranked, kv{id, c})
		}
		sort.Slice(ranked, func(i, j int) bool {
			if ranked[i].count != ranked[j].count {
				return ranked[i].count > ranked[j].count
			}
			return ranked[i].id < ranked[j].id
		})

		limit := len(ranked)
		if limit > 3 {
			limit = 3
		}
		parts := make([]string, 0, limit)
		for i := 0; i < limit; i++ {
			parts = append(parts, "'"+ranked[i].id+"'")
		}
		fmt.Printf("Top 3: [%s]\n", strings.Join(parts, ", "))
	}
}
