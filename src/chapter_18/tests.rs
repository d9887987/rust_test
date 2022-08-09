use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;

#[cfg(test)]
mod tests{

}

#[derive(Debug,PartialEq,Eq)]
pub struct TreeNode{
    pub val:i32,
    pub left:Option<Rc<RefCell<TreeNode>>>,
    pub right:Option<Rc<RefCell<TreeNode>>>,
}

//子树节点可能存在，也可能不存在，Rc表示可以引用，RefCell表示可以使用引用进行修改
impl TreeNode {
    #[inline]
    fn new(val:i32)->Self{
        Self{
            val,
            left:None,
            right:None
        }
    }
}

#[derive(PartialEq,Eq,Clone,Debug)]
pub struct ListNode{
    pub val:i32,
    pub next:Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new (val:i32)->Self{
        Self{
            val,
            next:None
        }
    }
}


