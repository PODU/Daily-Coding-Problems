// Next bigger integer with the same number of set bits (Gosper's hack). O(1).
public class Solution {
    static int nextSamePopcount(int n){
        int c = n & (-n);               // lowest set bit
        int r = n + c;                  // ripple the carry
        return r | (((n ^ r) >> 2) / c); // restore trailing ones
    }

    public static void main(String[] args){
        int n = 6;                      // 0110
        System.out.println(nextSamePopcount(n)); // 9 (1001)
    }
}
