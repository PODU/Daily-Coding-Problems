// Minimum coins for {1,5,10,25} via greedy (optimal for this canonical system).
// Time: O(1), Space: O(1).
public class Solution {
    static int minCoins(int n) {
        return n/25 + (n%25)/10 + (n%25%10)/5 + (n%25%10%5);
    }

    public static void main(String[] args) {
        System.out.println(minCoins(16));
    }
}
