// HitCounter: keep timestamps in a sorted slice (binary-insert); record O(n), total O(1), range via binary search (upper-lower).
package main

import (
	"fmt"
	"sort"
)

type HitCounter struct {
	hits []int
	cnt  int64
}

func (h *HitCounter) record(t int) {
	i := sort.SearchInts(h.hits, t)
	h.hits = append(h.hits, 0)
	copy(h.hits[i+1:], h.hits[i:])
	h.hits[i] = t
	h.cnt++
}

func (h *HitCounter) total() int64 { return h.cnt }

func (h *HitCounter) rangeCount(lo, hi int) int {
	left := sort.SearchInts(h.hits, lo)
	right := sort.SearchInts(h.hits, hi+1)
	return right - left
}

func main() {
	hc := &HitCounter{}
	for _, t := range []int{1, 1, 2, 3, 5, 8} {
		hc.record(t)
	}
	fmt.Printf("total: %d\n", hc.total())
	fmt.Printf("range(2,5): %d\n", hc.rangeCount(2, 5))
}
