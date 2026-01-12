// Edit distance (Levenshtein) via DP. Time O(n*m), Space O(min(n,m)).
#include <bits/stdc++.h>
using namespace std;

int editDistance(const string& a, const string& b) {
    const string &s = a.size() <= b.size() ? a : b;
    const string &t = a.size() <= b.size() ? b : a;
    int n = s.size(), m = t.size();
    vector<int> prev(n + 1), cur(n + 1);
    for (int i = 0; i <= n; ++i) prev[i] = i;
    for (int j = 1; j <= m; ++j) {
        cur[0] = j;
        for (int i = 1; i <= n; ++i) {
            int cost = s[i - 1] == t[j - 1] ? 0 : 1;
            cur[i] = min({prev[i] + 1, cur[i - 1] + 1, prev[i - 1] + cost});
        }
        swap(prev, cur);
    }
    return prev[n];
}

int main() {
    cout << editDistance("kitten", "sitting") << endl;
    return 0;
}
