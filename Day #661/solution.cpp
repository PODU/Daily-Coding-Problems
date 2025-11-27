// Day 661: Search sorted array without mul/div/bit-shift.
// Fibonacci search uses only +/- to pick probe points. Time O(log N), Space O(1).
#include <bits/stdc++.h>
using namespace std;

int fibSearch(const vector<int>& a, int x) {
    int n = (int)a.size();
    int f2 = 0, f1 = 1, f = f1 + f2;
    while (f < n) { f2 = f1; f1 = f; f = f1 + f2; }
    int offset = -1;
    while (f > 1) {
        int i = min(offset + f2, n - 1);
        if (a[i] < x) { f = f1; f1 = f2; f2 = f - f1; offset = i; }
        else if (a[i] > x) { f = f2; f1 = f1 - f2; f2 = f - f1; }
        else return i;
    }
    if (f1 && offset + 1 < n && a[offset + 1] == x) return offset + 1;
    return -1;
}

int main() {
    vector<int> a = {-1, 0, 3, 5, 9, 12};
    cout << "Index of 5: " << fibSearch(a, 5) << "\n";   // 3
    cout << "Index of 7: " << fibSearch(a, 7) << "\n";   // -1 (absent)
    return 0;
}
