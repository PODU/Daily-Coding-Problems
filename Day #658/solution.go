// Longest absolute file path: split on '\n', track pathLen by depth via slice. Time O(n), Space O(depth).
package main

import (
	"fmt"
	"strings"
)

func lengthLongestPath(input string) int {
	lines := strings.Split(input, "\n")
	pathLen := make([]int, len(lines)+1)
	maxLen := 0
	for _, line := range lines {
		depth := 0
		for depth < len(line) && line[depth] == '\t' {
			depth++
		}
		name := line[depth:]
		curLen := len(name)
		if depth > 0 {
			curLen += pathLen[depth-1] + 1
		}
		pathLen[depth] = curLen
		if strings.Contains(name, ".") {
			if curLen > maxLen {
				maxLen = curLen
			}
		}
	}
	return maxLen
}

func main() {
	input := "dir\n\tsubdir1\n\t\tfile1.ext\n\t\tsubsubdir1\n\tsubdir2\n\t\tsubsubdir2\n\t\t\tfile2.ext"
	fmt.Println(lengthLongestPath(input))
}
