// Day 541: Run-length encoding/decoding. Scan runs to build count+char; parse digits to expand.
// Time O(n) encode, O(output) decode. Space O(n).
package main

import (
	"strconv"
	"strings"
)
import "fmt"

func encode(s string) string {
	var out strings.Builder
	for i := 0; i < len(s); {
		j := i
		for j < len(s) && s[j] == s[i] {
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
	original := "AAAABBBCCDAA"
	enc := encode(original)
	fmt.Println(enc) // 4A3B2C1D2A
	fmt.Println(decode(enc) == original)
}
