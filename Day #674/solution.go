// Day 674: Longest contiguous subarray with at most 2 distinct values. Sliding window.
// Time O(n), Space O(1) (at most 3 keys in the map).
package main

import "fmt"

func longestTwoTypes(a []int) int {
	cnt := map[int]int{}
	best, l := 0, 0
	for r := 0; r < len(a); r++ {
		cnt[a[r]]++
		for len(cnt) > 2 {
			cnt[a[l]]--
			if cnt[a[l]] == 0 {
				delete(cnt, a[l])
			}
			l++
		}
		if r-l+1 > best {
			best = r - l + 1
		}
	}
	return best
}

func main() {
	fmt.Println(longestTwoTypes([]int{2, 1, 2, 3, 3, 1, 3, 5})) // 4
}
