// Rearrange string so no two adjacent chars equal.
// Greedy: place chars by descending frequency into even indices then odd indices.
// Time O(n + k log k), Space O(n).
#include <bits/stdc++.h>
using namespace std;

string reorganize(const string& s) {
    int n = s.size();
    int cnt[256] = {0};
    for (unsigned char c : s) cnt[c]++;
    int maxc = 0;
    for (int i = 0; i < 256; i++) maxc = max(maxc, cnt[i]);
    if (maxc > (n + 1) / 2) return string("\0", 1); // sentinel = None
    vector<pair<int,char>> v;
    for (int i = 0; i < 256; i++) if (cnt[i]) v.push_back({cnt[i], (char)i});
    sort(v.begin(), v.end(), [](const pair<int,char>&a, const pair<int,char>&b){
        return a.first > b.first;
    });
    string res(n, ' ');
    int idx = 0;
    for (auto& p : v)
        for (int j = 0; j < p.first; j++) {
            res[idx] = p.second;
            idx += 2;
            if (idx >= n) idx = 1;
        }
    return res;
}

int main() {
    string r = reorganize("aaabbc");
    cout << (r.size() == 1 && r[0] == '\0' ? "None" : r) << "\n";
    return 0;
}
