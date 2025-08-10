// Day 97: Time-versioned map. Each key keeps times/vals sorted by time; get
// binary-searches the latest time <= query. set/get O(log n).
package main

import (
	"fmt"
	"sort"
)

type entry struct{ times, vals []int }

type TimeMap struct{ store map[int]*entry }

func NewTimeMap() *TimeMap { return &TimeMap{store: map[int]*entry{}} }

func (t *TimeMap) Set(key, value, time int) {
	e := t.store[key]
	if e == nil {
		e = &entry{}
		t.store[key] = e
	}
	i := sort.SearchInts(e.times, time)
	if i < len(e.times) && e.times[i] == time {
		e.vals[i] = value
		return
	}
	e.times = append(e.times, 0)
	copy(e.times[i+1:], e.times[i:])
	e.times[i] = time
	e.vals = append(e.vals, 0)
	copy(e.vals[i+1:], e.vals[i:])
	e.vals[i] = value
}

// Get returns (value, ok).
func (t *TimeMap) Get(key, time int) (int, bool) {
	e := t.store[key]
	if e == nil {
		return 0, false
	}
	i := sort.Search(len(e.times), func(i int) bool { return e.times[i] > time })
	if i == 0 {
		return 0, false
	}
	return e.vals[i-1], true
}

func show(v int, ok bool) {
	if ok {
		fmt.Println(v)
	} else {
		fmt.Println("null")
	}
}

func main() {
	// The README's three blocks are independent scenarios (fresh maps).
	a := NewTimeMap()
	a.Set(1, 1, 0); a.Set(1, 2, 2)
	show(a.Get(1, 1)) // 1
	show(a.Get(1, 3)) // 2

	b := NewTimeMap()
	b.Set(1, 1, 5)
	show(b.Get(1, 0))  // null
	show(b.Get(1, 10)) // 1

	c := NewTimeMap()
	c.Set(1, 1, 0); c.Set(1, 2, 0)
	show(c.Get(1, 0)) // 2
}
