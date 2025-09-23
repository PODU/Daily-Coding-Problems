// Smallest window containing all distinct chars: O(n) sliding window.
// Time O(n), Space O(alphabet).
package main

import "fmt"

func smallestWindow(s string) int {
	distinct := map[byte]bool{}
	for i := 0; i < len(s); i++ {
		distinct[s[i]] = true
	}
	need := len(distinct)
	cnt := map[byte]int{}
	have, left, best := 0, 0, len(s)+1
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
