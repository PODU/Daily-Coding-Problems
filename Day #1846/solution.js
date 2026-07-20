// Day 1846: Serialize/deserialize a binary tree via preorder traversal with null markers.
// Time O(N), Space O(N). Uses JSON nested arrays for safe round-tripping.

class Node {
  constructor(val, left = null, right = null) {
    this.val = val;
    this.left = left;
    this.right = right;
  }
}

function serialize(root) {
  const build = (n) => (n === null ? null : [n.val, build(n.left), build(n.right)]);
  return JSON.stringify(build(root));
}

function deserialize(s) {
  const build = (d) => (d === null ? null : new Node(d[0], build(d[1]), build(d[2])));
  return build(JSON.parse(s));
}

function main() {
  const node = new Node("root", new Node("left", new Node("left.left")), new Node("right"));
  console.log(deserialize(serialize(node)).left.left.val === "left.left");
}

main();
