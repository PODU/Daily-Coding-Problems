// Day 1324: Point-update / range-sum over a 24-element array using a Fenwick (Binary Indexed) Tree.
// update O(log n), query O(log n). 1-indexed internally over fixed size 24.
public class Solution {
    static class Subscribers {
        int[] tree = new int[25];
        void update(int hour, int value) {
            for (int i = hour + 1; i <= 24; i += i & (-i)) tree[i] += value;
        }
        int prefix(int hour) {
            int s = 0;
            for (int i = hour + 1; i > 0; i -= i & (-i)) s += tree[i];
            return s;
        }
        int query(int start, int end) {
            return prefix(end) - (start > 0 ? prefix(start - 1) : 0);
        }
    }

    public static void main(String[] args) {
        Subscribers s = new Subscribers();
        s.update(2, 10);
        s.update(5, 3);
        s.update(5, 7);
        System.out.println(s.query(2, 5));  // 20
        System.out.println(s.query(0, 23)); // 20
        System.out.println(s.query(3, 4));  // 0
    }
}
