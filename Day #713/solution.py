# Day 713: Normalize absolute Unix path. Split on '/', use a stack: skip ""/".",
# pop on "..". Preserve a trailing slash if the input had one. Time O(n).

def normalize(path):
    stk = []
    for comp in path.split('/'):
        if comp == '' or comp == '.':
            continue
        if comp == '..':
            if stk:
                stk.pop()
        else:
            stk.append(comp)
    res = '/' + '/'.join(stk)
    trailing = len(path) > 1 and path[-1] == '/'
    if trailing and res != '/' and not res.endswith('/'):
        res += '/'
    return res


if __name__ == "__main__":
    print('"' + normalize("/usr/bin/../bin/./scripts/../") + '"')
