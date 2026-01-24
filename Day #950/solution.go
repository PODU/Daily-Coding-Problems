// Day 950: busiest period - interval with the most people inside the building.
// Sort events by timestamp, sweep maintaining running count. Time O(n log n), Space O(n).
package main

import (
	"fmt"
	"sort"
)

type Event struct {
	ts    int64
	count int
	enter bool
}

func busiest(ev []Event) (int64, int64) {
	sort.Slice(ev, func(i, j int) bool { return ev[i].ts < ev[j].ts })
	var cur int64 = 0
	best := int64(-1 << 62)
	var s, e int64
	for i := range ev {
		if ev[i].enter {
			cur += int64(ev[i].count)
		} else {
			cur -= int64(ev[i].count)
		}
		nextTs := ev[i].ts
		if i+1 < len(ev) {
			nextTs = ev[i+1].ts
		}
		if cur > best {
			best = cur
			s, e = ev[i].ts, nextTs
		}
	}
	return s, e
}

func main() {
	ev := []Event{
		{1526579928, 3, true},
		{1526579943, 4, true},
		{1526580382, 2, false},
		{1526581000, 5, false},
	}
	s, e := busiest(ev)
	fmt.Printf("(%d, %d)\n", s, e) // (1526579943, 1526580382)
}
