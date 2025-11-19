// Day 631: Skyline problem.
// Approach: sweep line over edges + multiset (map) of active heights.
// Time: O(n log n), Space: O(n).
package main

import (
	"fmt"
	"sort"
	"strings"
)

func getSkyline(buildings [][3]int) [][2]int {
	type ev struct{ x, h int }
	events := []ev{}
	for _, b := range buildings {
		events = append(events, ev{b[0], -b[2]}) // start
		events = append(events, ev{b[1], b[2]})  // end
	}
	sort.Slice(events, func(i, j int) bool {
		if events[i].x != events[j].x {
			return events[i].x < events[j].x
		}
		return events[i].h < events[j].h
	})
	cnt := map[int]int{0: 1}
	res := [][2]int{}
	prev := 0
	curMax := func() int {
		m := 0
		for k, c := range cnt {
			if c > 0 && k > m {
				m = k
			}
		}
		return m
	}
	for _, e := range events {
		if e.h < 0 {
			cnt[-e.h]++
		} else {
			cnt[e.h]--
		}
		cur := curMax()
		if cur != prev {
			res = append(res, [2]int{e.x, cur})
			prev = cur
		}
	}
	return res
}

func main() {
	buildings := [][3]int{{0, 15, 3}, {4, 11, 5}, {19, 23, 4}}
	sky := getSkyline(buildings)
	parts := []string{}
	for _, p := range sky {
		parts = append(parts, fmt.Sprintf("(%d, %d)", p[0], p[1]))
	}
	fmt.Println("[" + strings.Join(parts, ", ") + "]")
}
