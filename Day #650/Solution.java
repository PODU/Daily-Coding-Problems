// Per-row binary search: count elements < A[i1][j1] (lowerBound) plus elements >= A[i2][j2] (M - lowerBound).
// Upper bound taken inclusive (>=) to match reference example. Time O(N log M), space O(1).
public class Solution {
    // index of first element >= key
    static int lowerBound(int[] row, int key) {
        int lo = 0, hi = row.length;
        while (lo < hi) {
            int mid = (lo + hi) >>> 1;
            if (row[mid] < key) lo = mid + 1; else hi = mid;
        }
        return lo;
    }

    public static void main(String[] args) {
        int[][] A = {
            {1, 3, 7, 10, 15, 20},
            {2, 6, 9, 14, 22, 25},
            {3, 8, 10, 15, 25, 30},
            {10, 11, 12, 23, 30, 35},
            {20, 25, 30, 35, 40, 45}
        };
        int i1 = 1, j1 = 1, i2 = 3, j2 = 3;
        int pivot1 = A[i1][j1]; // 6
        int pivot2 = A[i2][j2]; // 23
        int M = A[0].length;
        long countSmaller = 0, countUpper = 0;
        for (int[] row : A) {
            countSmaller += lowerBound(row, pivot1);
            countUpper += M - lowerBound(row, pivot2);
        }
        System.out.println(countSmaller + countUpper);
    }
}
