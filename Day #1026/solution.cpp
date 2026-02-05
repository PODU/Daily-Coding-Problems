// Day 1026: Full text justification.
// Greedy line packing; distribute extra spaces evenly, leftover from the left.
// Time O(total chars), Space O(total chars).
#include <bits/stdc++.h>
using namespace std;

vector<string> justify(const vector<string>& words, int k) {
    vector<string> res;
    int n = words.size(), i = 0;
    while (i < n) {
        int j = i, lineLen = words[i].size();
        // pack as many words as possible: words[i..j-1]
        while (j + 1 < n && lineLen + 1 + (int)words[j + 1].size() <= k) {
            lineLen += 1 + words[j + 1].size();
            ++j;
        }
        int cnt = j - i + 1;
        string line;
        if (cnt == 1) {
            line = words[i];
            line += string(k - line.size(), ' ');
        } else {
            int totalChars = 0;
            for (int w = i; w <= j; ++w) totalChars += words[w].size();
            int spaces = k - totalChars, gaps = cnt - 1;
            int base = spaces / gaps, extra = spaces % gaps;
            for (int w = i; w <= j; ++w) {
                line += words[w];
                if (w < j) {
                    int s = base + (w - i < extra ? 1 : 0);
                    line += string(s, ' ');
                }
            }
        }
        res.push_back(line);
        i = j + 1;
    }
    return res;
}

int main() {
    vector<string> words = {"the", "quick", "brown", "fox", "jumps", "over", "the", "lazy", "dog"};
    for (auto& line : justify(words, 16)) cout << line << "\n";
}
