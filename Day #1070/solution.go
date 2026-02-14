// Sliding window with count map: longest subarray with at most 2 distinct values. O(n) time, O(1) space.
package main

import "fmt"

func longestAtMost2Distinct(nums []int) int {
	cnt := make(map[int]int)
	left, best := 0, 0
	for right := 0; right < len(nums); right++ {
		cnt[nums[right]]++
		for len(cnt) > 2 {
			cnt[nums[left]]--
			if cnt[nums[left]] == 0 {
				delete(cnt, nums[left])
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
	nums := []int{2, 1, 2, 3, 3, 1, 3, 5}
	fmt.Println(longestAtMost2Distinct(nums))
}
