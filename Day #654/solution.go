// Smallest window containing every distinct char: sliding window with two pointers.
// Expand right until all distinct chars present, shrink left to minimize. Time O(n), space O(k).
package main

import "fmt"

func smallestWindow(s string) int {
	distinct := make(map[byte]bool)
	for i := 0; i < len(s); i++ {
		distinct[s[i]] = true
	}
	need := len(distinct)
	cnt := make(map[byte]int)
	have, best, left := 0, -1, 0
	for right := 0; right < len(s); right++ {
		cnt[s[right]]++
		if cnt[s[right]] == 1 {
			have++
		}
		for have == need {
			if best == -1 || right-left+1 < best {
				best = right - left + 1
			}
			cnt[s[left]]--
			if cnt[s[left]] == 0 {
				have--
			}
			left++
		}
	}
	if best == -1 {
		return 0
	}
	return best
}

func main() {
	fmt.Println(smallestWindow("jiujitsu"))
}
