// Day 1185: Smallest distance (words in between) between two words in a text.
// Single pass tracking last index of each target word; min |i-j| - 1.
// Time O(N), Space O(1).
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

func shortestDistance(text, w1, w2 string) int {
	words := strings.Fields(text)
	p1, p2, best := -1, -1, 1<<31-1
	for i, t := range words {
		if t == w1 {
			p1 = i
		}
		if t == w2 {
			p2 = i
		}
		if p1 >= 0 && p2 >= 0 {
			if d := abs(p1 - p2); d < best {
				best = d
			}
		}
	}
	if best == 1<<31-1 {
		return -1
	}
	return best - 1
}

func main() {
	text := "dog cat hello cat dog dog hello cat world"
	fmt.Println(shortestDistance(text, "hello", "world")) // 1
}
