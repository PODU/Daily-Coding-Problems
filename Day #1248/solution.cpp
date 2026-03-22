// Concatenation substring indices via sliding window over wordLen offsets with hashmap counts. O(n*wordLen) time, O(m) space.
#include <bits/stdc++.h>
using namespace std;

vector<int> findSubstring(const string& s, const vector<string>& words) {
    vector<int> res;
    if (words.empty() || s.empty()) return res;
    int wordLen = words[0].size();
    int numWords = words.size();
    int windowLen = wordLen * numWords;
    if ((int)s.size() < windowLen) return res;

    unordered_map<string, int> need;
    for (auto& w : words) need[w]++;

    for (int offset = 0; offset < wordLen; ++offset) {
        unordered_map<string, int> window;
        int count = 0;
        int left = offset;
        for (int right = offset; right + wordLen <= (int)s.size(); right += wordLen) {
            string word = s.substr(right, wordLen);
            if (need.count(word)) {
                window[word]++;
                count++;
                while (window[word] > need[word]) {
                    string lw = s.substr(left, wordLen);
                    window[lw]--;
                    count--;
                    left += wordLen;
                }
                if (count == numWords) {
                    res.push_back(left);
                    string lw = s.substr(left, wordLen);
                    window[lw]--;
                    count--;
                    left += wordLen;
                }
            } else {
                window.clear();
                count = 0;
                left = right + wordLen;
            }
        }
    }
    sort(res.begin(), res.end());
    return res;
}

int main() {
    string s = "dogcatcatcodecatdog";
    vector<string> words = {"cat", "dog"};
    vector<int> res = findSubstring(s, words);
    cout << "[";
    for (size_t i = 0; i < res.size(); ++i) {
        cout << res[i];
        if (i + 1 < res.size()) cout << ", ";
    }
    cout << "]" << endl;
    return 0;
}
