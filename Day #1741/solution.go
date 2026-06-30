// Approach: Single-process simulation of a distributed BFS web crawler. Central frontier (queue) +
// visited set for dedup; N round-robin workers fetch outlinks from a mock graph, store to a mock DB,
// requeue work from a blacklisted worker. Time O(V+E), Space O(V+E).
//
// Real distributed design: a master holds the frontier and shards the "URL-seen" set across nodes via
// consistent hashing or a bloom filter; workers crawl politely (robots.txt, rate limits, rotating IPs)
// to avoid blacklisting; recrawl uses last-modified/ETag to detect and update changed pages.

package main

import (
	"fmt"
	"sort"
	"strings"
)

func main() {
	graph := map[string][]string{
		"Main": {"A", "B", "C"},
		"A":    {"B", "D"},
		"B":    {"C"},
		"C":    {"A", "E"},
		"D":    {"E"},
		"E":    {},
	}

	numWorkers := 3
	frontier := []string{"Main"}
	visited := map[string]bool{"Main": true}
	db := map[string][]string{}
	blacklisted := map[int]bool{}
	worker := 0
	processedAny := false

	for len(frontier) > 0 {
		url := frontier[0]
		frontier = frontier[1:]
		w := worker % numWorkers
		worker++

		// Blacklist worker #1 after at least one page processed; requeue its task.
		if w == 1 && processedAny && !blacklisted[1] {
			blacklisted[1] = true
			frontier = append(frontier, url) // in-flight task requeued, no page lost
			continue
		}

		db[url] = graph[url] // "fetch" + store to mock DB
		processedAny = true

		for _, link := range graph[url] {
			if !visited[link] {
				visited[link] = true
				frontier = append(frontier, link)
			}
		}
	}

	fmt.Printf("Crawled %d pages\n", len(db))
	titles := make([]string, 0, len(db))
	for k := range db {
		titles = append(titles, k)
	}
	sort.Strings(titles)
	fmt.Println(strings.Join(titles, " "))
}
