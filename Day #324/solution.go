// Assign mice to holes minimizing max distance: sort both, pair i-th, answer = max|mice[i]-holes[i]|.
// Time: O(N log N), Space: O(1) extra.
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

func minMaxDistance(mice, holes []int) int {
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
	mice := []int{1, 4, 9, 15}
	holes := []int{10, -5, 0, 16}
	fmt.Println(minMaxDistance(mice, holes))
}
