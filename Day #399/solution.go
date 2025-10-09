// Partition into 3 contiguous equal-sum parts: greedy prefix cut at target, absorbing trailing zeros. O(n) time, O(n) space.
package main

import (
	"fmt"
	"strconv"
	"strings"
)

func partition3(L []int) ([][]int, bool) {
	total := 0
	for _, x := range L {
		total += x
	}
	if total%3 != 0 {
		return nil, false
	}
	target := total / 3
	var res [][]int
	var cur []int
	running := 0
	for i := 0; i < len(L); i++ {
		cur = append(cur, L[i])
		running += L[i]
		// close part when sum hits target and next element is non-zero (zeros stay attached)
		if len(res) < 2 && running == target && (i+1 == len(L) || L[i+1] != 0) {
			res = append(res, cur)
			cur = nil
			running = 0
		}
	}
	res = append(res, cur)
	if len(res) != 3 {
		return nil, false
	}
	for _, p := range res {
		s := 0
		for _, x := range p {
			s += x
		}
		if s != target {
			return nil, false
		}
	}
	return res, true
}

func format(parts [][]int, ok bool) string {
	if !ok {
		return "None"
	}
	outer := make([]string, len(parts))
	for i, p := range parts {
		nums := make([]string, len(p))
		for j, x := range p {
			nums[j] = strconv.Itoa(x)
		}
		outer[i] = "[" + strings.Join(nums, ", ") + "]"
	}
	return "[" + strings.Join(outer, ", ") + "]"
}

func main() {
	L := []int{3, 5, 8, 0, 8}
	parts, ok := partition3(L)
	fmt.Println(format(parts, ok))
}
