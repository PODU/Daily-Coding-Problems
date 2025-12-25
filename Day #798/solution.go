// Day 798: Sentence equivalence via synonym pairs (NOT transitive).
// Store each unordered pair in a set; words match if equal or directly paired.
// Time O(W) per comparison, Space O(P).
package main

import (
	"fmt"
	"strings"
)

var syn = map[string]bool{}

func key(a, b string) string {
	if a <= b {
		return a + "\x00" + b
	}
	return b + "\x00" + a
}

func add(a, b string) { syn[key(a, b)] = true }

func same(a, b string) bool { return a == b || syn[key(a, b)] }

func equivalent(s1, s2 []string) bool {
	if len(s1) != len(s2) {
		return false
	}
	for i := range s1 {
		if !same(s1[i], s2[i]) {
			return false
		}
	}
	return true
}

func main() {
	add("big", "large")
	add("eat", "consume")
	a := strings.Fields("He wants to eat food.")
	b := strings.Fields("He wants to consume food.")
	if equivalent(a, b) {
		fmt.Println("True (equivalent)")
	} else {
		fmt.Println("False")
	}
}
