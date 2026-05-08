// Day 1489: Time-indexed map. Per key, keep entries sorted by time; get does
// binary search for the most recent time <= query. set O(log n), get O(log n).
package main

import (
	"fmt"
	"sort"
)

type entry struct {
	time, value int
}

type TimeMap struct {
	store map[int][]entry // key -> sorted-by-time entries
}

func NewTimeMap() *TimeMap { return &TimeMap{store: map[int][]entry{}} }

func (m *TimeMap) Set(key, value, time int) {
	v := m.store[key]
	i := sort.Search(len(v), func(i int) bool { return v[i].time >= time })
	if i < len(v) && v[i].time == time {
		v[i].value = value // overwrite same time
	} else {
		v = append(v, entry{})
		copy(v[i+1:], v[i:])
		v[i] = entry{time, value}
	}
	m.store[key] = v
}

// Get returns (value, true) or (0, false) if nothing set at/before time.
func (m *TimeMap) Get(key, time int) (int, bool) {
	v, ok := m.store[key]
	if !ok {
		return 0, false
	}
	// rightmost entry with time <= query
	i := sort.Search(len(v), func(i int) bool { return v[i].time > time })
	if i == 0 {
		return 0, false
	}
	return v[i-1].value, true
}

func show(d *TimeMap, key, time int) {
	if val, ok := d.Get(key, time); ok {
		fmt.Printf("get(%d,%d) = %d\n", key, time, val)
	} else {
		fmt.Printf("get(%d,%d) = null\n", key, time)
	}
}

func main() {
	d1 := NewTimeMap()
	d1.Set(1, 1, 0)
	d1.Set(1, 2, 2)
	show(d1, 1, 1)  // 1
	show(d1, 1, 3)  // 2

	d2 := NewTimeMap()
	d2.Set(1, 1, 5)
	show(d2, 1, 0)  // null
	show(d2, 1, 10) // 1

	d3 := NewTimeMap()
	d3.Set(1, 1, 0)
	d3.Set(1, 2, 0)
	show(d3, 1, 0) // 2
}
