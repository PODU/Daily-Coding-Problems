// Day 1471: O(log N) search in a sorted array with no *, /, or bit-shift.
// Build powers of two by doubling (addition); jump-based binary search.
// Time O(log N), Space O(log N).
#include <bits/stdc++.h>
using namespace std;

bool search(const vector<int>& a, int x) {
    int n = (int)a.size();
    if (n == 0) return false;
    vector<int> powers;
    for (int p = 1; p <= n; p = p + p) powers.push_back(p);
    int pos = -1;
    for (int i = (int)powers.size() - 1; i >= 0; --i) {
        int nxt = pos + powers[i];
        if (nxt < n && a[nxt] <= x) pos = nxt;
    }
    return pos >= 0 && a[pos] == x;
}

int main() {
    vector<int> arr = {1, 3, 5, 7, 9, 11};
    cout << (search(arr, 7) ? "True" : "False") << "\n";
    cout << (search(arr, 8) ? "True" : "False") << "\n";
}
