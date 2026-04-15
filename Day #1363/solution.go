// Stream voting: map candidate->count, set of voters to detect fraud; top-3 via sort.
// Time: O(records) processing + O(C log C) reporting. Space: O(C + V).
package main

import (
	"fmt"
	"sort"
	"strings"
)

func main() {
	stream := [][2]string{
		{"v1", "A"}, {"v2", "B"}, {"v3", "A"}, {"v4", "C"},
		{"v5", "B"}, {"v6", "A"}, {"v7", "C"}, {"v1", "B"},
	}

	counts := map[string]int{}
	seen := map[string]bool{}

	for _, rec := range stream {
		voter, cand := rec[0], rec[1]
		if seen[voter] {
			fmt.Printf("Fraud detected: voter %s\n", voter)
			continue
		}
		seen[voter] = true
		counts[cand]++
	}

	type cc struct {
		name  string
		count int
	}
	var v []cc
	for name, c := range counts {
		v = append(v, cc{name, c})
	}
	sort.Slice(v, func(i, j int) bool {
		if v[i].count != v[j].count {
			return v[i].count > v[j].count
		}
		return v[i].name < v[j].name
	})

	var top []string
	for i := 0; i < len(v) && i < 3; i++ {
		top = append(top, v[i].name)
	}
	fmt.Printf("Top 3 candidates: %s\n", strings.Join(top, ", "))
}
