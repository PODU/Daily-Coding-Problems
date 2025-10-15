// Day 432: Distributed Wikipedia crawler — concrete single-process simulation.
// Design: a central coordinator holds a URL frontier (BFS queue) + visited set for dedup.
// Workers pull URLs, "fetch" pages from an in-memory graph, parse links, store content keyed
// by a content hash + last-crawled timestamp, and push new URLs back. Politeness delay per
// domain + worker blacklist/rotation prevent overload. Incremental re-crawl compares the
// stored content hash to detect updates. O(V+E) over the page graph.
package main

import (
	"fmt"
	"strings"
)

type Page struct {
	content string
	links   []string
}

func fnv1a(s string) uint32 {
	var h uint32 = 2166136261
	for i := 0; i < len(s); i++ {
		h ^= uint32(s[i])
		h *= 16777619
	}
	return h
}

type Coordinator struct {
	wiki        map[string]Page
	frontier    []string
	visited     map[string]bool
	db          map[string][2]uint32 // url -> {hash, ts}
	order       []string
	workers     []string
	reqCount    map[string]int
	blacklisted map[string]bool
	lastAccess  int64
	wi          int
}

func newCoord(wiki map[string]Page) *Coordinator {
	return &Coordinator{
		wiki: wiki, visited: map[string]bool{}, db: map[string][2]uint32{},
		workers: []string{"w0", "w1", "w2"}, reqCount: map[string]int{},
		blacklisted: map[string]bool{},
	}
}

func (c *Coordinator) pickWorker() string {
	for k := 0; k < len(c.workers); k++ {
		w := c.workers[c.wi%len(c.workers)]
		c.wi++
		if !c.blacklisted[w] {
			return w
		}
	}
	c.blacklisted = map[string]bool{}
	return c.workers[0]
}

func (c *Coordinator) fetch(worker, url string) Page {
	c.lastAccess++ // politeness tick
	c.reqCount[worker]++
	if c.reqCount[worker] >= 2 { // rate-limit -> rotate out
		c.blacklisted[worker] = true
	}
	return c.wiki[url]
}

func (c *Coordinator) crawl(seed string) []string {
	c.frontier = append(c.frontier, seed)
	for len(c.frontier) > 0 {
		url := c.frontier[0]
		c.frontier = c.frontier[1:]
		if c.visited[url] {
			continue
		}
		c.visited[url] = true
		worker := c.pickWorker()
		page := c.fetch(worker, url)
		c.db[url] = [2]uint32{fnv1a(page.content), uint32(len(c.order))}
		c.order = append(c.order, url)
		for _, l := range page.links {
			if !c.visited[l] {
				c.frontier = append(c.frontier, l)
			}
		}
	}
	return c.order
}

func (c *Coordinator) recrawl(url string) (uint32, uint32, bool) {
	nh := fnv1a(c.wiki[url].content)
	oh := c.db[url][0]
	if nh != oh {
		c.db[url] = [2]uint32{nh, c.db[url][1]}
		return oh, nh, true
	}
	return 0, 0, false
}

func main() {
	wiki := map[string]Page{
		"Main": {"Welcome to the wiki", []string{"A", "B"}},
		"A":    {"Page A content", []string{"C"}},
		"B":    {"Page B content", []string{"C"}},
		"C":    {"Page C content", []string{"Main"}},
	}
	c := newCoord(wiki)
	order := c.crawl("Main")
	fmt.Printf("Crawled %d pages: [%s]\n", len(order), strings.Join(order, ", "))

	wiki["C"] = Page{"Page C content (edited 2026)", []string{"Main"}}
	oh, nh, ok := c.recrawl("C")
	if ok {
		fmt.Printf("Re-crawl detected update on 'C': hash %08x -> %08x, re-stored.\n", oh, nh)
	}
}
