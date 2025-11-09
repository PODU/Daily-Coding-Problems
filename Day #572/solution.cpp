// Next greater permutation in-place (lexicographic). If none, wrap to smallest.
// Approach: find pivot, successor swap, reverse suffix. Time O(n), Space O(1).
#include <bits/stdc++.h>
using namespace std;

void nextPermutation(vector<int>& a) {
    int n = a.size(), i = n - 2;
    while (i >= 0 && a[i] >= a[i + 1]) --i;
    if (i >= 0) {
        int j = n - 1;
        while (a[j] <= a[i]) --j;
        swap(a[i], a[j]);
    }
    reverse(a.begin() + i + 1, a.end());
}

void run(vector<int> a) {
    nextPermutation(a);
    cout << '[';
    for (size_t k = 0; k < a.size(); ++k) cout << a[k] << (k + 1 < a.size() ? "," : "");
    cout << "]\n";
}

int main() {
    run({1, 2, 3});
    run({1, 3, 2});
    run({3, 2, 1});
    return 0;
}
