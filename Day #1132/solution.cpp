// Left-rotate array in place by k using 3 reversals: reverse[0,k), reverse[k,n), reverse[0,n).
// O(n) time, O(1) space.
#include <bits/stdc++.h>
using namespace std;

void reverseRange(vector<int>& a, int i, int j) {
    while (i < j) swap(a[i++], a[j--]);
}

void rotateLeft(vector<int>& a, int k) {
    int n = a.size();
    if (n == 0) return;
    k %= n;
    reverseRange(a, 0, k - 1);
    reverseRange(a, k, n - 1);
    reverseRange(a, 0, n - 1);
}

int main() {
    vector<int> a = {1, 2, 3, 4, 5, 6};
    rotateLeft(a, 2);
    cout << "[";
    for (size_t i = 0; i < a.size(); ++i) {
        if (i) cout << ", ";
        cout << a[i];
    }
    cout << "]" << endl;
    return 0;
}
