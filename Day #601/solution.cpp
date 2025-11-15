// Day 601: Reconstruct a permutation of [0..N] matching +/- relations between neighbors.
// Approach: greedy low/high pointers (DI-match). Time O(n), Space O(n). Any consistent array is valid.
#include <bits/stdc++.h>
using namespace std;

// signs[0] is the placeholder (None); signs[k] (k>=1) is '+' if a[k] > a[k-1], else '-'.
vector<int> reconstruct(const vector<char>& signs) {
    int n = signs.size();          // numbers are 0..n-1 (N = n-1)
    int low = 0, high = n - 1;
    vector<int> res;
    for (int k = 1; k < n; k++) {
        if (signs[k] == '+') res.push_back(low++);
        else res.push_back(high--);
    }
    res.push_back(low);            // low == high here
    return res;
}

int main() {
    // [None, +, +, -, +]
    vector<char> signs = {' ', '+', '+', '-', '+'};
    vector<int> a = reconstruct(signs);
    cout << "[";
    for (size_t i = 0; i < a.size(); i++)
        cout << a[i] << (i + 1 < a.size() ? ", " : "");
    cout << "]\n";
    return 0;
}
