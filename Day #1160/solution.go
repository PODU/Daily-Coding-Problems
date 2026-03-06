// Recover digits from scrambled English spellings using unique identifying letters.
// Time: O(L), Space: O(1).
package main

import (
	"fmt"
	"strings"
)

func recover_(s string) string {
	var c [26]int
	for _, ch := range s {
		c[ch-'a']++
	}
	var cnt [10]int
	cnt[0] = c['z'-'a']
	cnt[2] = c['w'-'a']
	cnt[4] = c['u'-'a']
	cnt[6] = c['x'-'a']
	cnt[8] = c['g'-'a']
	cnt[3] = c['h'-'a'] - cnt[8]
	cnt[5] = c['f'-'a'] - cnt[4]
	cnt[7] = c['s'-'a'] - cnt[6]
	cnt[1] = c['o'-'a'] - cnt[0] - cnt[2] - cnt[4]
	cnt[9] = c['i'-'a'] - cnt[5] - cnt[6] - cnt[8]
	var sb strings.Builder
	for d := 0; d <= 9; d++ {
		sb.WriteString(strings.Repeat(string(rune('0'+d)), cnt[d]))
	}
	return sb.String()
}

func main() {
	fmt.Println(recover_("niesevehrtfeev"))
}
