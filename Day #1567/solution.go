// Busiest period: sort events by timestamp, sweep current occupancy, track max interval.
// Time O(n log n), space O(n).
package main

import (
	"fmt"
	"sort"
)

type Event struct {
	ts    int64
	count int64
	enter bool
}

func busiestPeriod(events []Event) (int64, int64) {
	sort.SliceStable(events, func(i, j int) bool {
		return events[i].ts < events[j].ts
	})
	var cur, best int64 = 0, -1
	var bestStart, bestEnd int64
	for i := 0; i < len(events); i++ {
		if events[i].enter {
			cur += events[i].count
		} else {
			cur -= events[i].count
		}
		if cur > best && i+1 < len(events) {
			best = cur
			bestStart = events[i].ts
			bestEnd = events[i+1].ts
		}
	}
	return bestStart, bestEnd
}

func main() {
	events := []Event{
		{1, 3, true},
		{4, 2, true},
		{6, 5, false},
	}
	start, end := busiestPeriod(events)
	fmt.Printf("(%d, %d)\n", start, end)
}
