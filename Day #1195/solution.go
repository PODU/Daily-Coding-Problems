// Smallest window containing all distinct chars of the string. Sliding window:
// expand right, shrink left while all distinct kinds present. O(N) time, O(K) space.
package main

import "fmt"

func smallestWindow(s string) int {
	distinctSet := make(map[byte]bool)
	for i := 0; i < len(s); i++ {
		distinctSet[s[i]] = true
	}
	distinct := len(distinctSet)

	cnt := make(map[byte]int)
	have, left := 0, 0
	best := len(s) + 1
	for right := 0; right < len(s); right++ {
		cnt[s[right]]++
		if cnt[s[right]] == 1 {
			have++
		}
		for have == distinct {
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
	if best == len(s)+1 {
		return 0
	}
	return best
}

func main() {
	fmt.Println(smallestWindow("jiujitsu"))
}
