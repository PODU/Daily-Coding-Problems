// Elo rating: expected = 1/(1+10^((Rb-Ra)/400)); delta = K*(score-expected), zero-sum. O(1) per game.
package main

import (
	"fmt"
	"math"
	"sort"
)

type EloSystem struct {
	ratings map[string]float64
	K       float64
}

func NewEloSystem() *EloSystem {
	return &EloSystem{ratings: make(map[string]float64), K: 32.0}
}

func (e *EloSystem) Add(p string) { e.ratings[p] = 1200.0 }

func expected(ra, rb float64) float64 {
	return 1.0 / (1.0 + math.Pow(10.0, (rb-ra)/400.0))
}

func (e *EloSystem) RecordGame(w, l string) {
	rw, rl := e.ratings[w], e.ratings[l]
	ew := expected(rw, rl)
	delta := e.K * (1.0 - ew)
	e.ratings[w] = rw + delta
	e.ratings[l] = rl - delta
	fmt.Printf("%s beats %s: %s %d->%d, %s %d->%d\n",
		w, l, w, int64(math.Round(rw)), int64(math.Round(rw+delta)),
		l, int64(math.Round(rl)), int64(math.Round(rl-delta)))
}

func main() {
	e := NewEloSystem()
	for _, p := range []string{"A", "B", "C", "D"} {
		e.Add(p)
	}
	e.RecordGame("A", "B")
	e.RecordGame("A", "C")
	e.RecordGame("D", "A")
	fmt.Println("Final ratings:")
	keys := make([]string, 0, len(e.ratings))
	for k := range e.ratings {
		keys = append(keys, k)
	}
	sort.Strings(keys)
	for _, k := range keys {
		fmt.Printf("%s %d\n", k, int64(math.Round(e.ratings[k])))
	}
}
