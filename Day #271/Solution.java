// Day 271: Search sorted list without *, /, or bit-shift -> Fibonacci search.
// Uses only addition/subtraction/comparison. Time O(log N), Space O(1).
public class Solution {
    static int fibSearch(int[] arr, int x) {
        int n = arr.length;
        int fibMm2 = 0, fibMm1 = 1, fibM = fibMm2 + fibMm1;
        while (fibM < n) {
            fibMm2 = fibMm1;
            fibMm1 = fibM;
            fibM = fibMm2 + fibMm1;
        }
        int offset = -1;
        while (fibM > 1) {
            int i = Math.min(offset + fibMm2, n - 1);
            if (arr[i] < x) {
                fibM = fibMm1; fibMm1 = fibMm2; fibMm2 = fibM - fibMm1;
                offset = i;
            } else if (arr[i] > x) {
                fibM = fibMm2; fibMm1 = fibMm1 - fibMm2; fibMm2 = fibM - fibMm1;
            } else {
                return i;
            }
        }
        if (fibMm1 != 0 && offset + 1 < n && arr[offset + 1] == x) return offset + 1;
        return -1;
    }

    public static void main(String[] args) {
        int[] arr = {1, 3, 4, 7, 9, 11, 15};
        System.out.println("7 -> index " + fibSearch(arr, 7)); // 3
        System.out.println("8 -> index " + fibSearch(arr, 8)); // -1
    }
}
