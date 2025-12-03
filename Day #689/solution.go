// Unit conversion as a weighted graph; convert via BFS multiplying edge ratios.
// add_conversion(a, b, r): 1 a = r b. Time O(V + E) per query, Space O(V + E).
package main

import (
	"fmt"
	"math"
)

type UnitConverter struct {
	graph map[string]map[string]float64
}

func NewUnitConverter() *UnitConverter {
	return &UnitConverter{graph: make(map[string]map[string]float64)}
}

func (uc *UnitConverter) AddConversion(frm, to string, factor float64) {
	// 1 frm = factor to
	if uc.graph[frm] == nil {
		uc.graph[frm] = make(map[string]float64)
	}
	if uc.graph[to] == nil {
		uc.graph[to] = make(map[string]float64)
	}
	uc.graph[frm][to] = factor
	uc.graph[to][frm] = 1.0 / factor
}

func (uc *UnitConverter) Convert(qty float64, frm, to string) (float64, bool) {
	if frm == to {
		return qty, true
	}
	if _, ok := uc.graph[frm]; !ok {
		return 0, false
	}
	if _, ok := uc.graph[to]; !ok {
		return 0, false
	}
	type item struct {
		unit  string
		ratio float64
	}
	visited := map[string]bool{frm: true}
	queue := []item{{frm, 1.0}}
	for len(queue) > 0 {
		cur := queue[0]
		queue = queue[1:]
		if cur.unit == to {
			return qty * cur.ratio, true
		}
		for nxt, f := range uc.graph[cur.unit] {
			if !visited[nxt] {
				visited[nxt] = true
				queue = append(queue, item{nxt, cur.ratio * f})
			}
		}
	}
	return 0, false
}

func main() {
	uc := NewUnitConverter()
	uc.AddConversion("foot", "inch", 12)   // 12 inches = 1 foot
	uc.AddConversion("yard", "foot", 3)    // 3 feet = 1 yard
	uc.AddConversion("chain", "yard", 22)  // 22 yards = 1 chain

	r1, _ := uc.Convert(1, "chain", "inch")
	r2, _ := uc.Convert(1, "yard", "inch")
	fmt.Printf("1 chain = %d inches\n", int64(math.Round(r1)))
	fmt.Printf("1 yard = %d inches\n", int64(math.Round(r2)))
}
