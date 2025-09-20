// Day 303: Angle between clock hands. O(1).
// Bonus: hands overlap (angle 0) every 12/11 hours (~65.45 min apart).
package main

import (
	"fmt"
	"math"
)

func clockAngle(h, m int) int {
	hr := float64(h%12)*30.0 + float64(m)*0.5
	mn := float64(m) * 6.0
	diff := math.Abs(hr - mn)
	diff = math.Min(diff, 360.0-diff)
	return int(math.Round(diff))
}

func main() {
	fmt.Println(clockAngle(3, 30)) // 75
}
