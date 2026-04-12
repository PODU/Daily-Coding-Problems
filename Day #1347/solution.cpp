// Reorganize string: count freqs, if max > (n+1)/2 impossible; sort chars by freq desc (tie by char),
// fill even indices first then odd. Time O(n log k), Space O(n).
#include <bits/stdc++.h>
using namespace std;

string reorganize(const string& s) {
    int n = s.size();
    map<char,int> cnt;
    for (char c : s) cnt[c]++;
    for (auto& p : cnt) if (p.second > (n + 1) / 2) return "";
    vector<pair<int,char>> v;
    for (auto& p : cnt) v.push_back({p.second, p.first});
    sort(v.begin(), v.end(), [](const pair<int,char>& a, const pair<int,char>& b){
        if (a.first != b.first) return a.first > b.first; // freq desc
        return a.second < b.second;                       // tie by char asc
    });
    string res(n, ' ');
    int idx = 0;
    for (auto& p : v) {
        for (int k = 0; k < p.first; k++) {
            res[idx] = p.second;
            idx += 2;
            if (idx >= n) idx = 1;
        }
    }
    return res;
}

int main() {
    string r1 = reorganize("aaabbc");
    cout << (r1.empty() ? "None" : r1) << "\n";
    string r2 = reorganize("aaab");
    cout << (r2.empty() ? "None" : r2) << "\n";
    return 0;
}
