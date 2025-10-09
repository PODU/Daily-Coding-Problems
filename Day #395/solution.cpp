// Group anagrams: hash map keyed by sorted chars -> list, preserving first-seen group order.
// Time O(N*K log K), Space O(N*K).
#include <bits/stdc++.h>
using namespace std;

vector<vector<string>> groupAnagrams(const vector<string>& words) {
    unordered_map<string, int> idx;
    vector<vector<string>> groups;
    for (const string& w : words) {
        string key = w;
        sort(key.begin(), key.end());
        auto it = idx.find(key);
        if (it == idx.end()) {
            idx[key] = (int)groups.size();
            groups.push_back({w});
        } else {
            groups[it->second].push_back(w);
        }
    }
    return groups;
}

int main() {
    vector<string> input = {"eat", "ate", "apt", "pat", "tea", "now"};
    auto groups = groupAnagrams(input);
    cout << "[";
    for (size_t i = 0; i < groups.size(); ++i) {
        cout << "[";
        for (size_t j = 0; j < groups[i].size(); ++j) {
            cout << "'" << groups[i][j] << "'";
            if (j + 1 < groups[i].size()) cout << ", ";
        }
        cout << "]";
        if (i + 1 < groups.size()) cout << ", ";
    }
    cout << "]" << endl;
    return 0;
}
