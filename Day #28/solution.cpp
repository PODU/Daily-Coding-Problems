// Text justification: greedy line packing, distribute spaces with extras to LEFT gaps.
// Time: O(total chars), Space: O(total chars) for output.
#include <iostream>
#include <vector>
#include <string>
using namespace std;

vector<string> justify(const vector<string>& words, int k) {
    vector<string> res;
    int n = words.size();
    int i = 0;
    while (i < n) {
        int j = i, lineLen = words[i].size();
        while (j + 1 < n && lineLen + 1 + (int)words[j + 1].size() <= k) {
            j++;
            lineLen += 1 + words[j].size();
        }
        int gaps = j - i; // number of gaps between words
        string line;
        if (gaps == 0) {
            line = words[i];
            line += string(k - line.size(), ' ');
        } else {
            int totalChars = 0;
            for (int w = i; w <= j; w++) totalChars += words[w].size();
            int totalSpaces = k - totalChars;
            int base = totalSpaces / gaps;
            int extra = totalSpaces % gaps;
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
    int k = 16;
    for (const string& line : justify(words, k)) cout << line << "\n";
    return 0;
}
