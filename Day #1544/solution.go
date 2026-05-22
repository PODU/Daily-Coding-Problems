// Longest absolute path to a file in a tab-indented filesystem string.
// Track cumulative path length per depth; a name with '.' is a file.
// Time O(n), Space O(depth).
package main

import (
	"fmt"
	"strings"
)

func longestPath(s string) int {
	lens := map[int]int{0: 0}
	maxLen := 0
	for _, line := range strings.Split(s, "\n") {
		depth := 0
		for depth < len(line) && line[depth] == '\t' {
			depth++
		}
		name := line[depth:]
		if strings.Contains(name, ".") {
			if lens[depth]+len(name) > maxLen {
				maxLen = lens[depth] + len(name)
			}
		} else {
			lens[depth+1] = lens[depth] + len(name) + 1 // +1 for '/'
		}
	}
	return maxLen
}

func main() {
	s := "dir\n\tsubdir1\n\t\tfile1.ext\n\t\tsubsubdir1\n\tsubdir2\n\t\tsubsubdir2\n\t\t\tfile2.ext"
	fmt.Println(longestPath(s))
}
