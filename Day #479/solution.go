// Generate all permutations via backtracking, picking remaining elements in
// index order so output is lexicographic. Time: O(n! * n), Space: O(n) recursion.
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
	var perms []string
	for _, p := range res {
		var parts []string
		for _, v := range p {
			parts = append(parts, strconv.Itoa(v))
		}
		perms = append(perms, "["+strings.Join(parts, ",")+"]")
	}
	fmt.Println("[" + strings.Join(perms, ",") + "]")
}
