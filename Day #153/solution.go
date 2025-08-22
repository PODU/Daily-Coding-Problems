// Day 153: Min words separating two words. Single pass tracking last seen index
// of each word; answer is min(|i-j|-1). Time O(n), Space O(1).
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

func minDistance(words []string, a, b string) int {
	lastA, lastB := -1, -1
	best := int(^uint(0) >> 1)
	for i, w := range words {
		if w == a {
			lastA = i
			if lastB != -1 && abs(lastA-lastB)-1 < best {
				best = abs(lastA-lastB) - 1
			}
		}
		if w == b {
			lastB = i
			if lastA != -1 && abs(lastA-lastB)-1 < best {
				best = abs(lastA-lastB) - 1
			}
		}
	}
	return best
}

func main() {
	text := "dog cat hello cat dog dog hello cat world"
	words := strings.Fields(text)
	fmt.Println(minDistance(words, "hello", "world"))
}
