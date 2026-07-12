// Longest contiguous subarray with at most 2 distinct values via sliding window + hashmap. O(n) time, O(1) space.
package main

import "fmt"

func longestTwoDistinct(a []int) int {
	cnt := make(map[int]int)
	left, best := 0, 0
	for right := 0; right < len(a); right++ {
		cnt[a[right]]++
		for len(cnt) > 2 {
			cnt[a[left]]--
			if cnt[a[left]] == 0 {
				delete(cnt, a[left])
			}
			left++
		}
		if right-left+1 > best {
			best = right - left + 1
		}
	}
	return best
}

func main() {
	a := []int{2, 1, 2, 3, 3, 1, 3, 5}
	fmt.Println(longestTwoDistinct(a)) // 4
}
