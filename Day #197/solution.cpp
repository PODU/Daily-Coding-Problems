// Day 197: Rotate array right by k in-place.
// Triple-reversal: reverse whole, reverse first k, reverse rest. O(n) time, O(1) space.
#include <bits/stdc++.h>
using namespace std;

void rotateRight(vector<int>& a, int k) {
    int n = a.size();
    if (n == 0) return;
    k %= n;
    reverse(a.begin(), a.end());
    reverse(a.begin(), a.begin() + k);
    reverse(a.begin() + k, a.end());
}

int main() {
    vector<int> a = {1, 2, 3, 4, 5};
    rotateRight(a, 2);
    cout << "[";
    for (size_t i = 0; i < a.size(); ++i) cout << a[i] << (i + 1 < a.size() ? ", " : "");
    cout << "]" << endl; // [4, 5, 1, 2, 3]
    return 0;
}
