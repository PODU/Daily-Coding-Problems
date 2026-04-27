// Day 1430: Space-efficient SparseArray for a mostly-zero array.
// Approach: store only non-zero indices in a HashMap. Time: O(1) avg per op, Space: O(#nonzero).
import java.util.HashMap;
import java.util.Map;

public class Solution {
    static class SparseArray {
        private final Map<Integer, Integer> data = new HashMap<>();
        private int n = 0;

        void init(int[] arr, int size) {
            n = size;
            data.clear();
            for (int i = 0; i < size; i++)
                if (arr[i] != 0) data.put(i, arr[i]);
        }

        void set(int i, int val) {
            if (i < 0 || i >= n) throw new IndexOutOfBoundsException();
            if (val == 0) data.remove(i);
            else data.put(i, val);
        }

        int get(int i) {
            if (i < 0 || i >= n) throw new IndexOutOfBoundsException();
            return data.getOrDefault(i, 0);
        }
    }

    public static void main(String[] args) {
        SparseArray sa = new SparseArray();
        sa.init(new int[]{0, 0, 5, 0, 0, 0, 9, 0}, 8);
        System.out.println(sa.get(2)); // 5
        System.out.println(sa.get(3)); // 0
        sa.set(3, 7);
        System.out.println(sa.get(3)); // 7
        sa.set(2, 0);
        System.out.println(sa.get(2)); // 0
    }
}
