// Full text justification: greedy packing + even space distribution (extras
// to the left). Time O(total chars), Space O(output).
#include <bits/stdc++.h>
using namespace std;

vector<string> justify(const vector<string>& words, int k) {
    vector<string> lines;
    int i = 0, n = words.size();
    while (i < n) {
        int j = i, length = 0;
        while (j < n && length + (int)words[j].size() + (j - i) <= k) {
            length += words[j].size();
            ++j;
        }
        int gaps = j - i - 1;
        string line;
        if (gaps == 0) {
            line = words[i] + string(k - words[i].size(), ' ');
        } else {
            int spaces = k - length, base = spaces / gaps, extra = spaces % gaps;
            for (int w = i; w < j - 1; ++w) {
                line += words[w];
                line += string(base + ((w - i) < extra ? 1 : 0), ' ');
            }
            line += words[j - 1];
        }
        lines.push_back(line);
        i = j;
    }
    return lines;
}

int main() {
    vector<string> words = {"the", "quick", "brown", "fox", "jumps",
                            "over", "the", "lazy", "dog"};
    for (auto& line : justify(words, 16)) cout << "\"" << line << "\"\n";
    return 0;
}
