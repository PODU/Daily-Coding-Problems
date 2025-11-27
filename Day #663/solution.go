// Day 663: HitCounter. Keep timestamps sorted; total = O(1); range = binary search.
// record O(n) sorted insert, total O(1), range O(log n).
// Limited-memory follow-up: bucket hits into fixed time windows storing (window, count).
package main

import (
	"fmt"
	"sort"
)

type HitCounter struct{ ts []int }

func (h *HitCounter) record(t int) {
	i := sort.SearchInts(h.ts, t)
	h.ts = append(h.ts, 0)
	copy(h.ts[i+1:], h.ts[i:])
	h.ts[i] = t
}
func (h *HitCounter) total() int { return len(h.ts) }
func (h *HitCounter) rng(lo, hi int) int {
	l := sort.SearchInts(h.ts, lo)
	r := sort.SearchInts(h.ts, hi+1)
	return r - l
}

func main() {
	h := &HitCounter{}
	for _, t := range []int{1, 2, 2, 5, 9, 10} {
		h.record(t)
	}
	fmt.Println("total:", h.total())     // 6
	fmt.Println("range(2,9):", h.rng(2, 9)) // 4
}
