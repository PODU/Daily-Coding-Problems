// Longest absolute file path. Parse tab-indented FS, track cumulative path
// length per depth in a map. Time O(n), Space O(depth).
#include <bits/stdc++.h>
using namespace std;

int lengthLongestPath(const string& input) {
    vector<string> lines;
    string cur;
    for (char c : input) {
        if (c == '\n') { lines.push_back(cur); cur.clear(); }
        else cur.push_back(c);
    }
    lines.push_back(cur);

    unordered_map<int,int> pathLen; // pathLen[level] = prefix length (with trailing slash)
    pathLen[0] = 0;
    int best = 0;
    for (auto& l : lines) {
        int level = 0;
        while (level < (int)l.size() && l[level] == '\t') level++;
        string name = l.substr(level);
        if (name.find('.') != string::npos) {              // a file
            best = max(best, pathLen[level] + (int)name.size());
        } else {                                            // a directory
            pathLen[level + 1] = pathLen[level] + (int)name.size() + 1;
        }
    }
    return best;
}

int main() {
    string input = "dir\n\tsubdir1\n\t\tfile1.ext\n\t\tsubsubdir1\n\tsubdir2\n\t\tsubsubdir2\n\t\t\tfile2.ext";
    cout << lengthLongestPath(input) << "\n"; // 32
    return 0;
}
