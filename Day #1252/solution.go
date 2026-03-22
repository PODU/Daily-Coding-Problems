// Clock angle: minute=mm*6, hour=(hh%12)*30+mm*0.5, take min(diff,360-diff), round. O(1) time/space.
package main

import (
	"fmt"
	"math"
)

func clockAngle(hh, mm int) int {
	minute := float64(mm) * 6.0
	hour := float64(hh%12)*30.0 + float64(mm)*0.5
	diff := math.Abs(hour - minute)
	diff = math.Min(diff, 360.0-diff)
	return int(math.Floor(diff + 0.5))
}

func main() {
	fmt.Println(clockAngle(12, 30))
	fmt.Println(clockAngle(3, 15))
}
