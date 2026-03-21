// Boats: sort, greedily pair lightest+heaviest (two-pointer). Time O(n log n).
package main

import (
	"fmt"
	"sort"
)

func numBoats(weights []int, k int) int {
	w := append([]int(nil), weights...)
	sort.Ints(w)
	i, j, boats := 0, len(w)-1, 0
	for i <= j {
		if w[i]+w[j] <= k {
			i++
		}
		j--
		boats++
	}
	return boats
}

func main() {
	fmt.Println(numBoats([]int{100, 200, 150, 80}, 200))
}
