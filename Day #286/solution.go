// Day 286: Skyline problem.
// Sweep line over events; a count-map acts as a multiset of active heights.
// Time: O(n log n), Space: O(n).
package main

import (
	"fmt"
	"sort"
)

func getSkyline(buildings [][3]int) [][2]int {
	type ev struct{ x, h int }
	events := []ev{}
	for _, b := range buildings {
		events = append(events, ev{b[0], -b[2]})
		events = append(events, ev{b[1], b[2]})
	}
	sort.Slice(events, func(i, j int) bool {
		if events[i].x != events[j].x {
			return events[i].x < events[j].x
		}
		return events[i].h < events[j].h
	})
	live := map[int]int{0: 1}
	maxKey := func() int {
		mx := 0
		for k := range live {
			if k > mx {
				mx = k
			}
		}
		return mx
	}
	prev := 0
	res := [][2]int{}
	for _, e := range events {
		if e.h < 0 {
			live[-e.h]++
		} else {
			live[e.h]--
			if live[e.h] == 0 {
				delete(live, e.h)
			}
		}
		cur := maxKey()
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
	out := "["
	for i, p := range sky {
		out += fmt.Sprintf("(%d, %d)", p[0], p[1])
		if i+1 < len(sky) {
			out += ", "
		}
	}
	out += "]"
	fmt.Println(out)
}
