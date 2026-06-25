// Day 1717: Fully justify text into lines of length k.
// Greedy line packing + even space distribution (extras from left).
// Time: O(total characters), Space: O(output).
#include <bits/stdc++.h>
using namespace std;

vector<string> justify(const vector<string>& words, int k) {
    vector<string> lines;
    int n = (int)words.size(), i = 0;
    while (i < n) {
        int j = i, lineLen = (int)words[i].size();
        // pack words: need lineLen + (count-1) spaces <= k
        while (j + 1 < n && lineLen + 1 + (int)words[j + 1].size() <= k) {
            ++j;
            lineLen += 1 + (int)words[j].size();
        }
        int cnt = j - i + 1;
        int wordChars = 0;
        for (int t = i; t <= j; ++t) wordChars += (int)words[t].size();
        string line;
        if (cnt == 1) {
            line = words[i];
            line += string(k - (int)line.size(), ' ');
        } else {
            int gaps = cnt - 1;
            int totalSpaces = k - wordChars;
            int base = totalSpaces / gaps, extra = totalSpaces % gaps;
            for (int t = i; t <= j; ++t) {
                line += words[t];
                if (t < j) {
                    int sp = base + (t - i < extra ? 1 : 0);
                    line += string(sp, ' ');
                }
            }
        }
        lines.push_back(line);
        i = j + 1;
    }
    return lines;
}

int main() {
    vector<string> words = {"the","quick","brown","fox","jumps","over","the","lazy","dog"};
    int k = 16;
    for (const string& l : justify(words, k))
        cout << "\"" << l << "\"\n";
    return 0;
}
