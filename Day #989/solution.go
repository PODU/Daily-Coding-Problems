// Day 989: Deduce coin denominations from a ways-to-make-change array.
// Walk amounts; whenever target[i] exceeds reconstructed ways, i is a coin and we fold it into the DP.
// O(N^2) time, O(N) space.
package main

import (
	"fmt"
	"strconv"
	"strings"
)

func findDenominations(target []int64) []int {
	n := len(target)
	have := make([]int64, n)
	have[0] = 1 // one way to make 0 with no coins
	coins := []int{}
	for i := 1; i < n; i++ {
		if target[i] > have[i] { // unaccounted combinations => i is a denomination
			coins = append(coins, i)
			for j := i; j < n; j++ {
				have[j] += have[j-i]
			}
		}
	}
	return coins
}

func main() {
	target := []int64{1, 0, 1, 1, 2}
	coins := findDenominations(target)
	parts := make([]string, len(coins))
	for i, c := range coins {
		parts[i] = strconv.Itoa(c)
	}
	fmt.Println(strings.Join(parts, ", ")) // expected: 2, 3, 4
}
