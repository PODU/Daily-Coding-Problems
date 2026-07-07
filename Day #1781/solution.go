// Unit converter as a graph: addRelation stores 1 a = factor b (edge a->b, b->a=1/factor).
// convert does BFS multiplying factors along the path. Time O(V+E) per query, Space O(V+E).
package main

import "fmt"

type UnitConverter struct {
	adj map[string]map[string]float64
}

func NewUnitConverter() *UnitConverter {
	return &UnitConverter{adj: make(map[string]map[string]float64)}
}

func (u *UnitConverter) edge(a, b string, f float64) {
	if u.adj[a] == nil {
		u.adj[a] = make(map[string]float64)
	}
	u.adj[a][b] = f
}

func (u *UnitConverter) AddRelation(a, b string, factor float64) {
	u.edge(a, b, factor)
	u.edge(b, a, 1/factor)
}

// returns (value, ok)
func (u *UnitConverter) Convert(qty float64, from, to string) (float64, bool) {
	if from == to {
		return qty, true
	}
	if _, ok := u.adj[from]; !ok {
		return 0, false
	}
	if _, ok := u.adj[to]; !ok {
		return 0, false
	}
	dist := map[string]float64{from: qty}
	q := []string{from}
	for len(q) > 0 {
		cur := q[0]
		q = q[1:]
		for nxt, f := range u.adj[cur] {
			if _, seen := dist[nxt]; !seen {
				dist[nxt] = dist[cur] * f
				if nxt == to {
					return dist[nxt], true
				}
				q = append(q, nxt)
			}
		}
	}
	return 0, false
}

func main() {
	uc := NewUnitConverter()
	uc.AddRelation("inches", "foot", 1.0/12.0)
	uc.AddRelation("feet", "yard", 1.0/3.0)
	uc.AddRelation("yards", "chain", 1.0/22.0)
	uc.AddRelation("foot", "feet", 1.0)
	uc.AddRelation("yard", "yards", 1.0)

	v1, _ := uc.Convert(1, "yard", "inches")
	v2, _ := uc.Convert(1, "chain", "inches")
	v3, _ := uc.Convert(1, "chain", "feet")
	fmt.Printf("1 yard = %.0f inches\n", v1)
	fmt.Printf("1 chain = %.0f inches\n", v2)
	fmt.Printf("1 chain = %.0f feet\n", v3)
}
