// Sliding window with last-seen index map; advance left past duplicates, track max window length.
// Time: O(n), Space: O(n).
#include <bits/stdc++.h>
using namespace std;

int longestDistinct(const vector<int>& a) {
    unordered_map<int, int> last;
    int best = 0, left = 0;
    for (int r = 0; r < (int)a.size(); r++) {
        auto it = last.find(a[r]);
        if (it != last.end() && it->second >= left) left = it->second + 1;
        last[a[r]] = r;
        best = max(best, r - left + 1);
    }
    return best;
}

int main() {
    vector<int> a = {5, 1, 3, 5, 2, 3, 4, 1};
    cout << longestDistinct(a) << endl;
    return 0;
}
