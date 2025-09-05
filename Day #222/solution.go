// Day 222: Normalize an absolute path (resolve . and ..).
// Approach: split on '/', push names onto a stack, pop on "..", skip "." and "". Time O(n), Space O(n).
package main

import (
	"fmt"
	"strings"
)

func simplifyPath(path string) string {
	st := []string{}
	for _, tok := range strings.Split(path, "/") {
		if tok == "" || tok == "." {
			continue
		}
		if tok == ".." {
			if len(st) > 0 {
				st = st[:len(st)-1]
			}
		} else {
			st = append(st, tok)
		}
	}
	if len(st) == 0 {
		return "/"
	}
	return "/" + strings.Join(st, "/") + "/" // trailing slash (directory form)
}

func main() {
	fmt.Printf("\"%s\"\n", simplifyPath("/usr/bin/../bin/./scripts/../")) // "/usr/bin/"
}
