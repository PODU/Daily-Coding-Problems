// Pancake sort using only reverse(lst,i,j): for each end, bring max of prefix to front then flip to its spot.
// O(n^2) comparisons, O(n) reversals, in place. Space O(1).
package main

import (
	"fmt"
	"strconv"
	"strings"
)

func reverse(lst []int, i, j int) {
	for i < j {
		lst[i], lst[j] = lst[j], lst[i]
		i++
		j--
	}
}

func pancakeSort(lst []int) {
	n := len(lst)
	for end := n - 1; end > 0; end-- {
		mi := 0
		for k := 1; k <= end; k++ {
			if lst[k] > lst[mi] {
				mi = k
			}
		}
		if mi == end {
			continue
		}
		if mi != 0 {
			reverse(lst, 0, mi) // bring max to front
		}
		reverse(lst, 0, end) // move max to its final position
	}
}

func main() {
	arr := []int{3, 1, 4, 1, 5, 9, 2, 6}
	pancakeSort(arr)
	parts := make([]string, len(arr))
	for i, v := range arr {
		parts[i] = strconv.Itoa(v)
	}
	fmt.Println("[" + strings.Join(parts, ", ") + "]")
}
