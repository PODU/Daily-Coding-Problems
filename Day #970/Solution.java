// Day 970: Space-efficient SparseArray storing only non-zero entries.
// Approach: hash map of index->value, default 0. Time O(1) avg per op, Space O(#nonzero).
import java.util.*;

public class Solution {
    static class SparseArray {
        Map<Integer, Integer> m = new HashMap<>();
        int size;
        void init(int[] arr, int sz) {
            size = sz;
            m.clear();
            for (int i = 0; i < sz; i++) if (arr[i] != 0) m.put(i, arr[i]);
        }
        void set(int i, int val) {
            if (i < 0 || i >= size) throw new IndexOutOfBoundsException();
            if (val == 0) m.remove(i); else m.put(i, val);
        }
        int get(int i) {
            if (i < 0 || i >= size) throw new IndexOutOfBoundsException();
            return m.getOrDefault(i, 0);
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
