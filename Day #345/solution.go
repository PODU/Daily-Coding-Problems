// Sentence Similarity. Direct (non-transitive) synonym pairs via map set.
// Time O(P + N) for P pairs and N words. Space O(P).
// Secondary union-find approach (transitive follow-up) included below.
package main

import (
	"fmt"
	"strings"
)

func tokenize(s string) []string {
	fields := strings.Fields(s)
	out := make([]string, 0, len(fields))
	for _, w := range fields {
		out = append(out, strings.TrimRight(w, ".,!?;:"))
	}
	return out
}

func areSimilar(synonyms [][2]string, s1, s2 string) bool {
	pairs := make(map[string]bool)
	for _, p := range synonyms {
		pairs[p[0]+"\x00"+p[1]] = true
		pairs[p[1]+"\x00"+p[0]] = true
	}
	w1, w2 := tokenize(s1), tokenize(s2)
	if len(w1) != len(w2) {
		return false
	}
	for i := range w1 {
		if w1[i] == w2[i] {
			continue
		}
		if pairs[w1[i]+"\x00"+w2[i]] {
			continue
		}
		return false
	}
	return true
}

// --- Follow-up (transitive): union-find ---
func areSimilarTransitive(synonyms [][2]string, s1, s2 string) bool {
	parent := make(map[string]string)
	var find func(string) string
	find = func(x string) string {
		if _, ok := parent[x]; !ok {
			parent[x] = x
		}
		for parent[x] != x {
			parent[x] = parent[parent[x]]
			x = parent[x]
		}
		return x
	}
	for _, p := range synonyms {
		parent[find(p[0])] = find(p[1])
	}
	w1, w2 := tokenize(s1), tokenize(s2)
	if len(w1) != len(w2) {
		return false
	}
	for i := range w1 {
		if w1[i] != w2[i] && find(w1[i]) != find(w2[i]) {
			return false
		}
	}
	return true
}

func main() {
	synonyms := [][2]string{{"big", "large"}, {"eat", "consume"}}
	s1 := "He wants to eat food."
	s2 := "He wants to consume food."
	if areSimilar(synonyms, s1, s2) {
		fmt.Println("equivalent")
	} else {
		fmt.Println("not equivalent")
	}
}
