// Kaprekar's routine: repeatedly subtract ascending-digit from descending-digit
// (4-digit, zero-padded) until 6174; count steps. Time: O(7) iters, Space: O(1).
package main

import (
	"fmt"
	"sort"
	"strconv"
)

func kaprekar(n int) int {
	steps := 0
	for n != 6174 {
		s := fmt.Sprintf("%04d", n)
		b := []byte(s)
		sort.Slice(b, func(i, j int) bool { return b[i] < b[j] })
		asc, _ := strconv.Atoi(string(b))
		sort.Slice(b, func(i, j int) bool { return b[i] > b[j] })
		desc, _ := strconv.Atoi(string(b))
		n = desc - asc
		steps++
	}
	return steps
}

func main() {
	fmt.Println(kaprekar(1234))
}
