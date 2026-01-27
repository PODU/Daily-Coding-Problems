// Day 972: Rearrange string so no two adjacent chars match (else "None").
// Approach: place most frequent chars into even-then-odd slots. Time O(n log k), Space O(n).
#include <bits/stdc++.h>
using namespace std;

string rearrange(const string& s) {
    int n = s.size();
    unordered_map<char,int> cnt;
    for (char c : s) cnt[c]++;
    vector<pair<int,char>> v;
    for (auto& p : cnt) v.push_back({p.second, p.first});
    sort(v.rbegin(), v.rend());
    if (v[0].first > (n + 1) / 2) return "None";
    string res(n, ' ');
    int idx = 0;
    for (auto& p : v) {
        for (int k = 0; k < p.first; ++k) {
            res[idx] = p.second;
            idx += 2;
            if (idx >= n) idx = 1;
        }
    }
    return res;
}

int main() {
    cout << rearrange("aaabbc") << endl; // ababac
    cout << rearrange("aaab") << endl;   // None
    return 0;
}
