// Day 853: smallest distance (number of words between) between two words in a text.
// Single pass tracking last index of each word. distance = |i-j| - 1. O(n) time, O(1) space.
package main

import (
	"fmt"
	"strings"
)

func abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}

func minDistance(text, w1, w2 string) int {
	words := strings.Fields(text)
	p1, p2, best := -1, -1, 1<<31-1
	for i, w := range words {
		if w == w1 {
			p1 = i
		}
		if w == w2 {
			p2 = i
		}
		if p1 != -1 && p2 != -1 {
			if d := abs(p1-p2) - 1; d < best {
				best = d
			}
		}
	}
	return best
}

func main() {
	text := "dog cat hello cat dog dog hello cat world"
	fmt.Println(minDistance(text, "hello", "world")) // 1
}
