// Find all anagram start indices of W in S.
// Sliding window + 26-letter freq + match counter. Time O(|S|), Space O(1).
#include <bits/stdc++.h>
using namespace std;

vector<int> findAnagrams(const string& W, const string& S) {
    vector<int> res;
    int n = S.size(), m = W.size();
    if (m == 0 || m > n) return res;
    int need[26] = {0}, win[26] = {0};
    for (char c : W) need[c - 'a']++;
    int matches = 0;
    for (int i = 0; i < 26; i++) if (need[i] == 0) matches++;
    for (int i = 0; i < n; i++) {
        int add = S[i] - 'a';
        win[add]++;
        if (win[add] == need[add]) matches++;
        else if (win[add] == need[add] + 1) matches--;
        if (i >= m) {
            int rem = S[i - m] - 'a';
            win[rem]--;
            if (win[rem] == need[rem]) matches++;
            else if (win[rem] == need[rem] - 1) matches--;
        }
        if (i >= m - 1 && matches == 26) res.push_back(i - m + 1);
    }
    return res;
}

int main() {
    string W = "ab", S = "abxaba";
    vector<int> idx = findAnagrams(W, S);
    for (size_t i = 0; i < idx.size(); i++) {
        if (i) cout << ", ";
        cout << idx[i];
    }
    cout << "\n";
    return 0;
}
