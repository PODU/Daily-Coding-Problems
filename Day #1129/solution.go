// Simplified Elo rating system. Expected score Ea=1/(1+10^((Rb-Ra)/400)),
// update Ra'=Ra+K*(Sa-Ea), K=32. recordGame transfers points winner<-loser. O(1) per game.
package main

import (
	"fmt"
	"math"
)

const K = 32.0

type EloSystem struct {
	rating map[string]float64
}

func NewElo() *EloSystem { return &EloSystem{rating: make(map[string]float64)} }

func (e *EloSystem) addPlayer(name string) {
	if _, ok := e.rating[name]; !ok {
		e.rating[name] = 1200.0
	}
}
func expected(ra, rb float64) float64 { return 1.0 / (1.0 + math.Pow(10.0, (rb-ra)/400.0)) }

func (e *EloSystem) recordGame(winner, loser string) {
	e.addPlayer(winner)
	e.addPlayer(loser)
	ra, rb := e.rating[winner], e.rating[loser]
	ea, eb := expected(ra, rb), expected(rb, ra)
	e.rating[winner] = ra + K*(1.0-ea)
	e.rating[loser] = rb + K*(0.0-eb)
}
func (e *EloSystem) get(name string) int { return int(math.Round(e.rating[name])) }

func main() {
	elo := NewElo()
	elo.addPlayer("A")
	elo.rating["A"] = 1200.0
	elo.addPlayer("B")
	elo.rating["B"] = 2000.0
	fmt.Printf("Before: A=%d, B=%d\n", elo.get("A"), elo.get("B"))
	elo.recordGame("A", "B") // lower-rated A beats higher-rated B
	fmt.Printf("After A(1200) beats B(2000): A=%d, B=%d\n", elo.get("A"), elo.get("B"))
}
