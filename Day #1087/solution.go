// B is a rotation of A iff lengths match and B is a substring of A+A. Time O(n), Space O(n).
package main

import (
	"fmt"
	"strings"
)

func isRotation(a, b string) bool {
	return len(a) == len(b) && strings.Contains(a+a, b)
}

func main() {
	fmt.Println(isRotation("abcde", "cdeab"))
	fmt.Println(isRotation("abc", "acb"))
}
