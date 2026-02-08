// Time-versioned map: per key a sorted slice of (time,value); get binary-searches for floor.
// set/get O(log n) lookup. Setting same key+time overwrites.
package main

import (
	"fmt"
	"sort"
)

type entry struct {
	time, value int
}

type TimeMap struct {
	store map[int][]entry
}

func NewTimeMap() *TimeMap { return &TimeMap{store: make(map[int][]entry)} }

func (t *TimeMap) set(key, value, time int) {
	arr := t.store[key]
	idx := sort.Search(len(arr), func(i int) bool { return arr[i].time >= time })
	if idx < len(arr) && arr[idx].time == time {
		arr[idx].value = value // overwrite
	} else {
		arr = append(arr, entry{})
		copy(arr[idx+1:], arr[idx:])
		arr[idx] = entry{time, value}
	}
	t.store[key] = arr
}

func (t *TimeMap) get(key, time int) (int, bool) {
	arr, ok := t.store[key]
	if !ok {
		return 0, false
	}
	idx := sort.Search(len(arr), func(i int) bool { return arr[i].time > time }) // upper_bound
	if idx == 0 {
		return 0, false
	}
	return arr[idx-1].value, true
}

func show(v int, ok bool) string {
	if !ok {
		return "null"
	}
	return fmt.Sprintf("%d", v)
}

func main() {
	// README presents three logical blocks; each starts from its own map state.
	d := NewTimeMap()
	d.set(1, 1, 0)
	d.set(1, 2, 2)
	v, ok := d.get(1, 1)
	fmt.Println("d.get(1, 1) ->", show(v, ok))
	v, ok = d.get(1, 3)
	fmt.Println("d.get(1, 3) ->", show(v, ok))

	d = NewTimeMap()
	d.set(1, 1, 5)
	v, ok = d.get(1, 0)
	fmt.Println("d.get(1, 0) ->", show(v, ok))
	v, ok = d.get(1, 10)
	fmt.Println("d.get(1, 10) ->", show(v, ok))

	d = NewTimeMap()
	d.set(1, 1, 0)
	d.set(1, 2, 0) // overwrite same key+time -> value 2
	v, ok = d.get(1, 0)
	fmt.Println("d.get(1, 0) ->", show(v, ok))
}
