// Smallest window containing every distinct character of the string.
// Sliding window with frequency counts; expand then shrink. O(n) time, O(k) space.
package main

import "fmt"

func smallestWindow(s string) int {
	distinct := make(map[byte]bool)
	for i := 0; i < len(s); i++ {
		distinct[s[i]] = true
	}
	need := len(distinct)
	cnt := make(map[byte]int)
	have, best, left := 0, 1<<31-1, 0
	for right := 0; right < len(s); right++ {
		cnt[s[right]]++
		if cnt[s[right]] == 1 {
			have++
		}
		for have == need {
			if right-left+1 < best {
				best = right - left + 1
			}
			cnt[s[left]]--
			if cnt[s[left]] == 0 {
				have--
			}
			left++
		}
	}
	return best
}

func main() {
	fmt.Println(smallestWindow("jiujitsu"))
}
