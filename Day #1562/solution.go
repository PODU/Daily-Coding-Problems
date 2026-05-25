// Sort mice and holes, pair by index, answer = max |mice[i]-holes[i]|. Time O(n log n), Space O(1).
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

func main() {
	mice := []int{1, 4, 9, 15}
	holes := []int{10, -5, 0, 16}
	sort.Ints(mice)
	sort.Ints(holes)
	ans := 0
	for i := range mice {
		if d := abs(mice[i] - holes[i]); d > ans {
			ans = d
		}
	}
	fmt.Println(ans)
}
