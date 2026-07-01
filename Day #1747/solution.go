// Day 1747: All permutations of a list of digits.
// Approach: backtracking with a used[] mask, iterating values in order -> lexicographic.
// Time O(n * n!), space O(n) recursion (plus O(n!) for the output).
package main

import (
	"fmt"
	"strconv"
	"strings"
)

func permutations(nums []int) [][]int {
	var res [][]int
	used := make([]bool, len(nums))
	var cur []int

	var backtrack func()
	backtrack = func() {
		if len(cur) == len(nums) {
			cp := make([]int, len(cur))
			copy(cp, cur)
			res = append(res, cp)
			return
		}
		for i := 0; i < len(nums); i++ {
			if used[i] {
				continue
			}
			used[i] = true
			cur = append(cur, nums[i])
			backtrack()
			cur = cur[:len(cur)-1]
			used[i] = false
		}
	}
	backtrack()
	return res
}

func main() {
	nums := []int{1, 2, 3}
	res := permutations(nums)
	parts := make([]string, len(res))
	for i, p := range res {
		strs := make([]string, len(p))
		for j, v := range p {
			strs[j] = strconv.Itoa(v)
		}
		parts[i] = "[" + strings.Join(strs, ",") + "]"
	}
	fmt.Println("[" + strings.Join(parts, ",") + "]")
}
