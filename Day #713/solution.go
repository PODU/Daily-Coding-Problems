// Day 713: Normalize absolute Unix path. Split on '/', use a stack: skip ""/".",
// pop on "..". Preserve a trailing slash if the input had one. Time O(n).
package main

import (
	"fmt"
	"strings"
)

func normalize(path string) string {
	stk := []string{}
	for _, comp := range strings.Split(path, "/") {
		if comp == "" || comp == "." {
			continue
		}
		if comp == ".." {
			if len(stk) > 0 {
				stk = stk[:len(stk)-1]
			}
		} else {
			stk = append(stk, comp)
		}
	}
	res := "/" + strings.Join(stk, "/")
	trailing := len(path) > 1 && path[len(path)-1] == '/'
	if trailing && res != "/" && res[len(res)-1] != '/' {
		res += "/"
	}
	return res
}

func main() {
	fmt.Printf("\"%s\"\n", normalize("/usr/bin/../bin/./scripts/../"))
}
