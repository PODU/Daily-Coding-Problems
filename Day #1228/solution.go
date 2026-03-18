// Merge overlapping intervals: sort by start, sweep merging when start <= last end.
// Time: O(n log n), Space: O(n).
package main

import (
	"fmt"
	"sort"
	"strings"
)

func merge(intervals [][2]int) [][2]int {
	sort.Slice(intervals, func(i, j int) bool { return intervals[i][0] < intervals[j][0] })
	res := [][2]int{}
	for _, iv := range intervals {
		if len(res) > 0 && iv[0] <= res[len(res)-1][1] {
			if iv[1] > res[len(res)-1][1] {
				res[len(res)-1][1] = iv[1]
			}
		} else {
			res = append(res, iv)
		}
	}
	return res
}

func main() {
	data := [][2]int{{1, 3}, {5, 8}, {4, 10}, {20, 25}}
	out := merge(data)
	parts := make([]string, len(out))
	for i, iv := range out {
		parts[i] = fmt.Sprintf("(%d, %d)", iv[0], iv[1])
	}
	fmt.Println("[" + strings.Join(parts, ", ") + "]")
}
