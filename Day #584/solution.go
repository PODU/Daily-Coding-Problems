// Greedy rearrange: at each step pick highest remaining count != prev char, tie by smallest char.
// Feasible iff maxCount <= (n+1)/2. Time O(n*26), Space O(26).
package main

import "fmt"

func rearrange(s string) (string, bool) {
	var cnt [26]int
	for _, c := range s {
		cnt[c-'a']++
	}
	n := len(s)
	res := make([]byte, 0, n)
	prev := -1
	for k := 0; k < n; k++ {
		best := -1
		for i := 0; i < 26; i++ {
			if i == prev || cnt[i] <= 0 {
				continue
			}
			if best == -1 || cnt[i] > cnt[best] {
				best = i
			}
		}
		if best == -1 {
			return "", false
		}
		res = append(res, byte('a'+best))
		cnt[best]--
		prev = best
	}
	return string(res), true
}

func main() {
	if a, ok := rearrange("aaabbc"); ok {
		fmt.Println(a)
	} else {
		fmt.Println("None")
	}
	if b, ok := rearrange("aaab"); ok {
		fmt.Println(b)
	} else {
		fmt.Println("None")
	}
}
