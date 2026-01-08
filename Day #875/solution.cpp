// Longest absolute file path: track path length per depth in one pass.
// Time O(n), Space O(d) for depth d.
#include <bits/stdc++.h>
using namespace std;

int lengthLongestPath(const string& input){
    vector<int> pathLen(1, 0); // pathLen[depth] = prefix length at that depth
    int maxLen = 0;
    size_t i = 0, n = input.size();
    while(i < n){
        size_t j = input.find('\n', i);
        if(j == string::npos) j = n;
        string line = input.substr(i, j - i);
        i = j + 1;
        int depth = 0;
        while(depth < (int)line.size() && line[depth] == '\t') depth++;
        string name = line.substr(depth);
        if((int)pathLen.size() <= depth + 1) pathLen.resize(depth + 2, 0);
        if(name.find('.') != string::npos){
            maxLen = max(maxLen, pathLen[depth] + (int)name.size());
        } else {
            pathLen[depth + 1] = pathLen[depth] + (int)name.size() + 1; // +1 for '/'
        }
    }
    return maxLen;
}

int main(){
    string input = "dir\n\tsubdir1\n\t\tfile1.ext\n\t\tsubsubdir1\n\tsubdir2\n\t\tsubsubdir2\n\t\t\tfile2.ext";
    cout << lengthLongestPath(input) << "\n";
    return 0;
}
