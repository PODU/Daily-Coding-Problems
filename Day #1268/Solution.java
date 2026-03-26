// Day 1268: Select x if b==1 else y using only arithmetic/bit ops (no branches).
// y ^ ((x ^ y) & -b): -b is all-ones when b==1, all-zeros when b==0. O(1).
public class Solution {
    static int select(int x, int y, int b) {
        return y ^ ((x ^ y) & (-b));
    }

    public static void main(String[] args) {
        int x = 5, y = 10;
        System.out.println("b=1 -> " + select(x, y, 1)); // 5
        System.out.println("b=0 -> " + select(x, y, 0)); // 10
    }
}
