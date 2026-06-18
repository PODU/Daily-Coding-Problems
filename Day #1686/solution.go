// Selection sort using only reverse(lst,i,j). For each i, find min in [i..n-1],
// reverse [i..m] to bring it to front. Time O(n^2), Space O(1).
package main

import (
	"fmt"
	"strings"
)

func reverseRange(lst []int, i, j int) {
	for i < j {
		lst[i], lst[j] = lst[j], lst[i]
		i++
		j--
	}
}

func sortWithReverse(lst []int) {
	n := len(lst)
	for i := 0; i < n; i++ {
		m := i
		for k := i + 1; k < n; k++ {
			if lst[k] < lst[m] {
				m = k
			}
		}
		reverseRange(lst, i, m)
	}
}

func main() {
	data := []int{3, 1, 2, 5, 4}
	sortWithReverse(data)
	parts := make([]string, len(data))
	for i, v := range data {
		parts[i] = fmt.Sprintf("%d", v)
	}
	fmt.Println(strings.Join(parts, " "))
}
