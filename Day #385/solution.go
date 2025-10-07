// Brute-force all 256 single-byte keys; score by letters/spaces to pick plaintext.
// Time: O(256 * n), Space: O(n).
package main

import (
	"fmt"
	"strconv"
)

func hexToBytes(h string) []int {
	b := make([]int, len(h)/2)
	for i := range b {
		v, _ := strconv.ParseInt(h[2*i:2*i+2], 16, 32)
		b[i] = int(v)
	}
	return b
}

func score(codes []int) int {
	for _, c := range codes {
		if c < 32 || c > 126 {
			return -1
		}
	}
	sc := 0
	for _, c := range codes {
		if (c >= 'A' && c <= 'Z') || (c >= 'a' && c <= 'z') || c == ' ' {
			sc++
		}
	}
	return sc
}

func decrypt(hex string) string {
	bytes := hexToBytes(hex)
	best, bestScore := "", -1
	for k := 0; k < 256; k++ {
		codes := make([]int, len(bytes))
		buf := make([]byte, len(bytes))
		for i, b := range bytes {
			codes[i] = b ^ k
			buf[i] = byte(codes[i])
		}
		if sc := score(codes); sc > bestScore {
			bestScore = sc
			best = string(buf)
		}
	}
	return best
}

func main() {
	h := "7a575e5e5d12455d405e561254405d5f1276535b5e4b12715d565b5c551262405d505e575f"
	fmt.Println(decrypt(h))
}
