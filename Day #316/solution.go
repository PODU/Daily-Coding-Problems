// Reconstruct coin denominations from a ways-to-make-change array.
// DP coin detection: A[i] > ways[i] means i is a coin. Time O(N^2), Space O(N).
package main

import (
	"fmt"
	"strconv"
	"strings"
)

func joinCoins(c []int) string {
	if len(c) == 0 {
		return ""
	}
	if len(c) == 1 {
		return strconv.Itoa(c[0])
	}
	if len(c) == 2 {
		return fmt.Sprintf("%d and %d", c[0], c[1])
	}
	parts := make([]string, 0, len(c)-1)
	for i := 0; i+1 < len(c); i++ {
		parts = append(parts, strconv.Itoa(c[i]))
	}
	return strings.Join(parts, ", ") + ", and " + strconv.Itoa(c[len(c)-1])
}

func findCoins(A []int) []int {
	n := len(A)
	ways := make([]int64, n)
	ways[0] = 1
	coins := []int{}
	for i := 1; i < n; i++ {
		if int64(A[i]) > ways[i] {
			coins = append(coins, i)
			for j := i; j < n; j++ {
				ways[j] += ways[j-i]
			}
		}
	}
	return coins
}

func main() {
	A := []int{1, 0, 1, 1, 2}
	fmt.Println(joinCoins(findCoins(A)))
}
