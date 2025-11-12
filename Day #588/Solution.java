// Day 588: Space-efficient SparseArray backed by a hash map of non-zero entries.
// Approach: store only non-zero indices. Time O(1) avg per op, Space O(#nonzero).
import java.util.HashMap;
import java.util.Map;

public class Solution {
    static class SparseArray {
        private final Map<Integer, Integer> data = new HashMap<>();
        private int size;

        void init(int[] arr, int sz) {
            size = sz;
            data.clear();
            for (int i = 0; i < arr.length && i < sz; i++)
                if (arr[i] != 0) data.put(i, arr[i]);
        }

        void set(int i, int val) {
            if (i < 0 || i >= size) throw new IndexOutOfBoundsException();
            if (val == 0) data.remove(i);
            else data.put(i, val);
        }

        int get(int i) {
            if (i < 0 || i >= size) throw new IndexOutOfBoundsException();
            return data.getOrDefault(i, 0);
        }
    }

    public static void main(String[] args) {
        SparseArray sa = new SparseArray();
        sa.init(new int[]{0, 0, 0, 5, 0, 0, 9, 0}, 8);
        System.out.println("get(3) = " + sa.get(3)); // 5
        System.out.println("get(6) = " + sa.get(6)); // 9
        System.out.println("get(0) = " + sa.get(0)); // 0
        sa.set(1, 42);
        System.out.println("after set(1,42), get(1) = " + sa.get(1)); // 42
        sa.set(3, 0);
        System.out.println("after set(3,0), get(3) = " + sa.get(3)); // 0
    }
}
