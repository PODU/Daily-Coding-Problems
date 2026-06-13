// Simplified Lesk WSD: score each candidate meaning by overlap with the union of other
// in-dict context words and their meaning texts. O(W*M*L) time, O(V) space.
package main

import (
	"fmt"
	"strings"
)

func words(s string) []string { return strings.Fields(strings.ToLower(s)) }

func main() {
	meanings := map[string][]string{
		"bank":  {"place where people deposit and withdraw money", "land beside a river or lake"},
		"money": {"currency coins and cash used to buy things"},
		"river": {"a large natural stream of flowing water"},
	}
	sentence := "I went to get money from the bank"
	toks := words(sentence)
	for _, w := range toks {
		if m, ok := meanings[w]; ok && len(m) > 1 {
			ctx := map[string]bool{}
			for _, o := range toks {
				if o != w {
					if om, ok2 := meanings[o]; ok2 {
						ctx[o] = true
						for _, mm := range om {
							for _, x := range words(mm) {
								ctx[x] = true
							}
						}
					}
				}
			}
			best, bestScore := m[0], -1
			for _, cand := range m {
				score := 0
				for _, t := range words(cand) {
					if ctx[t] {
						score++
					}
				}
				if score > bestScore {
					bestScore = score
					best = cand
				}
			}
			fmt.Printf("%s: %s\n", w, best)
		}
	}
}
