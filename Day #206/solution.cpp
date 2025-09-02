// Day 206: Apply permutation P to array (result[P[i]] = a[i]).
// In-place via cycle following on the permutation. Time: O(n), Space: O(1).
#include <bits/stdc++.h>
using namespace std;

void applyPermutation(vector<string>& a, vector<int> p) {
    int n = a.size();
    for (int i = 0; i < n; ++i) {
        // Move element into place following the cycle until p[i] == i.
        while (p[i] != i) {
            swap(a[i], a[p[i]]);
            swap(p[i], p[p[i]]);
        }
    }
}

int main() {
    vector<string> a = {"a", "b", "c"};
    applyPermutation(a, {2, 1, 0});
    cout << "[";
    for (size_t i = 0; i < a.size(); ++i) cout << '"' << a[i] << '"' << (i + 1 < a.size() ? ", " : "");
    cout << "]" << endl; // ["c", "b", "a"]
    return 0;
}
