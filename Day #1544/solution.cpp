// Longest absolute path to a file in a tab-indented filesystem string.
// Track cumulative path length per depth; a name with '.' is a file.
// Time O(n), Space O(depth).
#include <bits/stdc++.h>
using namespace std;

int longestPath(const string& input) {
    vector<int> lens(1, 0); // lens[d] = path length up to depth d (before this name)
    int maxLen = 0;
    size_t start = 0;
    while (start <= input.size()) {
        size_t nl = input.find('\n', start);
        string line = input.substr(start, nl == string::npos ? string::npos : nl - start);
        int depth = 0;
        while (depth < (int)line.size() && line[depth] == '\t') depth++;
        string name = line.substr(depth);
        if (name.find('.') != string::npos) {
            maxLen = max(maxLen, lens[depth] + (int)name.size());
        } else {
            if ((int)lens.size() <= depth + 1) lens.resize(depth + 2, 0);
            lens[depth + 1] = lens[depth] + (int)name.size() + 1; // +1 for '/'
        }
        if (nl == string::npos) break;
        start = nl + 1;
    }
    return maxLen;
}

int main() {
    string input = "dir\n\tsubdir1\n\t\tfile1.ext\n\t\tsubsubdir1\n\tsubdir2\n\t\tsubsubdir2\n\t\t\tfile2.ext";
    cout << longestPath(input) << "\n";
    return 0;
}
