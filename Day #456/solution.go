// Day 456: Busiest period in a building from enter/exit events.
// Sort by timestamp, sweep occupancy, track interval of max. Time O(n log n).
package main

import (
	"fmt"
	"sort"
)

type Event struct {
	ts    int64
	count int
	typ   string
}

func busiest(ev []Event) (int64, int64) {
	sort.Slice(ev, func(i, j int) bool { return ev[i].ts < ev[j].ts })
	var cur, best int64 = 0, -1
	var bestStart, bestEnd int64
	n := len(ev)
	for i := 0; i < n; i++ {
		if ev[i].typ == "enter" {
			cur += int64(ev[i].count)
		} else {
			cur -= int64(ev[i].count)
		}
		end := ev[i].ts
		if i+1 < n {
			end = ev[i+1].ts
		}
		if cur > best {
			best, bestStart, bestEnd = cur, ev[i].ts, end
		}
	}
	return bestStart, bestEnd
}

func main() {
	ev := []Event{
		{1526579928, 3, "enter"},
		{1526579940, 2, "enter"},
		{1526580000, 1, "exit"},
		{1526580382, 4, "exit"},
	}
	s, e := busiest(ev)
	fmt.Printf("(%d, %d)\n", s, e) // (1526579940, 1526580000)
}
