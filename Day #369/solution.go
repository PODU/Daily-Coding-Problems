// Day 369: Stock price tracker.
// ts2price maps timestamp->price; a sorted slice of prices (binary insert) gives
// min/max at the ends, running sum+count give average. add/update/remove O(n).
package main

import (
	"fmt"
	"sort"
)

type StockTracker struct {
	ts2price map[int64]int
	prices   []int
	sum      int64
}

func NewStockTracker() *StockTracker {
	return &StockTracker{ts2price: map[int64]int{}}
}

func (s *StockTracker) Add(ts int64, price int) {
	s.ts2price[ts] = price
	i := sort.SearchInts(s.prices, price)
	s.prices = append(s.prices, 0)
	copy(s.prices[i+1:], s.prices[i:])
	s.prices[i] = price
	s.sum += int64(price)
}

func (s *StockTracker) Update(ts int64, price int) {
	s.Remove(ts)
	s.Add(ts, price)
}

func (s *StockTracker) Remove(ts int64) {
	price, ok := s.ts2price[ts]
	if !ok {
		return
	}
	delete(s.ts2price, ts)
	i := sort.SearchInts(s.prices, price)
	s.prices = append(s.prices[:i], s.prices[i+1:]...)
	s.sum -= int64(price)
}

func (s *StockTracker) Max() int        { return s.prices[len(s.prices)-1] }
func (s *StockTracker) Min() int        { return s.prices[0] }
func (s *StockTracker) Average() float64 { return float64(s.sum) / float64(len(s.prices)) }

func main() {
	s := NewStockTracker()
	s.Add(1, 100); s.Add(2, 200); s.Add(3, 150)
	fmt.Printf("max=%d min=%d avg=%.1f\n", s.Max(), s.Min(), s.Average())
	s.Update(2, 50)
	fmt.Printf("max=%d min=%d avg=%.1f\n", s.Max(), s.Min(), s.Average())
	s.Remove(3)
	fmt.Printf("max=%d min=%d avg=%.1f\n", s.Max(), s.Min(), s.Average())
}
