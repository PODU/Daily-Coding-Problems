// Misere Nim (3 heaps): first player wins iff
// (some heap>1 && xor!=0) || (all heaps<=1 && xor==0). Time: O(1), Space: O(1).
public class Solution {
    static boolean firstPlayerWins(int a, int b, int c) {
        int x = a ^ b ^ c;
        boolean anyBig = a > 1 || b > 1 || c > 1;
        return anyBig ? x != 0 : x == 0;
    }

    public static void main(String[] args) {
        System.out.println(firstPlayerWins(3, 4, 5));
    }
}
