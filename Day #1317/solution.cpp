// Reconstruct a permutation of [0..N] consistent with up/down signs.
// Two-pointer (DI-match): '+' takes next low, '-' takes next high. Time O(N).
// Any consistent array is valid; README shows one such answer.
#include <bits/stdc++.h>
using namespace std;

// signs[0] is the sentinel (None); signs[i] in {'+','-'} for i>=1.
vector<int> reconstruct(const vector<char>& signs) {
    int n = signs.size(), N = n - 1;
    vector<int> res;
    int lo = 0, hi = N;
    for (int i = 1; i < n; i++) {
        if (signs[i] == '+') res.push_back(lo++); // element i-1 is the small end
        else                 res.push_back(hi--);
    }
    res.push_back(lo); // lo == hi for the final element
    return res;
}

// Verify the array obeys the signs.
bool consistent(const vector<char>& s, const vector<int>& a) {
    for (size_t i = 1; i < s.size(); i++) {
        if (s[i] == '+' && !(a[i] > a[i-1])) return false;
        if (s[i] == '-' && !(a[i] < a[i-1])) return false;
    }
    return true;
}

int main() {
    vector<char> signs = {'#', '+', '+', '-', '+'}; // '#' stands in for None
    vector<int> a = reconstruct(signs);
    cout << "[";
    for (size_t i = 0; i < a.size(); i++) cout << a[i] << (i+1<a.size()?", ":"");
    cout << "]  consistent=" << (consistent(signs, a) ? "true" : "false") << "\n";
    // -> [0, 1, 4, 2, 3]  (README's [1, 2, 3, 0, 4] is another valid answer)
    return 0;
}
