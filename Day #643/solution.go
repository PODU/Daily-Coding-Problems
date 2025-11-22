// Day 643: Longest common subsequence of three strings.
// Approach: 3D DP over prefix lengths of a, b, c.
// Time: O(|a|*|b|*|c|), Space: O(|b|*|c|) (two rolling layers).
package main

import "fmt"

func max3(a, b, c int) int {
	if a < b {
		a = b
	}
	if a < c {
		a = c
	}
	return a
}

func lcs3(a, b, c string) int {
	B, C := len(b), len(c)
	prev := make([][]int, B+1)
	for i := range prev {
		prev[i] = make([]int, C+1)
	}
	for i := 1; i <= len(a); i++ {
		cur := make([][]int, B+1)
		for x := range cur {
			cur[x] = make([]int, C+1)
		}
		for j := 1; j <= B; j++ {
			for k := 1; k <= C; k++ {
				if a[i-1] == b[j-1] && b[j-1] == c[k-1] {
					cur[j][k] = prev[j-1][k-1] + 1
				} else {
					cur[j][k] = max3(prev[j][k], cur[j-1][k], cur[j][k-1])
				}
			}
		}
		prev = cur
	}
	return prev[B][C]
}

func main() {
	fmt.Println(lcs3("epidemiologist", "refrigeration",
		"supercalifragilisticexpialodocious")) // 5
}
