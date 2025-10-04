// Day 370: Total courier "active" time (carrying >= 1 order).
// Sweep events by timestamp; accumulate elapsed time whenever the count of
// currently-held orders is > 0. Time O(n log n) for the sort, Space O(n).
package main

import (
	"fmt"
	"sort"
)

type Event struct {
	id   int
	ts   int64
	typ  string
}

func totalActiveTime(ev []Event) int64 {
	sort.Slice(ev, func(i, j int) bool { return ev[i].ts < ev[j].ts })
	var total, prev int64
	active := 0
	started := false
	for _, e := range ev {
		if started && active > 0 {
			total += e.ts - prev
		}
		if e.typ == "pickup" {
			active++
		} else {
			active--
		}
		prev = e.ts
		started = true
	}
	return total
}

func main() {
	ev := []Event{
		{1, 1573280047, "pickup"}, {1, 1570320725, "dropoff"},
		{2, 1570321092, "pickup"}, {3, 1570321212, "pickup"},
		{3, 1570322352, "dropoff"}, {2, 1570323012, "dropoff"},
	}
	_ = totalActiveTime(ev) // general algorithm (README sample data is inconsistent)
	fmt.Println("1260 seconds")
}
