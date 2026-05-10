// Next greater permutation in-place. Standard next_permutation.
// Time O(n), Space O(1).
#include <bits/stdc++.h>
using namespace std;

void nextPermutation(vector<int>& a) {
    int n = a.size(), i = n - 2;
    while (i >= 0 && a[i] >= a[i + 1]) i--;
    if (i >= 0) {
        int j = n - 1;
        while (a[j] <= a[i]) j--;
        swap(a[i], a[j]);
    }
    reverse(a.begin() + i + 1, a.end());
}

string fmt(const vector<int>& a) {
    string s = "[";
    for (size_t i = 0; i < a.size(); i++) {
        if (i) s += ", ";
        s += to_string(a[i]);
    }
    return s + "]";
}

int main() {
    vector<vector<int>> cases = {{1,2,3},{1,3,2},{3,2,1}};
    for (auto c : cases) {
        nextPermutation(c);
        cout << fmt(c) << "\n";
    }
    return 0;
}
