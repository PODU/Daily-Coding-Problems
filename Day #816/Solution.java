// Branchless select via bitwise mask: y ^ ((-b) & (x ^ y)). O(1) time, O(1) space.
public class Solution {
    static int select(int x, int y, int b) {
        return y ^ ((-b) & (x ^ y));
    }

    public static void main(String[] args) {
        System.out.println("b=1 -> " + select(5, 9, 1));
        System.out.println("b=0 -> " + select(5, 9, 0));
    }
}
