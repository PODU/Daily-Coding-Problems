// Hex string -> bytes -> standard Base64 (with '=' padding).
// Time: O(n), Space: O(n).  Note: canonical Base64 of "deadbeef" is "3q2+7w==".
package main

import (
	"fmt"
	"strconv"
	"strings"
)

const b64 = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"

func hexToBase64(h string) string {
	b := make([]int, len(h)/2)
	for i := range b {
		v, _ := strconv.ParseInt(h[2*i:2*i+2], 16, 32)
		b[i] = int(v)
	}
	var out strings.Builder
	for i := 0; i < len(b); i += 3 {
		rem := len(b) - i
		n := b[i] << 16
		if rem > 1 {
			n |= b[i+1] << 8
		}
		if rem > 2 {
			n |= b[i+2]
		}
		out.WriteByte(b64[(n>>18)&63])
		out.WriteByte(b64[(n>>12)&63])
		if rem > 1 {
			out.WriteByte(b64[(n>>6)&63])
		} else {
			out.WriteByte('=')
		}
		if rem > 2 {
			out.WriteByte(b64[n&63])
		} else {
			out.WriteByte('=')
		}
	}
	return out.String()
}

func main() {
	fmt.Println(hexToBase64("deadbeef"))
}
