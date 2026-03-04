// Day 1152: Simplify absolute Unix path.
// Stack of components; '.' ignored, '..' pops. Time O(n), Space O(n).
package main

import (
	"fmt"
	"strings"
)

func simplify(path string) string {
	var st []string
	for _, part := range strings.Split(path, "/") {
		if part == "" || part == "." {
			continue
		}
		if part == ".." {
			if len(st) > 0 {
				st = st[:len(st)-1]
			}
		} else {
			st = append(st, part)
		}
	}
	var sb strings.Builder
	sb.WriteByte('/')
	for _, p := range st {
		sb.WriteString(p)
		sb.WriteByte('/')
	}
	return sb.String() // trailing slash preserved
}

func main() {
	fmt.Println(simplify("/usr/bin/../bin/./scripts/../")) // /usr/bin/
}
