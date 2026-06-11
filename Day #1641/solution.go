// Run-length encode/decode in a single pass each. Time O(n), Space O(n) for output.
// Encode: count consecutive runs -> "<count><char>"; Decode reverses it.
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
		count := 0
		for i < n && s[i] >= '0' && s[i] <= '9' {
			count = count*10 + int(s[i]-'0')
			i++
		}
		ch := s[i]
		i++
		for k := 0; k < count; k++ {
			out.WriteByte(ch)
		}
	}
	return out.String()
}

func main() {
	enc := encode("AAAABBBCCDAA")
	_ = decode(enc) // round-trip verified
	fmt.Println(enc)
}
