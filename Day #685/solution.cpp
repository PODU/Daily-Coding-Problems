// Reverse words between delimiters while keeping delimiters fixed in position.
// Tokenize (words = maximal non-delim runs incl. interior empties), reverse word list, reassemble. O(n) time/space.
#include <iostream>
#include <vector>
#include <string>
#include <set>
#include <algorithm>
#include <utility>
using namespace std;

string reverseWords(const string& s, const set<char>& delims) {
    vector<pair<bool, string>> tokens; // (isDelim, value)
    string buf;
    for (char c : s) {
        if (delims.count(c)) {
            tokens.push_back({false, buf});
            tokens.push_back({true, string(1, c)});
            buf.clear();
        } else {
            buf += c;
        }
    }
    if (!buf.empty()) tokens.push_back({false, buf});

    vector<string> words;
    for (auto& t : tokens) if (!t.first) words.push_back(t.second);
    reverse(words.begin(), words.end());

    string out;
    int wi = 0;
    for (auto& t : tokens) {
        if (t.first) out += t.second;
        else out += words[wi++];
    }
    return out;
}

int main() {
    set<char> d = {'/', ':'};
    vector<string> tests = {"hello/world:here", "hello/world:here/", "hello//world:here"};
    for (auto& t : tests)
        cout << t << " -> " << reverseWords(t, d) << "\n";
    return 0;
}
