# Day 961: Normalize an absolute Unix path resolving "." and "..".
# Approach: split by '/', use a stack. Time O(n), Space O(n).


def simplify_path(path: str) -> str:
    stack = []
    for seg in path.split('/'):
        if seg == '' or seg == '.':
            continue
        if seg == '..':
            if stack:
                stack.pop()
        else:
            stack.append(seg)
    res = '/' + '/'.join(stack)
    if path and path[-1] == '/' and res != '/':
        res += '/'
    return res


if __name__ == '__main__':
    print('"' + simplify_path("/usr/bin/../bin/./scripts/../") + '"')
