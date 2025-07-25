// Run-length encoding/decoding in a single pass.
// Time: O(n), Space: O(n) for output.
package main

import (
	"fmt"
	"strconv"
	"strings"
)

func encode(s string) string {
	var sb strings.Builder
	n := len(s)
	for i := 0; i < n; {
		j := i
		for j < n && s[j] == s[i] {
			j++
		}
		sb.WriteString(strconv.Itoa(j - i))
		sb.WriteByte(s[i])
		i = j
	}
	return sb.String()
}

func decode(s string) string {
	var sb strings.Builder
	n := len(s)
	for i := 0; i < n; {
		count := 0
		for i < n && s[i] >= '0' && s[i] <= '9' {
			count = count*10 + int(s[i]-'0')
			i++
		}
		c := s[i]
		i++
		sb.WriteString(strings.Repeat(string(c), count))
	}
	return sb.String()
}

func main() {
	input := "AAAABBBCCDAA"
	enc := encode(input)
	fmt.Println(enc)
	fmt.Println(decode(enc))
}
