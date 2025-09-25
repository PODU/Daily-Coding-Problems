// Simplified Elo: expected = 1/(1+10^((Rb-Ra)/400)); delta = round(K*(s-expected)); zero-sum transfer.
// Time O(1), Space O(1).
public class Solution {
    public static void main(String[] args) {
        int Ra = 1200, Rb = 2000, K = 32;
        double expectedA = 1.0 / (1.0 + Math.pow(10.0, (Rb - Ra) / 400.0));
        int delta = (int) Math.round(K * (1.0 - expectedA)); // A wins, s=1
        int newA = Ra + delta;
        int newB = Rb - delta;
        System.out.println("Winner (" + Ra + ") -> " + newA
                + ", Loser (" + Rb + ") -> " + newB);
    }
}
