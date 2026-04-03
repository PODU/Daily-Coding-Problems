// Day 1294: Run-length encoding and decoding for alphabetic strings.
// Single linear scan for each direction. O(n) time, O(n) space.
package main

import (
	"fmt"
	"strconv"
	"strings"
)

func encode(s string) string {
	var out strings.Builder
	n := len(s)
	for i := 0; i < n; {
		j := i
		for j < n && s[j] == s[i] {
			j++
		}
		out.WriteString(strconv.Itoa(j - i))
		out.WriteByte(s[i])
		i = j
	}
	return out.String()
}

func decode(s string) string {
	var out strings.Builder
	count := 0
	for i := 0; i < len(s); i++ {
		c := s[i]
		if c >= '0' && c <= '9' {
			count = count*10 + int(c-'0')
		} else {
			out.WriteString(strings.Repeat(string(c), count))
			count = 0
		}
	}
	return out.String()
}

func main() {
	e := encode("AAAABBBCCDAA")
	fmt.Println(e)         // 4A3B2C1D2A
	fmt.Println(decode(e)) // AAAABBBCCDAA
}
