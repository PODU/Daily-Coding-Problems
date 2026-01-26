// Day 961: Normalize an absolute Unix path resolving "." and "..".
// Approach: split by '/', use a stack. Time O(n), Space O(n).
package main

import (
	"fmt"
	"strings"
)

func simplifyPath(path string) string {
	var stack []string
	for _, seg := range strings.Split(path, "/") {
		if seg == "" || seg == "." {
			continue
		}
		if seg == ".." {
			if len(stack) > 0 {
				stack = stack[:len(stack)-1]
			}
		} else {
			stack = append(stack, seg)
		}
	}
	res := "/" + strings.Join(stack, "/")
	if len(path) > 0 && path[len(path)-1] == '/' && res != "/" {
		res += "/"
	}
	return res
}

func main() {
	fmt.Printf("\"%s\"\n", simplifyPath("/usr/bin/../bin/./scripts/../"))
}
