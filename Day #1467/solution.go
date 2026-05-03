// All permutations via backtracking, picking remaining elements left-to-right (lexicographic order).
// Time O(n! * n), Space O(n) recursion + output.
package main

import (
	"fmt"
	"strconv"
	"strings"
)

func permute(nums []int) [][]int {
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
	res := permute([]int{1, 2, 3})
	parts := make([]string, len(res))
	for i, p := range res {
		nums := make([]string, len(p))
		for j, x := range p {
			nums[j] = strconv.Itoa(x)
		}
		parts[i] = "[" + strings.Join(nums, ",") + "]"
	}
	fmt.Println("[" + strings.Join(parts, ",") + "]")
}
