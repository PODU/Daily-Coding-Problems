// HitCounter: maintain timestamps in a sorted slice via binary-search insert. record O(n) shift,
// total = len O(1), range = upperBound - lowerBound via binary search O(log n).
// Limited-memory follow-up: bucket hits by time window (circular buffer of fixed size)
// so memory stays O(window) instead of O(#hits), trading exact old-range queries for recency.
package main

import (
	"fmt"
	"sort"
)

type HitCounter struct {
	ts []int
}

func (h *HitCounter) record(t int) {
	pos := sort.SearchInts(h.ts, t)
	h.ts = append(h.ts, 0)
	copy(h.ts[pos+1:], h.ts[pos:])
	h.ts[pos] = t
}

func (h *HitCounter) total() int {
	return len(h.ts)
}

func (h *HitCounter) rangeCount(lower, upper int) int {
	lo := sort.SearchInts(h.ts, lower)
	hi := sort.SearchInts(h.ts, upper+1)
	return hi - lo
}

func main() {
	hc := &HitCounter{}
	hc.record(1)
	hc.record(2)
	hc.record(3)
	hc.record(2)
	fmt.Println(hc.total())            // 4
	fmt.Println(hc.rangeCount(2, 3))   // 3
}
