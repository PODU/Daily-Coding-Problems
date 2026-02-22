// Day 1115 - Uniform random in [0, n) excluding list l
// Approach: remap |E| excluded slots below m=n-|E| to available high slots,
// then sample once in [0, m). Time: O(|E|) prep, O(1)/sample.
package main

import (
	"fmt"
	"math/rand"
	"sort"
	"strings"
)

type Sampler struct {
	m       int
	mapping map[int]int
}

func NewSampler(n int, l []int) *Sampler {
	excluded := make(map[int]bool)
	for _, x := range l {
		if x >= 0 && x < n {
			excluded[x] = true
		}
	}
	m := n - len(excluded)
	var available []int
	for v := m; v < n; v++ {
		if !excluded[v] {
			available = append(available, v)
		}
	}
	mapping := make(map[int]int)
	var exKeys []int
	for e := range excluded {
		exKeys = append(exKeys, e)
	}
	sort.Ints(exKeys)
	ai := 0
	for _, e := range exKeys {
		if e < m {
			mapping[e] = available[ai]
			ai++
		}
	}
	return &Sampler{m: m, mapping: mapping}
}

func (s *Sampler) Sample() int {
	r := rand.Intn(s.m)
	if v, ok := s.mapping[r]; ok {
		return v
	}
	return r
}

func main() {
	n, l := 10, []int{2, 5, 7}
	s := NewSampler(n, l)
	seenSet := make(map[int]bool)
	for i := 0; i < 2000; i++ {
		seenSet[s.Sample()] = true
	}
	var seen []int
	for v := range seenSet {
		seen = append(seen, v)
	}
	sort.Ints(seen)
	parts := make([]string, len(seen))
	for i, v := range seen {
		parts[i] = fmt.Sprintf("%d", v)
	}
	fmt.Println("n=10, excluded=[2, 5, 7]")
	fmt.Printf("Sampled valid numbers: [%s]\n", strings.Join(parts, ", "))
}
