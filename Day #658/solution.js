// Longest absolute file path: split on '\n', track pathLen by depth via array. Time O(n), Space O(depth).
function lengthLongestPath(input) {
    const lines = input.split("\n");
    const pathLen = {};
    let maxLen = 0;
    for (const line of lines) {
        let depth = 0;
        while (depth < line.length && line[depth] === "\t") depth++;
        const name = line.slice(depth);
        const curLen = (depth > 0 ? pathLen[depth - 1] + 1 : 0) + name.length;
        pathLen[depth] = curLen;
        if (name.indexOf(".") !== -1)
            maxLen = Math.max(maxLen, curLen);
    }
    return maxLen;
}

const input = "dir\n\tsubdir1\n\t\tfile1.ext\n\t\tsubsubdir1\n\tsubdir2\n\t\tsubsubdir2\n\t\t\tfile2.ext";
console.log(lengthLongestPath(input));
