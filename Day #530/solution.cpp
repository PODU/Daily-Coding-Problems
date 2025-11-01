// Levenshtein edit distance via DP with rolling array.
// Time O(m*n), Space O(min(m,n)).
#include <bits/stdc++.h>
using namespace std;

int editDistance(const string& a, const string& b) {
    if (a.size() < b.size()) return editDistance(b, a);
    int n = b.size();
    vector<int> prev(n + 1), cur(n + 1);
    for (int j = 0; j <= n; ++j) prev[j] = j;
    for (int i = 1; i <= (int)a.size(); ++i) {
        cur[0] = i;
        for (int j = 1; j <= n; ++j) {
            int cost = (a[i - 1] == b[j - 1]) ? 0 : 1;
            cur[j] = min({prev[j] + 1, cur[j - 1] + 1, prev[j - 1] + cost});
        }
        swap(prev, cur);
    }
    return prev[n];
}

int main() {
    cout << editDistance("kitten", "sitting") << "\n";
    return 0;
}
