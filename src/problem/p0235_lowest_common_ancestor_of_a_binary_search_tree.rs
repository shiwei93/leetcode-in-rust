// 0235. Lowest Common Ancestor of a Binary Search Tree
// https://leetcode.cn/problems/lowest-common-ancestor-of-a-binary-search-tree/

#[allow(dead_code)]
pub struct Solution {}

use crate::data_structure::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

type Node = Rc<RefCell<TreeNode>>;

#[allow(dead_code)]
impl Solution {
    pub fn lowest_common_ancestor(
        _root: Option<Node>,
        _p: Option<Node>,
        _q: Option<Node>,
    ) -> Option<Node> {
        unimplemented!()
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0235() {
        assert_eq!(
            Solution::lowest_common_ancestor(
                tree![6, 2, 8, 0, 4, 7, 9, null, null, 3, 5],
                tree![2],
                tree![8]
            ),
            tree![6]
        );
        assert_eq!(
            Solution::lowest_common_ancestor(
                tree![6, 2, 8, 0, 4, 7, 9, null, null, 3, 5],
                tree![2],
                tree![4]
            ),
            tree![2]
        );
        assert_eq!(
            Solution::lowest_common_ancestor(tree![2, 1], tree![2], tree![1]),
            tree![2]
        );
    }
}
