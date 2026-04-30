// Day 1445: Unit converter. Model units as a weighted graph (edge = conversion
// factor) and BFS for a path on query. AddUnit O(1); Convert O(V+E).
package main

import (
	"fmt"
	"math"
)

type UnitConverter struct {
	graph map[string]map[string]float64 // graph[a][b] = factor; 1 a = factor b
}

func NewUnitConverter() *UnitConverter {
	return &UnitConverter{graph: map[string]map[string]float64{}}
}

func (uc *UnitConverter) AddUnit(from, to string, factor float64) {
	if uc.graph[from] == nil {
		uc.graph[from] = map[string]float64{}
	}
	if uc.graph[to] == nil {
		uc.graph[to] = map[string]float64{}
	}
	uc.graph[from][to] = factor
	uc.graph[to][from] = 1.0 / factor
}

func (uc *UnitConverter) Convert(value float64, from, to string) float64 {
	if from == to {
		return value
	}
	dist := map[string]float64{from: 1.0}
	queue := []string{from}
	for len(queue) > 0 {
		u := queue[0]
		queue = queue[1:]
		for v, f := range uc.graph[u] {
			if _, ok := dist[v]; !ok {
				dist[v] = dist[u] * f
				if v == to {
					return value * dist[v]
				}
				queue = append(queue, v)
			}
		}
	}
	return math.NaN()
}

func main() {
	uc := NewUnitConverter()
	uc.AddUnit("foot", "inch", 12)
	uc.AddUnit("yard", "foot", 3)
	uc.AddUnit("chain", "yard", 22)
	fmt.Println(int(uc.Convert(1, "yard", "inch"))) // 36
}
