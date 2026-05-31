// Smallest window containing every distinct char: sliding window with counts.
// Time O(n), Space O(k).
package main

import "fmt"

func smallestWindow(s string) int {
	distinct := map[byte]bool{}
	for i := 0; i < len(s); i++ {
		distinct[s[i]] = true
	}
	need := len(distinct)
	cnt := map[byte]int{}
	formed, left, best := 0, 0, len(s)
	for right := 0; right < len(s); right++ {
		cnt[s[right]]++
		if cnt[s[right]] == 1 {
			formed++
		}
		for formed == need {
			if right-left+1 < best {
				best = right - left + 1
			}
			cnt[s[left]]--
			if cnt[s[left]] == 0 {
				formed--
			}
			left++
		}
	}
	return best
}

func main() {
	fmt.Println(smallestWindow("jiujitsu"))
}
