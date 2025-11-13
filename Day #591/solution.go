// Word sense disambiguation via simplified Lesk.
// Score each candidate gloss by content-word overlap with the sentence
// context (other words + their glosses); pick the highest.
// Time O(words * meanings * glossLen). Space O(vocab).
package main

import (
	"fmt"
	"strings"
)

var stop = map[string]bool{
	"i": true, "to": true, "the": true, "a": true, "an": true, "of": true,
	"by": true, "and": true, "or": true, "where": true, "people": true,
	"that": true, "is": true, "are": true, "in": true, "on": true, "at": true,
	"with": true, "went": true, "sat": true, "this": true,
}

func tokens(text string) map[string]bool {
	out := map[string]bool{}
	for _, w := range strings.Fields(strings.ToLower(text)) {
		var lw strings.Builder
		for _, c := range w {
			if c >= 'a' && c <= 'z' {
				lw.WriteRune(c)
			}
		}
		s := lw.String()
		if s != "" && !stop[s] {
			out[s] = true
		}
	}
	return out
}

func main() {
	meanings := map[string][]string{
		"bank": {"a financial institution where people deposit and withdraw money",
			"the land alongside a river or lake"},
		"money": {"currency that people deposit and withdraw"},
		"river": {"a large natural stream of water"},
	}

	sentences := []string{
		"I went to the bank to deposit money",
		"I sat by the bank of the river",
	}

	for _, sentence := range sentences {
		words := strings.Fields(sentence)
		for _, w := range words {
			lw := strings.ToLower(w)
			cands, ok := meanings[lw]
			if !ok || len(cands) <= 1 {
				continue
			}

			context := map[string]bool{}
			for _, other := range words {
				ol := strings.ToLower(other)
				if ol == lw {
					continue
				}
				for t := range tokens(other) {
					context[t] = true
				}
				for _, g := range meanings[ol] {
					for t := range tokens(g) {
						context[t] = true
					}
				}
			}

			best := -1
			bestGloss := ""
			for _, gloss := range cands {
				overlap := 0
				for t := range tokens(gloss) {
					if context[t] {
						overlap++
					}
				}
				if overlap > best {
					best = overlap
					bestGloss = gloss
				}
			}
			fmt.Printf("%s: %s\n", lw, bestGloss)
		}
	}
}
