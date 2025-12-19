// Day 762: Smallest distance (words in between) between two target words.
// Single pass tracking last seen index of each word. Time: O(n), Space: O(1).
package main

import (
	"fmt"
	"strings"
)

func smallestDistance(words []string, a, b string) int {
	lastA, lastB, bestGap := -1, -1, 1<<31-1
	for i, w := range words {
		if w == a {
			lastA = i
			if lastB != -1 && lastA-lastB < bestGap {
				bestGap = lastA - lastB
			}
		}
		if w == b {
			lastB = i
			if lastA != -1 && lastB-lastA < bestGap {
				bestGap = lastB - lastA
			}
		}
	}
	if bestGap == 1<<31-1 {
		return -1
	}
	return bestGap - 1
}

func main() {
	text := "dog cat hello cat dog dog hello cat world"
	fmt.Println(smallestDistance(strings.Fields(text), "hello", "world")) // 1
}
