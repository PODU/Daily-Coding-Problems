// All permutations of a list of digits in lexicographic order.
// Backtracking over sorted digits. O(n!*n) time, O(n) extra space.
package main

import (
	"fmt"
	"sort"
	"strconv"
	"strings"
)

func permutations(digits []int) [][]int {
	sort.Ints(digits)
	n := len(digits)
	used := make([]bool, n)
	cur := []int{}
	res := [][]int{}
	var backtrack func()
	backtrack = func() {
		if len(cur) == n {
			cp := make([]int, n)
			copy(cp, cur)
			res = append(res, cp)
			return
		}
		for i := 0; i < n; i++ {
			if used[i] {
				continue
			}
			used[i] = true
			cur = append(cur, digits[i])
			backtrack()
			cur = cur[:len(cur)-1]
			used[i] = false
		}
	}
	backtrack()
	return res
}

func main() {
	res := permutations([]int{1, 2, 3})
	inner := make([]string, len(res))
	for i, p := range res {
		parts := make([]string, len(p))
		for j, x := range p {
			parts[j] = strconv.Itoa(x)
		}
		inner[i] = "[" + strings.Join(parts, ",") + "]"
	}
	fmt.Println("[" + strings.Join(inner, ",") + "]")
}
