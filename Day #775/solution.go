// Day 775: Minimum meeting rooms = max overlapping intervals.
// Sort starts and ends, two-pointer sweep. O(n log n) time, O(n) space.
package main

import (
	"fmt"
	"sort"
)

func minRooms(intervals [][2]int) int {
	n := len(intervals)
	starts := make([]int, n)
	ends := make([]int, n)
	for i, iv := range intervals {
		starts[i] = iv[0]
		ends[i] = iv[1]
	}
	sort.Ints(starts)
	sort.Ints(ends)
	rooms, best, i, j := 0, 0, 0, 0
	for i < n {
		if starts[i] < ends[j] {
			rooms++
			i++
			if rooms > best {
				best = rooms
			}
		} else {
			rooms--
			j++
		}
	}
	return best
}

func main() {
	fmt.Println(minRooms([][2]int{{30, 75}, {0, 50}, {60, 150}})) // 2
}
