// Day 57: Greedy word wrap into lines of length <= k. Null if any word > k.
// Time: O(n), Space: O(n).
#include <bits/stdc++.h>
using namespace std;

// Returns false (=> null) if impossible.
bool wrap(const string& s, int k, vector<string>& out) {
    stringstream ss(s);
    string word, line;
    while (ss >> word) {
        if ((int)word.size() > k) return false;
        if (line.empty()) line = word;
        else if ((int)(line.size() + 1 + word.size()) <= k) line += " " + word;
        else { out.push_back(line); line = word; }
    }
    if (!line.empty()) out.push_back(line);
    return true;
}

int main() {
    vector<string> out;
    string s = "the quick brown fox jumps over the lazy dog";
    if (wrap(s, 10, out)) {
        cout << "[";
        for (size_t i = 0; i < out.size(); i++)
            cout << "\"" << out[i] << "\"" << (i + 1 < out.size() ? ", " : "");
        cout << "]" << endl;
    } else {
        cout << "null" << endl;
    }
    return 0;
}
