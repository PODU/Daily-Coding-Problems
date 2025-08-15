// Day 126: Rotate list left by k in-place via 3 reversals.
// O(n) time, O(1) extra space, ~n swaps total.
#include <bits/stdc++.h>
using namespace std;

void reverse(vector<int>& a, int i, int j) {
    while (i < j) swap(a[i++], a[j--]);
}

void rotateLeft(vector<int>& a, int k) {
    int n = a.size();
    if (n == 0) return;
    k %= n;
    reverse(a, 0, k - 1);
    reverse(a, k, n - 1);
    reverse(a, 0, n - 1);
}

int main() {
    vector<int> a = {1, 2, 3, 4, 5, 6};
    rotateLeft(a, 2);
    cout << "[";
    for (size_t i = 0; i < a.size(); i++) cout << a[i] << (i + 1 < a.size() ? ", " : "");
    cout << "]" << endl;
    return 0;
}
