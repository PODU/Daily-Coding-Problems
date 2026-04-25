// Day 1420: wrap words into lines of length <= k, greedily packing max words/line.
// Approach: greedy single pass over words. O(n) time, O(n) space. null if a word > k.
#include <bits/stdc++.h>
using namespace std;

bool wrap(const string& s, int k, vector<string>& out) {
    stringstream ss(s);
    string word, line;
    while (ss >> word) {
        if ((int)word.size() > k) return false; // impossible
        if (line.empty()) line = word;
        else if ((int)line.size() + 1 + (int)word.size() <= k) line += " " + word;
        else { out.push_back(line); line = word; }
    }
    if (!line.empty()) out.push_back(line);
    return true;
}

int main() {
    string s = "the quick brown fox jumps over the lazy dog";
    vector<string> out;
    if (wrap(s, 10, out)) {
        cout << "[";
        for (size_t i = 0; i < out.size(); ++i)
            cout << "\"" << out[i] << "\"" << (i + 1 < out.size() ? ", " : "");
        cout << "]\n";
    } else cout << "null\n";
    return 0;
}
