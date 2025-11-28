// Day 667: Simplified Elo rating. Expected score E = 1/(1+10^((Rb-Ra)/400)),
// update R += K*(actual - expected). Underdog gains more. Each update O(1).
package main

import (
	"fmt"
	"math"
)

type Elo struct {
	K, start float64
	r        map[string]float64
}

func NewElo() *Elo { return &Elo{32, 1200, map[string]float64{}} }
func (e *Elo) rating(p string) float64 {
	if _, ok := e.r[p]; !ok {
		e.r[p] = e.start
	}
	return e.r[p]
}
func (e *Elo) game(winner, loser string) {
	ra, rb := e.rating(winner), e.rating(loser)
	ea := 1.0 / (1.0 + math.Pow(10, (rb-ra)/400.0))
	eb := 1.0 - ea
	e.r[winner] = ra + e.K*(1-ea)
	e.r[loser] = rb + e.K*(0-eb)
}

func main() {
	e := NewElo()
	e.r["A"], e.r["B"] = 1200, 2000
	e.game("A", "B")
	fmt.Printf("A: %.1f\n", e.rating("A")) // ~1231.5
	fmt.Printf("B: %.1f\n", e.rating("B")) // ~1968.5
}
