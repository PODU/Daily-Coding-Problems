// Day 1756: HitCounter design.
// Store timestamps in a sorted slice; total() O(1), range() via binary search O(log n).
// Limited-memory follow-up: bucket hits by coarse time granularity (e.g. per-second
// counts in a map/ring buffer) so memory is O(#buckets) instead of O(#hits).
package main

import (
	"fmt"
	"sort"
)

type HitCounter struct {
	hits []int // kept sorted
}

func (h *HitCounter) Record(timestamp int) {
	idx := sort.SearchInts(h.hits, timestamp+1) // first > timestamp
	h.hits = append(h.hits, 0)
	copy(h.hits[idx+1:], h.hits[idx:])
	h.hits[idx] = timestamp
}

func (h *HitCounter) Total() int { return len(h.hits) }

func (h *HitCounter) Range(lower, upper int) int {
	lo := sort.SearchInts(h.hits, lower)   // first >= lower
	hi := sort.SearchInts(h.hits, upper+1) // first > upper
	return hi - lo
}

func main() {
	hc := &HitCounter{}
	for _, t := range []int{1, 2, 2, 5, 7, 9, 10} {
		hc.Record(t)
	}

	fmt.Println("total() =", hc.Total())          // 7
	fmt.Println("range(2, 7) =", hc.Range(2, 7))   // 4
	fmt.Println("range(0, 10) =", hc.Range(0, 10)) // 7
}
