// Day 758: Rotate a list left by k in place using the 3-reversal trick.
// No copy; ~n swaps total (each reversal swaps floor(len/2) pairs).
// Time: O(n), Space: O(1).
#include <bits/stdc++.h>
using namespace std;

int reverseRange(vector<int>& a, int i, int j) {
    int swaps = 0;
    while (i < j) { swap(a[i++], a[j--]); swaps++; }
    return swaps;
}

int rotateLeft(vector<int>& a, int k) {
    int n = a.size();
    if (n == 0) return 0;
    k %= n;
    int swaps = 0;
    swaps += reverseRange(a, 0, k - 1);
    swaps += reverseRange(a, k, n - 1);
    swaps += reverseRange(a, 0, n - 1);
    return swaps;
}

int main() {
    vector<int> a = {1, 2, 3, 4, 5, 6};
    int swaps = rotateLeft(a, 2);
    cout << "[";
    for (size_t i = 0; i < a.size(); ++i)
        cout << a[i] << (i + 1 < a.size() ? ", " : "");
    cout << "]\n";                       // [3, 4, 5, 6, 1, 2]
    cout << "swaps: " << swaps << "\n";
    return 0;
}
