// Run-length encoding/decoding. Single pass over the string.
// Time: O(n) encode/decode, Space: O(n) for output.
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
	n := len(s)
	for i := 0; i < n; {
		cnt := 0
		for i < n && s[i] >= '0' && s[i] <= '9' {
			cnt = cnt*10 + int(s[i]-'0')
			i++
		}
		c := s[i]
		i++
		out.WriteString(strings.Repeat(string(c), cnt))
	}
	return out.String()
}

func main() {
	input := "AAAABBBCCDAA"
	enc := encode(input)
	fmt.Println(enc)
	fmt.Println(decode(enc))
}
