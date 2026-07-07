// Minimum coins for {1,5,10,25} via greedy (optimal for this canonical system).
// Time: O(1), Space: O(1).
package main

import "fmt"

func minCoins(n int) int {
	return n/25 + (n%25)/10 + (n%25%10)/5 + (n%25%10%5)
}

func main() {
	fmt.Println(minCoins(16))
}
