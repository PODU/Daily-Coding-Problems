// Day 734: Time-travel map; get(key,t) returns value set at the most recent time <= t.
// Approach: per key a sorted slice of (time,value); get uses binary search.
// Time: set O(n) insert, get O(log n). Space: O(n).
package main

import (
	"fmt"
	"sort"
)

type entry struct{ time, value int }

type TimeMap struct {
	store map[int][]entry
}

func NewTimeMap() *TimeMap { return &TimeMap{store: make(map[int][]entry)} }

func (t *TimeMap) Set(key, value, time int) {
	arr := t.store[key]
	i := sort.Search(len(arr), func(i int) bool { return arr[i].time >= time })
	if i < len(arr) && arr[i].time == time {
		arr[i].value = value
	} else {
		arr = append(arr, entry{})
		copy(arr[i+1:], arr[i:])
		arr[i] = entry{time, value}
	}
	t.store[key] = arr
}

func (t *TimeMap) Get(key, time int) (int, bool) {
	arr, ok := t.store[key]
	if !ok {
		return 0, false
	}
	i := sort.Search(len(arr), func(i int) bool { return arr[i].time > time })
	if i == 0 {
		return 0, false
	}
	return arr[i-1].value, true
}

func show(v int, ok bool) {
	if ok {
		fmt.Println(v)
	} else {
		fmt.Println("null")
	}
}

func main() {
	d1 := NewTimeMap()
	d1.Set(1, 1, 0)
	d1.Set(1, 2, 2)
	show(d1.Get(1, 1)) // 1
	show(d1.Get(1, 3)) // 2
	d2 := NewTimeMap()
	d2.Set(1, 1, 5)
	show(d2.Get(1, 0))  // null
	show(d2.Get(1, 10)) // 1
	d3 := NewTimeMap()
	d3.Set(1, 1, 0)
	d3.Set(1, 2, 0)
	show(d3.Get(1, 0)) // 2
}
