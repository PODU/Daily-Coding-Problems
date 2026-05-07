// Day 1481: Reverse words while keeping delimiters in their original positions.
// Tokenize into word-runs and delimiter chars, reverse word tokens, re-emit.
// Handles trailing/consecutive delimiters. Time O(N), Space O(N).
#include <bits/stdc++.h>
using namespace std;

string reverseWords(const string& s, const set<char>& delims) {
    vector<pair<bool, string>> tokens;
    int i = 0, n = s.size();
    while (i < n) {
        if (delims.count(s[i])) {
            tokens.push_back({false, string(1, s[i])});
            ++i;
        } else {
            int j = i;
            while (j < n && !delims.count(s[j])) ++j;
            tokens.push_back({true, s.substr(i, j - i)});
            i = j;
        }
    }
    vector<string> words;
    for (auto& t : tokens) if (t.first) words.push_back(t.second);
    reverse(words.begin(), words.end());
    string out;
    int k = 0;
    for (auto& t : tokens) out += t.first ? words[k++] : t.second;
    return out;
}

int main() {
    set<char> d = {'/', ':'};
    cout << reverseWords("hello/world:here", d) << "\n";   // here/world:hello
    cout << reverseWords("hello/world:here/", d) << "\n";  // here/world:hello/
    cout << reverseWords("hello//world:here", d) << "\n";  // here//world:hello
}
