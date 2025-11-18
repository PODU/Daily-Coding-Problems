// Full text justification: greedily pack words per line, distribute spaces evenly
// with extra spaces favoring left gaps; last/single word left-justified. Time O(total chars).
#include <bits/stdc++.h>
using namespace std;

vector<string> justify(const vector<string>& words, int k) {
    vector<string> res;
    int n = words.size(), i = 0;
    while (i < n) {
        int j = i, lineLen = words[i].size();
        while (j + 1 < n && lineLen + 1 + (int)words[j + 1].size() <= k) {
            j++;
            lineLen += 1 + words[j].size();
        }
        int gaps = j - i; // number of gaps between words on this line
        string line;
        if (gaps == 0) {
            line = words[i];
            line += string(k - line.size(), ' ');
        } else {
            int totalSpaces = k;
            for (int w = i; w <= j; w++) totalSpaces -= words[w].size();
            int base = totalSpaces / gaps, extra = totalSpaces % gaps;
            for (int w = i; w <= j; w++) {
                line += words[w];
                if (w < j) {
                    int sp = base + (w - i < extra ? 1 : 0);
                    line += string(sp, ' ');
                }
            }
        }
        res.push_back(line);
        i = j + 1;
    }
    return res;
}

int main() {
    vector<string> words = {"the","quick","brown","fox","jumps","over","the","lazy","dog"};
    for (const string& line : justify(words, 16))
        cout << '"' << line << '"' << endl;
    return 0;
}
