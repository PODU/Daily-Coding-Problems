// Three-way stable partition around pivot x: collect <x, ==x, >x in order, concat.
// Time O(n), Space O(n).
package main

import (
	"fmt"
	"strings"
)

func partition3(x int, lst []int) []int {
	var less, equal, greater []int
	for _, v := range lst {
		if v < x {
			less = append(less, v)
		} else if v == x {
			equal = append(equal, v)
		} else {
			greater = append(greater, v)
		}
	}
	res := append(append(less, equal...), greater...)
	return res
}

func main() {
	res := partition3(10, []int{9, 12, 3, 5, 14, 10, 10})
	parts := make([]string, len(res))
	for i, v := range res {
		parts[i] = fmt.Sprintf("%d", v)
	}
	fmt.Println("[" + strings.Join(parts, ", ") + "]")
}
