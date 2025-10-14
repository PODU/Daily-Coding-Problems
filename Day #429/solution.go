// Day 429: kth row of Pascal's triangle (0-indexed: row 0 = [1]).
// Multiplicative recurrence row[j] = row[j-1]*(k-j+1)/j. Time O(k), Space O(k).
package main

import (
	"fmt"
	"strconv"
	"strings"
)

func main() {
	k := 4
	row := make([]int, k+1)
	row[0] = 1
	for j := 1; j <= k; j++ {
		row[j] = row[j-1] * (k - j + 1) / j
	}
	parts := make([]string, k+1)
	for j := 0; j <= k; j++ {
		parts[j] = strconv.Itoa(row[j])
	}
	fmt.Println(strings.Join(parts, " "))
}
