// Merge overlapping intervals: sort by start, merge when next.start <= current.end.
// Time: O(n log n), Space: O(n).
package main

import (
	"fmt"
	"sort"
	"strings"
)

func main() {
	iv := [][2]int{{1, 3}, {5, 8}, {4, 10}, {20, 25}}
	sort.Slice(iv, func(i, j int) bool { return iv[i][0] < iv[j][0] })
	var res [][2]int
	for _, p := range iv {
		if len(res) > 0 && p[0] <= res[len(res)-1][1] {
			if p[1] > res[len(res)-1][1] {
				res[len(res)-1][1] = p[1]
			}
		} else {
			res = append(res, p)
		}
	}
	parts := make([]string, len(res))
	for i, p := range res {
		parts[i] = fmt.Sprintf("(%d, %d)", p[0], p[1])
	}
	fmt.Println("[" + strings.Join(parts, ", ") + "]")
}
