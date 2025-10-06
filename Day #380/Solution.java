// Integer division without '/': repeated doubling/subtraction.
// Time: O(log n), Space: O(1).
public class Solution {
    static long[] divide(long a, long b){
        boolean neg = (a < 0) != (b < 0);
        long x = Math.abs(a), y = Math.abs(b), q = 0;
        while(x >= y){
            long temp = y, mult = 1;
            while(x >= (temp << 1)){ temp <<= 1; mult <<= 1; }
            x -= temp; q += mult;
        }
        return new long[]{ neg ? -q : q, x };
    }

    public static void main(String[] args){
        long[] r = divide(10, 3);
        System.out.println("(" + r[0] + ", " + r[1] + ")");
    }
}
