// Min coins for US denominations {1,5,10,25} via greedy (canonical system).
// Time O(#denominations), Space O(1).
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
        System.out.println(minCoins(16));
    }
}
