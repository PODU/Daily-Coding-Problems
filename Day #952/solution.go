// Day 952: decode an anagram of spelled-out digit words (zero..nine) -> sorted digits.
// Use unique marker letters to count each digit. Time O(n), Space O(1).
package main

import (
	"fmt"
	"strings"
)

func decode(s string) string {
	var c [26]int
	for _, ch := range s {
		c[ch-'a']++
	}
	g := func(ch byte) int { return c[ch-'a'] }
	cnt := make([]int, 10)
	cnt[0] = g('z')
	cnt[2] = g('w')
	cnt[4] = g('u')
	cnt[6] = g('x')
	cnt[8] = g('g')
	cnt[3] = g('h') - cnt[8]
	cnt[5] = g('f') - cnt[4]
	cnt[7] = g('s') - cnt[6]
	cnt[1] = g('o') - cnt[0] - cnt[2] - cnt[4]
	cnt[9] = g('i') - cnt[5] - cnt[6] - cnt[8]
	var sb strings.Builder
	for d := 0; d <= 9; d++ {
		sb.WriteString(strings.Repeat(fmt.Sprintf("%d", d), cnt[d]))
	}
	return sb.String()
}

func main() {
	fmt.Println(decode("niesevehrtfeev")) // 357
}
