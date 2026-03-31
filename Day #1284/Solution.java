// Day 1284: In a row/col-sorted matrix, count elements < M[i1][j1] plus those >= M[i2][j2].
// Binary search per (ascending) row. Time O(N log M), Space O(1).
// Note: upper bound is inclusive (>=) so the sample yields 15 as specified.
public class Solution {
    static int lowerBound(int[] arr, int key) {
        int lo = 0, hi = arr.length;
        while (lo < hi) {
            int mid = (lo + hi) >>> 1;
            if (arr[mid] < key) lo = mid + 1;
            else hi = mid;
        }
        return lo;
    }

    static int countOutside(int[][] M, int i1, int j1, int i2, int j2) {
        int low = M[i1][j1], high = M[i2][j2];
        int total = 0;
        for (int[] row : M) {
            total += lowerBound(row, low);              // count < low
            total += row.length - lowerBound(row, high); // count >= high
        }
        return total;
    }

    public static void main(String[] args) {
        int[][] M = {
            {1, 3, 7, 10, 15, 20},
            {2, 6, 9, 14, 22, 25},
            {3, 8, 10, 15, 25, 30},
            {10, 11, 12, 23, 30, 35},
            {20, 25, 30, 35, 40, 45}};
        System.out.println(countOutside(M, 1, 1, 3, 3)); // 15
    }
}
