// Apply permutation where result[P[i]] = array[i] (scatter). In-place cycle-following via swaps:
// O(n) time, O(1) extra space. Also a simple O(n)-space scatter is shown for clarity.
#include <bits/stdc++.h>
using namespace std;

// In-place: follow each cycle with swaps. Each swap settles an element, so <= n swaps total.
void applyInPlace(vector<string>& a, vector<int> P) {
    int n = a.size();
    for (int i = 0; i < n; i++) {
        while (P[i] != i) {
            swap(a[i], a[P[i]]);
            swap(P[i], P[P[i]]);
        }
    }
}

// Simple reference: O(n) time, O(n) space.
vector<string> applySimple(const vector<string>& a, const vector<int>& P) {
    vector<string> res(a.size());
    for (size_t i = 0; i < a.size(); i++) res[P[i]] = a[i];
    return res;
}

int main() {
    vector<string> a = {"a", "b", "c"};
    vector<int> P = {2, 1, 0}; // result[P[i]] = a[i]
    applyInPlace(a, P);
    cout << "[";
    for (size_t i = 0; i < a.size(); i++) cout << a[i] << (i + 1 < a.size() ? ", " : "");
    cout << "]\n";
    return 0;
}
