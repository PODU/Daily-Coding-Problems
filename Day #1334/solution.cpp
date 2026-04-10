// Day 1334: Levenshtein edit distance between two strings.
// Classic DP with rolling row. O(n*m) time, O(min(n,m)) space.
#include <bits/stdc++.h>
using namespace std;

int editDistance(const string& a, const string& b) {
    int n = a.size(), m = b.size();
    vector<int> prev(m + 1), cur(m + 1);
    for (int j = 0; j <= m; j++) prev[j] = j;
    for (int i = 1; i <= n; i++) {
        cur[0] = i;
        for (int j = 1; j <= m; j++) {
            if (a[i - 1] == b[j - 1]) cur[j] = prev[j - 1];
            else cur[j] = 1 + min({prev[j - 1], prev[j], cur[j - 1]});
        }
        swap(prev, cur);
    }
    return prev[m];
}

int main() {
    cout << editDistance("kitten", "sitting") << endl; // 3
    return 0;
}
