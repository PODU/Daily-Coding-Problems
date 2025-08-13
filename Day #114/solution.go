// Day 114: Reverse words but keep delimiters fixed. Collect words, reverse list,
// re-emit walking original structure. O(n) time, O(n) space.
package main

import (
	"fmt"
	"strings"
)

func reverseKeepDelims(s string, delim map[rune]bool) string {
	words := []string{}
	var cur strings.Builder
	for _, c := range s {
		if delim[c] {
			if cur.Len() > 0 {
				words = append(words, cur.String())
				cur.Reset()
			}
		} else {
			cur.WriteRune(c)
		}
	}
	if cur.Len() > 0 {
		words = append(words, cur.String())
	}
	for i, j := 0, len(words)-1; i < j; i, j = i+1, j-1 {
		words[i], words[j] = words[j], words[i]
	}

	rs := []rune(s)
	var out strings.Builder
	wi, i, n := 0, 0, len(rs)
	for i < n {
		if delim[rs[i]] {
			out.WriteRune(rs[i])
			i++
		} else {
			out.WriteString(words[wi])
			wi++
			for i < n && !delim[rs[i]] {
				i++
			}
		}
	}
	return out.String()
}

func main() {
	d := map[rune]bool{'/': true, ':': true}
	fmt.Println(reverseKeepDelims("hello/world:here", d))   // here/world:hello
	fmt.Println(reverseKeepDelims("hello/world:here/", d))  // here/world:hello/
	fmt.Println(reverseKeepDelims("hello//world:here", d))  // here//world:hello
}
