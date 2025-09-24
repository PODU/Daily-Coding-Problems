// Unit conversion via weighted graph; addConversion adds a<->b edges, convert does BFS multiplying ratios.
// Time: O(V+E) per query, Space: O(V+E).
package main

import (
	"fmt"
	"math"
)

type edge struct {
	to    string
	ratio float64
}

type UnitConverter struct {
	adj map[string][]edge
}

func NewUnitConverter() *UnitConverter {
	return &UnitConverter{adj: make(map[string][]edge)}
}

func (uc *UnitConverter) AddConversion(a, b string, ratio float64) {
	uc.adj[a] = append(uc.adj[a], edge{b, ratio})
	uc.adj[b] = append(uc.adj[b], edge{a, 1.0 / ratio})
}

func (uc *UnitConverter) Convert(value float64, from, to string) float64 {
	if from == to {
		return value
	}
	dist := map[string]float64{from: value}
	q := []string{from}
	for len(q) > 0 {
		u := q[0]
		q = q[1:]
		for _, e := range uc.adj[u] {
			if _, ok := dist[e.to]; !ok {
				dist[e.to] = dist[u] * e.ratio
				if e.to == to {
					return dist[e.to]
				}
				q = append(q, e.to)
			}
		}
	}
	if v, ok := dist[to]; ok {
		return v
	}
	return math.NaN()
}

func main() {
	uc := NewUnitConverter()
	uc.AddConversion("foot", "inch", 12)
	uc.AddConversion("yard", "foot", 3)
	uc.AddConversion("chain", "yard", 22)
	r := uc.Convert(1, "yard", "inch")
	fmt.Printf("1 yard = %.1f inch\n", r)
}
