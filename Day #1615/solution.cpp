// Rotate array right by k via three reversals: reverse all, reverse first k, reverse rest.
// Time: O(n), Space: O(1).
#include <bits/stdc++.h>
using namespace std;

void rotate(vector<int>& a, int k) {
    int n = a.size();
    if (n == 0) return;
    k %= n;
    reverse(a.begin(), a.end());
    reverse(a.begin(), a.begin() + k);
    reverse(a.begin() + k, a.end());
}

int main() {
    vector<int> a = {1, 2, 3, 4, 5, 6, 7};
    rotate(a, 3);
    for (size_t i = 0; i < a.size(); ++i) {
        cout << a[i];
        if (i + 1 < a.size()) cout << ' ';
    }
    cout << endl;
    return 0;
}
