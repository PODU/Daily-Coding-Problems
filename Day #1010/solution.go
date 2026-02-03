// Unit Converter: graph where edge a->b stores factor (1 a = f b).
// convert() does BFS multiplying factors. Time O(V+E) per query, Space O(V+E).
package main

import (
	"fmt"
	"math"
)

type edge struct {
	to     string
	factor float64
}

type UnitConverter struct {
	g map[string][]edge
}

func NewUnitConverter() *UnitConverter {
	return &UnitConverter{g: make(map[string][]edge)}
}

func (u *UnitConverter) addUnit(from, to string, factor float64) {
	u.g[from] = append(u.g[from], edge{to, factor})
	u.g[to] = append(u.g[to], edge{from, 1.0 / factor})
}

func (u *UnitConverter) convert(value float64, from, to string) float64 {
	if from == to {
		return value
	}
	dist := map[string]float64{from: 1.0}
	q := []string{from}
	for len(q) > 0 {
		cur := q[0]
		q = q[1:]
		for _, e := range u.g[cur] {
			if _, ok := dist[e.to]; !ok {
				dist[e.to] = dist[cur] * e.factor
				if e.to == to {
					return value * dist[e.to]
				}
				q = append(q, e.to)
			}
		}
	}
	return math.NaN()
}

func main() {
	uc := NewUnitConverter()
	uc.addUnit("inch", "foot", 1.0/12)
	uc.addUnit("foot", "yard", 1.0/3)
	uc.addUnit("yard", "chain", 1.0/22)

	fmt.Printf("1 chain = %d inches\n", int(math.Round(uc.convert(1, "chain", "inch"))))
	fmt.Printf("36 inches = %d yards\n", int(math.Round(uc.convert(36, "inch", "yard"))))
}
