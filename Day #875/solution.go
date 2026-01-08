// Longest absolute file path: track path length per depth in one pass.
// Time O(n), Space O(d) for depth d.
package main

import (
	"fmt"
	"strings"
)

func lengthLongestPath(input string) int {
	pathLen := map[int]int{0: 0}
	maxLen := 0
	for _, line := range strings.Split(input, "\n") {
		depth := 0
		for depth < len(line) && line[depth] == '\t' {
			depth++
		}
		name := line[depth:]
		if strings.Contains(name, ".") {
			if pathLen[depth]+len(name) > maxLen {
				maxLen = pathLen[depth] + len(name)
			}
		} else {
			pathLen[depth+1] = pathLen[depth] + len(name) + 1
		}
	}
	return maxLen
}

func main() {
	input := "dir\n\tsubdir1\n\t\tfile1.ext\n\t\tsubsubdir1\n\tsubdir2\n\t\tsubsubdir2\n\t\t\tfile2.ext"
	fmt.Println(lengthLongestPath(input))
}
