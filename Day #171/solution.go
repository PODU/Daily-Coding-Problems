// Busiest period: sort events by timestamp, sweep occupancy, track max-occupancy interval. O(n log n) time, O(n) space.
package main

import (
	"fmt"
	"sort"
)

type Type int

const (
	ENTER Type = iota
	EXIT
)

type Event struct {
	ts    int64
	count int
	typ   Type
}

func busiestPeriod(events []Event) (int64, int64) {
	sort.Slice(events, func(i, j int) bool { return events[i].ts < events[j].ts })
	var occ, maxOcc int64 = 0, -1
	var bestStart, bestEnd int64
	for i, e := range events {
		if e.typ == ENTER {
			occ += int64(e.count)
		} else {
			occ -= int64(e.count)
		}
		if occ > maxOcc && i+1 < len(events) {
			maxOcc = occ
			bestStart = e.ts
			bestEnd = events[i+1].ts
		}
	}
	return bestStart, bestEnd
}

func main() {
	events := []Event{
		{1526579928, 3, ENTER},
		{1526580382, 2, EXIT},
		{1526579999, 1, ENTER},
		{1526580001, 5, ENTER},
	}
	start, end := busiestPeriod(events)
	fmt.Printf("(%d, %d)\n", start, end)
}
