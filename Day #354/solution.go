// Distributed crawler design (simulation): Coordinator holds a FIFO URL frontier + central visited set.
// Workers pull a URL, "download" (map lookup), record it, push unvisited links. BFS over a mock graph.
// Architecture: reach pages = BFS from seeds; visited = central set/bloom filter on coordinator;
// blacklisting = rotate IPs/user-agents, rate-limit + backoff; updates = recrawl by last-modified queue.
// Time O(V+E), Space O(V).
package main

import "fmt"

func main() {
	wiki := map[string][]string{
		"/Main":   {"/Apple", "/Banana"},
		"/Apple":  {"/Banana", "/Fruit"},
		"/Banana": {"/Fruit"},
		"/Fruit":  {"/Main"},
	}

	seeds := []string{"/Main"}
	var frontier []string
	visited := make(map[string]bool)
	var order []string

	for _, s := range seeds {
		if !visited[s] {
			visited[s] = true
			frontier = append(frontier, s)
		}
	}

	for len(frontier) > 0 {
		url := frontier[0]
		frontier = frontier[1:]
		order = append(order, url) // "download" + record into results DB
		for _, link := range wiki[url] {
			if !visited[link] {
				visited[link] = true
				frontier = append(frontier, link)
			}
		}
	}

	fmt.Printf("Crawled %d pages:\n", len(order))
	for _, p := range order {
		fmt.Println(p)
	}
}
