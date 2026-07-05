// Direction-rule consistency: decompose each rule into strict x/y "greater-than"
// edges; a contradiction is a cycle in either axis graph (DFS cycle detection).
// Time: O(V+E) per axis, Space: O(V+E).
package main

import (
	"fmt"
	"sort"
)

type Checker struct {
	gx, gy map[string][]string
	nodes  map[string]bool
}

func newChecker() *Checker {
	return &Checker{gx: map[string][]string{}, gy: map[string][]string{}, nodes: map[string]bool{}}
}

func (c *Checker) edge(g map[string][]string, a, b string) {
	g[a] = append(g[a], b)
	c.nodes[a] = true
	c.nodes[b] = true
}

func (c *Checker) addRule(a, dir, b string) {
	for _, ch := range dir {
		switch ch {
		case 'N':
			c.edge(c.gy, a, b)
		case 'S':
			c.edge(c.gy, b, a)
		case 'E':
			c.edge(c.gx, a, b)
		case 'W':
			c.edge(c.gx, b, a)
		}
	}
}

func (c *Checker) hasCycle(g map[string][]string) bool {
	state := map[string]int{}
	var dfs func(u string) bool
	dfs = func(u string) bool {
		state[u] = 1
		for _, v := range g[u] {
			if state[v] == 1 {
				return true
			}
			if state[v] == 0 && dfs(v) {
				return true
			}
		}
		state[u] = 2
		return false
	}
	keys := make([]string, 0, len(c.nodes))
	for n := range c.nodes {
		keys = append(keys, n)
	}
	sort.Strings(keys)
	for _, n := range keys {
		if state[n] == 0 && dfs(n) {
			return true
		}
	}
	return false
}

func (c *Checker) validates() bool {
	return !c.hasCycle(c.gx) && !c.hasCycle(c.gy)
}

func main() {
	c1 := newChecker()
	c1.addRule("A", "N", "B")
	c1.addRule("B", "NE", "C")
	c1.addRule("C", "N", "A")
	if !c1.validates() {
		fmt.Println("does not validate, since A cannot be both north and south of C.")
	}

	c2 := newChecker()
	c2.addRule("A", "NW", "B")
	c2.addRule("A", "N", "B")
	if c2.validates() {
		fmt.Println("A NW B / A N B is considered valid.")
	}
}
