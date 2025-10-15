// Day 439: Reconstruct itinerary using all flights, lexicographically smallest.
// Hierholzer's Eulerian-path algorithm with sorted adjacency. O(E log E).
package main

import (
	"fmt"
	"sort"
)

func findItinerary(flights [][2]string, start string) []string {
	graph := make(map[string][]string)
	for _, f := range flights {
		graph[f[0]] = append(graph[f[0]], f[1])
	}
	// sort descending so the last element (popped) is lexicographically smallest
	for k := range graph {
		sort.Sort(sort.Reverse(sort.StringSlice(graph[k])))
	}

	var route []string
	stack := []string{start}
	for len(stack) > 0 {
		u := stack[len(stack)-1]
		if list := graph[u]; len(list) > 0 {
			next := list[len(list)-1]
			graph[u] = list[:len(list)-1]
			stack = append(stack, next)
		} else {
			route = append(route, u)
			stack = stack[:len(stack)-1]
		}
	}
	// reverse
	for i, j := 0, len(route)-1; i < j; i, j = i+1, j-1 {
		route[i], route[j] = route[j], route[i]
	}
	if len(route) != len(flights)+1 {
		return nil
	}
	return route
}

func printRoute(route []string) {
	if route == nil {
		fmt.Println("nil") // no valid itinerary
		return
	}
	fmt.Println(route)
}

func main() {
	flights := [][2]string{{"SFO", "HKO"}, {"YYZ", "SFO"}, {"YUL", "YYZ"}, {"HKO", "ORD"}}
	printRoute(findItinerary(flights, "YUL"))
	// [YUL YYZ SFO HKO ORD]
	printRoute(findItinerary([][2]string{{"SFO", "COM"}, {"COM", "YYZ"}}, "COM")) // nil
	printRoute(findItinerary([][2]string{{"A", "B"}, {"A", "C"}, {"B", "C"}, {"C", "A"}}, "A"))
	// [A B C A C]
}
