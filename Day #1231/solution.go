// Monte Carlo pi estimate with a shared 64-bit LCG. Time O(n), Space O(1).
package main

import "fmt"

const (
	A = uint64(6364136223846793005)
	C = uint64(1442695040888963407)
)

func estimatePi(samples int, seed uint64) float64 {
	x := seed
	inside := 0
	den := float64(uint64(1) << 53)
	for i := 0; i < samples; i++ {
		x = A*x + C
		px := float64(x>>11) / den
		x = A*x + C
		py := float64(x>>11) / den
		if px*px+py*py <= 1.0 {
			inside++
		}
	}
	return 4.0 * float64(inside) / float64(samples)
}

func main() {
	fmt.Printf("%.3f\n", estimatePi(2000000, 42))
}
