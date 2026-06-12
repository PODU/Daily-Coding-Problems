// Simplify Unix absolute path: split on '/', push names on a stack, skip ''/'.', pop on '..'. Time O(n), Space O(n).
// Build "/a/b" from the stack; if input ended with '/' and result isn't root, append a trailing '/'.
package main

import (
	"fmt"
	"strings"
)

func simplifyPath(path string) string {
	stack := []string{}
	for _, token := range strings.Split(path, "/") {
		if token == "" || token == "." {
			continue
		}
		if token == ".." {
			if len(stack) > 0 {
				stack = stack[:len(stack)-1]
			}
		} else {
			stack = append(stack, token)
		}
	}
	result := "/" + strings.Join(stack, "/")
	endsWithSlash := strings.HasSuffix(path, "/")
	if endsWithSlash && result != "/" {
		result += "/"
	}
	return result
}

func main() {
	fmt.Println(simplifyPath("/usr/bin/../bin/./scripts/../"))
}
