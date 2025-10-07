// Base64 -> bytes -> hex. Bit-accumulator decode (tolerates padding/whitespace).
// Time: O(n), Space: O(n).
package main

import (
	"fmt"
	"strings"
)

const b64 = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"

func base64ToHex(s string) string {
	val := make(map[rune]int)
	for i, c := range b64 {
		val[c] = i
	}
	bits, nbits := 0, 0
	var out strings.Builder
	for _, c := range s {
		v, ok := val[c]
		if !ok {
			continue
		}
		bits = (bits << 6) | v
		nbits += 6
		if nbits >= 8 {
			nbits -= 8
			b := (bits >> nbits) & 0xFF
			fmt.Fprintf(&out, "%02x", b)
		}
	}
	return out.String()
}

func main() {
	fmt.Println(base64ToHex("3q2+7w="))
}
