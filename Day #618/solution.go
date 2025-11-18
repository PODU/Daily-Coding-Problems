// Pancake sort using only reverse(lst,i,j): flip current max to front, then to its place.
// Time: O(n^2) comparisons, O(n) flips, Space: O(1).
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
	for size := len(lst); size > 1; size-- {
		maxIdx := 0
		for k := 1; k < size; k++ {
			if lst[k] > lst[maxIdx] {
				maxIdx = k
			}
		}
		if maxIdx != size-1 {
			if maxIdx != 0 {
				reverse(lst, 0, maxIdx)
			}
			reverse(lst, 0, size-1)
		}
	}
}

func main() {
	lst := []int{3, 1, 4, 1, 5, 9, 2, 6}
	pancakeSort(lst)
	parts := make([]string, len(lst))
	for i, v := range lst {
		parts[i] = strconv.Itoa(v)
	}
	fmt.Printf("[%s]\n", strings.Join(parts, ", "))
}
