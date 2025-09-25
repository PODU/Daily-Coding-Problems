// Gale-Shapley stable marriage, men propose; free man proposes down his list, women keep better partner.
// Time O(n^2), Space O(n^2).
package main

import "fmt"

func main() {
	men := []string{"andrew", "bill", "chester"}
	women := []string{"abigail", "betty", "caroline"}
	guyPref := map[string][]string{
		"andrew":  {"caroline", "abigail", "betty"},
		"bill":    {"caroline", "betty", "abigail"},
		"chester": {"betty", "caroline", "abigail"},
	}
	galPref := map[string][]string{
		"abigail":  {"andrew", "bill", "chester"},
		"betty":    {"bill", "andrew", "chester"},
		"caroline": {"bill", "chester", "andrew"},
	}

	wrank := map[string]map[string]int{}
	for _, w := range women {
		wrank[w] = map[string]int{}
		for i, m := range galPref[w] {
			wrank[w][m] = i
		}
	}

	next := map[string]int{}
	for _, m := range men {
		next[m] = 0
	}
	partnerOf := map[string]string{} // woman -> man
	freeMen := append([]string{}, men...)

	for len(freeMen) > 0 {
		m := freeMen[0]
		freeMen = freeMen[1:]
		w := guyPref[m][next[m]]
		next[m]++
		if cur, ok := partnerOf[w]; !ok {
			partnerOf[w] = m
		} else {
			if wrank[w][m] < wrank[w][cur] {
				partnerOf[w] = m
				freeMen = append(freeMen, cur)
			} else {
				freeMen = append(freeMen, m)
			}
		}
	}

	manToWoman := map[string]string{}
	for woman, man := range partnerOf {
		manToWoman[man] = woman
	}
	for _, m := range men { // already sorted
		fmt.Printf("%s - %s\n", m, manToWoman[m])
	}
}
