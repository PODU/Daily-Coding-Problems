// Time-versioned map: per key keep times sorted; get returns value at most-recent time <= t.
// set amortized O(log m), get O(log m) via binary search. Space: O(total entries).
package main

import (
	"fmt"
	"sort"
)

type entry struct{ time, value int }

type TimeMap struct {
	data map[int][]entry
}

func NewTimeMap() *TimeMap { return &TimeMap{data: map[int][]entry{}} }

func (t *TimeMap) Set(key, value, time int) {
	arr := t.data[key]
	i := sort.Search(len(arr), func(j int) bool { return arr[j].time >= time })
	if i < len(arr) && arr[i].time == time {
		arr[i].value = value
	} else {
		arr = append(arr, entry{})
		copy(arr[i+1:], arr[i:])
		arr[i] = entry{time, value}
	}
	t.data[key] = arr
}

// returns value, found
func (t *TimeMap) Get(key, time int) (int, bool) {
	arr, ok := t.data[key]
	if !ok {
		return 0, false
	}
	i := sort.Search(len(arr), func(j int) bool { return arr[j].time > time })
	if i == 0 {
		return 0, false
	}
	return arr[i-1].value, true
}

func show(d *TimeMap, key, time int) {
	if v, ok := d.Get(key, time); ok {
		fmt.Printf("get(%d, %d) = %d\n", key, time, v)
	} else {
		fmt.Printf("get(%d, %d) = null\n", key, time)
	}
}

func main() {
	d := NewTimeMap(); d.Set(1, 1, 0); d.Set(1, 2, 2); show(d, 1, 1); show(d, 1, 3) // 1, 2
	d = NewTimeMap(); d.Set(1, 1, 5); show(d, 1, 0); show(d, 1, 10)                  // null, 1
	d = NewTimeMap(); d.Set(1, 1, 0); d.Set(1, 2, 0); show(d, 1, 0)                  // 2
}
