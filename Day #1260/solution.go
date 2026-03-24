// Busiest period: sort events by timestamp, sweep occupancy, track max-occupancy interval [t_i, t_{i+1}). O(n log n) time, O(n) space.
package main

import (
	"fmt"
	"sort"
)

type Event struct {
	timestamp int64
	count     int
	typ       string
}

func main() {
	events := []Event{
		{1526579928, 3, "enter"},
		{1526580000, 2, "enter"},
		{1526580382, 2, "exit"},
		{1526580500, 1, "enter"},
		{1526580700, 4, "exit"},
	}
	sort.Slice(events, func(i, j int) bool {
		return events[i].timestamp < events[j].timestamp
	})

	var current, maxOcc int64 = 0, -1
	var bestStart, bestEnd int64
	for i := range events {
		if events[i].typ == "enter" {
			current += int64(events[i].count)
		} else {
			current -= int64(events[i].count)
		}
		nextTs := events[i].timestamp
		if i+1 < len(events) {
			nextTs = events[i+1].timestamp
		}
		if current > maxOcc {
			maxOcc = current
			bestStart = events[i].timestamp
			bestEnd = nextTs
		}
	}
	fmt.Printf("(%d, %d)\n", bestStart, bestEnd)
}
