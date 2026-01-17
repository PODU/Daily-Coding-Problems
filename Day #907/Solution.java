// Reflected binary Gray code: value i -> i ^ (i>>1) for i in 0..2^n. O(2^n) time/space.
public class Solution {
    public static void main(String[] args) {
        int n = 2;
        int total = 1 << n;
        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < total; i++) {
            int g = i ^ (i >> 1);
            StringBuilder code = new StringBuilder();
            for (int b = n - 1; b >= 0; b--) code.append(((g >> b) & 1) == 1 ? '1' : '0');
            sb.append(code);
            if (i + 1 < total) sb.append(", ");
        }
        sb.append("]");
        System.out.println(sb.toString());
    }
}
