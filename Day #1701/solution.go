// Stream voting: map vote counts + set of seen voters; duplicate voter = fraud.
// Top 3 by count (ties by candidate id). Time O(n + k log k), Space O(k+v).
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
	records := []rec{{1, "A"}, {2, "B"}, {3, "A"}, {4, "C"}, {2, "A"}, {5, "B"}, {6, "A"}}
	counts := map[string]int{}
	seen := map[int]bool{}
	for _, r := range records {
		if seen[r.voter] {
			fmt.Printf("Fraud detected: voter %d voted more than once\n", r.voter)
			continue
		}
		seen[r.voter] = true
		counts[r.cand]++
	}
	type cc struct {
		c string
		n int
	}
	var v []cc
	for c, n := range counts {
		v = append(v, cc{c, n})
	}
	sort.Slice(v, func(i, j int) bool {
		if v[i].n != v[j].n {
			return v[i].n > v[j].n
		}
		return v[i].c < v[j].c
	})
	var parts []string
	for i := 0; i < 3 && i < len(v); i++ {
		parts = append(parts, fmt.Sprintf("%s(%d)", v[i].c, v[i].n))
	}
	fmt.Println("Top 3 candidates: " + strings.Join(parts, ", "))
}
