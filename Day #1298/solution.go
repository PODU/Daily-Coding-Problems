// Day 1298: Count Kaprekar steps to reach 6174.
// Repeatedly sort digits desc - asc until 6174. Converges in <=7 steps. O(steps) time.
package main

import (
	"fmt"
	"sort"
	"strconv"
)

func kaprekarSteps(n int) int {
	steps := 0
	for n != 6174 {
		s := []byte(fmt.Sprintf("%04d", n))
		sort.Slice(s, func(i, j int) bool { return s[i] < s[j] })
		asc, _ := strconv.Atoi(string(s))
		// reverse for desc
		for i, j := 0, len(s)-1; i < j; i, j = i+1, j-1 {
			s[i], s[j] = s[j], s[i]
		}
		desc, _ := strconv.Atoi(string(s))
		n = desc - asc
		steps++
		if n == 0 {
			break
		}
	}
	return steps
}

func main() {
	fmt.Println(kaprekarSteps(1234)) // 3
}
