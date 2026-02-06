// Day 1028: Kaprekar's routine. Repeatedly subtract ascending- from descending-
// digit arrangement until 6174; count steps. Time O(steps), Space O(1).
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
		for i, j := 0, len(s)-1; i < j; i, j = i+1, j-1 {
			s[i], s[j] = s[j], s[i]
		}
		desc, _ := strconv.Atoi(string(s))
		n = desc - asc
		steps++
	}
	return steps
}

func main() {
	fmt.Println(kaprekarSteps(1234))
}
