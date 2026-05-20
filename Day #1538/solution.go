// Minimum rooms for overlapping intervals: sort starts & ends, sweep with two pointers.
// Time O(n log n), Space O(n).
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
		starts[i], ends[i] = iv[0], iv[1]
	}
	sort.Ints(starts)
	sort.Ints(ends)
	rooms, max, j := 0, 0, 0
	for i := 0; i < n; i++ {
		for j < n && ends[j] <= starts[i] {
			rooms--
			j++
		}
		rooms++
		if rooms > max {
			max = rooms
		}
	}
	return max
}

func main() {
	fmt.Println(minRooms([][2]int{{30, 75}, {0, 50}, {60, 150}}))
}
