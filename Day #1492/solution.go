// Minimum meeting rooms for overlapping intervals.
// Sort starts and ends, two-pointer sweep. Time O(n log n), Space O(n).
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
	rooms, maxRooms, j := 0, 0, 0
	for i := 0; i < n; i++ {
		for j < n && ends[j] <= starts[i] {
			rooms--
			j++
		}
		rooms++
		if rooms > maxRooms {
			maxRooms = rooms
		}
	}
	return maxRooms
}

func main() {
	intervals := [][2]int{{30, 75}, {0, 50}, {60, 150}}
	fmt.Println(minRooms(intervals)) // expected: 2
}
