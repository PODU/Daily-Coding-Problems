// Recover digits from scrambled letters: use unique marker letters (z,w,u,x,g) then derive the rest. O(N) time, O(1) space.
package main

import (
	"fmt"
	"strings"
)

func recover_(s string) string {
	var c [26]int
	for _, ch := range s {
		if ch >= 'a' && ch <= 'z' {
			c[ch-'a']++
		}
	}
	var d [10]int
	d[0] = c['z'-'a']
	d[2] = c['w'-'a']
	d[4] = c['u'-'a']
	d[6] = c['x'-'a']
	d[8] = c['g'-'a']
	d[3] = c['h'-'a'] - d[8]
	d[5] = c['f'-'a'] - d[4]
	d[7] = c['s'-'a'] - d[6]
	d[1] = c['o'-'a'] - d[0] - d[2] - d[4]
	d[9] = c['i'-'a'] - d[5] - d[6] - d[8]
	var sb strings.Builder
	for i := 0; i < 10; i++ {
		sb.WriteString(strings.Repeat(fmt.Sprintf("%d", i), d[i]))
	}
	return sb.String()
}

func main() {
	fmt.Println(recover_("niesevehrtfeev"))
}
