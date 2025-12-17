// Day 756: Recover digits from an anagram of their English spellings.
// Use unique marker letters (z,w,u,x,g) then deduce the rest. Time: O(n), Space: O(1).
package main

import (
	"fmt"
	"strings"
)

func recoverDigits(s string) string {
	var cnt [26]int
	for _, c := range s {
		cnt[c-'a']++
	}
	g := func(ch byte) int { return cnt[ch-'a'] }
	var d [10]int
	d[0] = g('z')
	d[2] = g('w')
	d[4] = g('u')
	d[6] = g('x')
	d[8] = g('g')
	d[1] = g('o') - d[0] - d[2] - d[4]
	d[3] = g('h') - d[8]
	d[5] = g('f') - d[4]
	d[7] = g('s') - d[6]
	d[9] = g('i') - d[5] - d[6] - d[8]

	var sb strings.Builder
	for i := 0; i < 10; i++ {
		sb.WriteString(strings.Repeat(fmt.Sprintf("%d", i), d[i]))
	}
	return sb.String()
}

func main() {
	fmt.Println(recoverDigits("niesevehrtfeev")) // 357
}
