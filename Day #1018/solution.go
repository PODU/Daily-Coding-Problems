// Clock angle: O(1) time, O(1) space.
// minute=mm*6, hour=(hh%12)*30+mm*0.5, diff=|h-m|, angle=min(diff,360-diff), rounded.
package main

import (
	"fmt"
	"math"
)

func clockAngle(hh, mm int) int {
	minute := float64(mm) * 6.0
	hour := float64(hh%12)*30.0 + float64(mm)*0.5
	diff := math.Abs(hour - minute)
	angle := math.Min(diff, 360.0-diff)
	return int(math.Round(angle))
}

func main() {
	fmt.Printf("%02d:%02d -> %d\n", 3, 30, clockAngle(3, 30))
	fmt.Printf("%02d:%02d -> %d\n", 12, 0, clockAngle(12, 0))
}
