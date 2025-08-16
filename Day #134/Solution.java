// Day 134: SparseArray storing only non-zero entries in a hash map.
// init O(n) once, set/get O(1) average. Space O(#nonzero).
import java.util.*;

public class Solution {
    static class SparseArray {
        Map<Integer, Integer> data = new HashMap<>();
        int size;

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
        sa.init(new int[]{0, 0, 7, 0, 0, 0, 3, 0}, 8);
        System.out.println(sa.get(2)); // 7
        System.out.println(sa.get(0)); // 0
        sa.set(0, 5);
        System.out.println(sa.get(0)); // 5
        sa.set(2, 0);
        System.out.println(sa.get(2)); // 0
    }
}
