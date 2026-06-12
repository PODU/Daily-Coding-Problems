// Bijective base-26: while n>0, n--, prepend 'A'+(n%26), n/=26. O(log n) time, O(log n) space.
public class Solution {
    static String columnTitle(long n) {
        StringBuilder sb = new StringBuilder();
        while (n > 0) {
            n--;
            sb.append((char) ('A' + (n % 26)));
            n /= 26;
        }
        return sb.reverse().toString();
    }

    public static void main(String[] args) {
        System.out.println(columnTitle(1));
        System.out.println(columnTitle(27));
    }
}
