// Min word distance: single pass tracking last-seen index of each word; on each
// hit, update min with |i-j|-1 (words strictly between). Time O(n), Space O(1).
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

func minWordDistance(text []string, w1, w2 string) int {
	last1, last2 := -1, -1
	best := int(^uint(0) >> 1)
	for i, word := range text {
		if word == w1 {
			last1 = i
			if last2 != -1 && abs(last1-last2)-1 < best {
				best = abs(last1-last2) - 1
			}
		}
		if word == w2 {
			last2 = i
			if last1 != -1 && abs(last1-last2)-1 < best {
				best = abs(last1-last2) - 1
			}
		}
	}
	return best
}

func main() {
	text := strings.Fields("dog cat hello cat dog dog hello cat world")
	fmt.Println(minWordDistance(text, "hello", "world"))
}
