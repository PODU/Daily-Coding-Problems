// Approach: Single pass, track running path length per depth via a map/stack. O(n) time, O(d) space.
package main

import (
	"fmt"
	"strings"
)

func lengthLongestPath(input string) int {
	lenAtDepth := map[int]int{-1: 0}
	maxLen := 0
	for _, line := range strings.Split(input, "\n") {
		depth := 0
		for depth < len(line) && line[depth] == '\t' {
			depth++
		}
		name := line[depth:]
		isFile := strings.Contains(name, ".")
		curLen := lenAtDepth[depth-1] + len(name)
		if isFile {
			if curLen > maxLen {
				maxLen = curLen
			}
		} else {
			lenAtDepth[depth] = curLen + 1 // +1 for '/'
		}
	}
	return maxLen
}

func main() {
	input := "dir\n\tsubdir1\n\t\tfile1.ext\n\t\tsubsubdir1\n\tsubdir2\n\t\tsubsubdir2\n\t\t\tfile2.ext"
	fmt.Println(lengthLongestPath(input))
}
