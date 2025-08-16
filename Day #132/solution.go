// Day 132: HitCounter (record, total, range).
// Keep timestamps sorted; range via binary search. total O(1), range O(log n).
// Limited-memory follow-up: bucket counts by coarse time granularity instead of per-hit.
package main

import (
	"fmt"
	"sort"
)

type HitCounter struct {
	ts []int
}

func (h *HitCounter) record(t int) {
	i := sort.SearchInts(h.ts, t+1) // upper bound
	h.ts = append(h.ts, 0)
	copy(h.ts[i+1:], h.ts[i:])
	h.ts[i] = t
}

func (h *HitCounter) total() int { return len(h.ts) }

func (h *HitCounter) rng(lo, hi int) int {
	a := sort.SearchInts(h.ts, lo)
	b := sort.SearchInts(h.ts, hi+1)
	return b - a
}

func main() {
	hc := &HitCounter{}
	for _, t := range []int{1, 1, 2, 3, 5, 8, 8, 10} {
		hc.record(t)
	}
	fmt.Println("total =", hc.total())     // 8
	fmt.Println("range(2, 8) =", hc.rng(2, 8)) // 5
}
