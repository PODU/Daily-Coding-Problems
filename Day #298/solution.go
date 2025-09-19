// Longest contiguous subarray with at most 2 distinct values via sliding window + count map.
// Time: O(n), Space: O(1) (at most 3 keys in map).
package main

import "fmt"

func longestAtMost2(a []int) int {
	cnt := map[int]int{}
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
	fmt.Println(longestAtMost2(a))
}
