// Day 901: Distributed Wikipedia crawler (concrete simulation).
// Approach: BFS over a page link graph with a shared visited set (dedup) and a
// frontier queue. Production: distributed frontier queue, sharded/bloom visited
// store, rotating rate-limited clients to avoid bans, RecentChanges-driven
// re-crawl. Time: O(V+E), Space: O(V).
package main

import (
	"fmt"
	"sort"
)

type CrawlerSystem struct {
	linkGraph map[string][]string
	visited   map[string]bool   // central dedup store
	db        map[string]string // page -> content
}

func NewCrawler(g map[string][]string) *CrawlerSystem {
	return &CrawlerSystem{linkGraph: g, visited: map[string]bool{}, db: map[string]string{}}
}

func (c *CrawlerSystem) crawl(seeds []string) {
	frontier := append([]string{}, seeds...) // distributed work queue
	for _, s := range seeds {
		c.visited[s] = true
	}
	for len(frontier) > 0 {
		page := frontier[0]
		frontier = frontier[1:]
		c.db[page] = "content of " + page
		for _, nxt := range c.linkGraph[page] {
			if !c.visited[nxt] { // dedup before enqueue
				c.visited[nxt] = true
				frontier = append(frontier, nxt)
			}
		}
	}
}

func main() {
	graph := map[string][]string{
		"Main_Page":   {"Python", "Wikipedia"},
		"Python":      {"Programming", "Wikipedia"},
		"Wikipedia":   {"Python"},
		"Programming": {},
	}
	sys := NewCrawler(graph)
	sys.crawl([]string{"Main_Page"})
	keys := make([]string, 0, len(sys.db))
	for k := range sys.db {
		keys = append(keys, k)
	}
	sort.Strings(keys)
	fmt.Println("Pages crawled:", len(sys.db))
	fmt.Println("Visited:", keys)
}
