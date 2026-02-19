// Day 1101: Assign mice to holes minimizing the maximum travel distance.
// Sort both, match in order; answer is max |mice[i]-holes[i]|.
// Time: O(N log N). Space: O(1).
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
	fmt.Println(minLastMouse([]int{1, 4, 9, 15}, []int{10, -5, 0, 16})) // 6
}
