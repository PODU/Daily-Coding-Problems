// Simplified Elo rating system. Expected score logistic, K=32 update on win/loss.
// recordGame adjusts both players. Time O(1) per game, Space O(players).
package main

import (
	"fmt"
	"math"
)

const K = 32.0

type Elo struct {
	ratings map[string]float64
}

func NewElo() *Elo { return &Elo{ratings: make(map[string]float64)} }

func (e *Elo) add(name string, r float64) { e.ratings[name] = r }

func expected(ra, rb float64) float64 {
	return 1.0 / (1.0 + math.Pow(10.0, (rb-ra)/400.0))
}

func (e *Elo) recordGame(winner, loser string) {
	ra, rb := e.ratings[winner], e.ratings[loser]
	ea, eb := expected(ra, rb), expected(rb, ra)
	e.ratings[winner] = ra + K*(1.0-ea)
	e.ratings[loser] = rb + K*(0.0-eb)
}

func main() {
	e := NewElo()
	e.add("A", 1200.0)
	e.add("B", 1200.0)
	fmt.Printf("Initial: A=%.2f B=%.2f\n", e.ratings["A"], e.ratings["B"])
	e.recordGame("B", "A")
	fmt.Printf("After B beats A (equal): A=%.2f B=%.2f\n", e.ratings["A"], e.ratings["B"])

	e2 := NewElo()
	e2.add("C", 1000.0)
	e2.add("D", 1600.0)
	fmt.Printf("Initial: C=%.2f D=%.2f\n", e2.ratings["C"], e2.ratings["D"])
	e2.recordGame("C", "D")
	fmt.Printf("After underdog C beats D: C=%.2f D=%.2f\n", e2.ratings["C"], e2.ratings["D"])
	fmt.Printf("Underdog gained %.2f vs even-match gain %.2f\n",
		e2.ratings["C"]-1000.0, e.ratings["B"]-1200.0)
}
