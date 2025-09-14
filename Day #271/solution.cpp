// Day 271: Search sorted list without *, /, or bit-shift -> Fibonacci search.
// Uses only addition/subtraction/comparison. Time O(log N), Space O(1).
#include <bits/stdc++.h>
using namespace std;

int fibSearch(const vector<int>& arr, int x) {
    int n = (int)arr.size();
    int fibMm2 = 0;          // (m-2)'th Fibonacci
    int fibMm1 = 1;          // (m-1)'th Fibonacci
    int fibM = fibMm2 + fibMm1; // m'th Fibonacci
    while (fibM < n) {
        fibMm2 = fibMm1;
        fibMm1 = fibM;
        fibM = fibMm2 + fibMm1;
    }
    int offset = -1;
    while (fibM > 1) {
        int i = min(offset + fibMm2, n - 1);
        if (arr[i] < x) {
            fibM = fibMm1; fibMm1 = fibMm2; fibMm2 = fibM - fibMm1;
            offset = i;
        } else if (arr[i] > x) {
            fibM = fibMm2; fibMm1 = fibMm1 - fibMm2; fibMm2 = fibM - fibMm1;
        } else {
            return i;
        }
    }
    if (fibMm1 && offset + 1 < n && arr[offset + 1] == x) return offset + 1;
    return -1;
}

int main() {
    vector<int> arr = {1, 3, 4, 7, 9, 11, 15};
    cout << "7 -> index " << fibSearch(arr, 7) << "\n";  // 3
    cout << "8 -> index " << fibSearch(arr, 8) << "\n";  // -1
    return 0;
}
