// Longest absolute file path. Parse tab-indented FS, track cumulative path
// length per depth. Time O(n), Space O(depth).
package main

import (
	"fmt"
	"strings"
)

func lengthLongestPath(input string) int {
	pathLen := map[int]int{0: 0}
	best := 0
	for _, line := range strings.Split(input, "\n") {
		level := 0
		for level < len(line) && line[level] == '\t' {
			level++
		}
		name := line[level:]
		if strings.Contains(name, ".") {
			if v := pathLen[level] + len(name); v > best {
				best = v
			}
		} else {
			pathLen[level+1] = pathLen[level] + len(name) + 1
		}
	}
	return best
}

func main() {
	s := "dir\n\tsubdir1\n\t\tfile1.ext\n\t\tsubsubdir1\n\tsubdir2\n\t\tsubsubdir2\n\t\t\tfile2.ext"
	fmt.Println(lengthLongestPath(s)) // 32
}
