// Distributed Wikipedia crawler simulation: master holds a FIFO URL frontier +
// visited set (dedup); workers "fetch" mock pages, parse links, report back; BFS.
// Real design: shard frontier by URL hash (load-balance), distributed visited set
// (bloom filter / consistent hashing), rotating IPs + politeness/backoff to avoid
// blacklisting, recrawl via Last-Modified timestamps. Time/Space: O(V + E).
package main

import "fmt"

func main() {
	// Mock in-memory "Wikipedia" link graph (adjacency, ordered).
	graph := map[string][]string{
		"Wikipedia":        {"Computer_Science", "Mathematics"},
		"Computer_Science": {"Algorithms", "Mathematics"},
		"Mathematics":      {"Algorithms"},
		"Algorithms":       {},
	}

	fetch := func(url string) []string { return graph[url] } // worker fetch+parse

	seed := "Wikipedia"
	frontier := []string{seed}            // master FIFO frontier
	seen := map[string]bool{seed: true}   // dedup set
	db := map[string][]string{}           // crawled database

	for len(frontier) > 0 {
		url := frontier[0]
		frontier = frontier[1:]
		links := fetch(url) // worker reports content + links
		db[url] = links
		fmt.Println("Crawled: " + url)
		for _, nxt := range links {
			if !seen[nxt] {
				seen[nxt] = true
				frontier = append(frontier, nxt)
			}
		}
	}
}
