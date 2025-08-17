// Greedy on canonical US denominations {25,10,5,1}: take largest coin each step.
// Time O(D) where D = #denominations; Space O(1).
public class Solution {
    static int minCoins(int n) {
        int[] coins = {25, 10, 5, 1};
        int count = 0;
        for (int c : coins) {
            count += n / c;
            n %= c;
        }
        return count;
    }

    public static void main(String[] args) {
        System.out.println(minCoins(16)); // 3
    }
}
