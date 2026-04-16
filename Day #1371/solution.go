// Assign mice to holes minimizing max distance: sort both, pair in order.
// Time O(n log n), Space O(1) extra.
package main

import (
	"fmt"
	"sort"
)

func abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}

func minLastMouse(mice, holes []int) int {
	sort.Ints(mice)
	sort.Ints(holes)
	ans := 0
	for i := range mice {
		if d := abs(mice[i] - holes[i]); d > ans {
			ans = d
		}
	}
	return ans
}

func main() {
	fmt.Println(minLastMouse([]int{1, 4, 9, 15}, []int{10, -5, 0, 16}))
}
