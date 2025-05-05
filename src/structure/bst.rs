use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub type BstNodeLink = Rc<RefCell<BstNode>>;
pub type WeakBstNodeLink = Weak<RefCell<BstNode>>;

//this package implement BST wrapper
#[derive(Debug, Clone)]
pub struct BstNode {
    pub key: Option<i32>,
    pub parent: Option<WeakBstNodeLink>,
    pub left: Option<BstNodeLink>,
    pub right: Option<BstNodeLink>,
}

impl BstNode {
    //private interface
    fn new(key: i32) -> Self {
        BstNode {
            key: Some(key),
            left: None,
            right: None,
            parent: None,
        }
    }

    pub fn new_bst_nodelink(value: i32) -> BstNodeLink {
        let currentnode = BstNode::new(value);
        let currentlink = Rc::new(RefCell::new(currentnode));
        currentlink
    }

    /**
     * Get a copy of node link
     */
    pub fn get_bst_nodelink_copy(&self) -> BstNodeLink {
        Rc::new(RefCell::new(self.clone()))
    }

    fn downgrade(node: &BstNodeLink) -> WeakBstNodeLink {
        Rc::<RefCell<BstNode>>::downgrade(node)
    }

    //private interface
    fn new_with_parent(parent: &BstNodeLink, value: i32) -> BstNodeLink {
        let mut currentnode = BstNode::new(value);
        //currentnode.add_parent(Rc::<RefCell<BstNode>>::downgrade(parent));
        currentnode.parent = Some(BstNode::downgrade(parent));
        let currentlink = Rc::new(RefCell::new(currentnode));
        currentlink
    }

    //add new left child, set the parent to current_node_link
    pub fn add_left_child(&mut self, current_node_link: &BstNodeLink, value: i32) {
        let new_node = BstNode::new_with_parent(current_node_link, value);
        self.left = Some(new_node);
    }

    //add new left child, set the parent to current_node_link
    pub fn add_right_child(&mut self, current_node_link: &BstNodeLink, value: i32) {
        let new_node = BstNode::new_with_parent(current_node_link, value);
        self.right = Some(new_node);
    }

    //search the current tree which node fit the value
    pub fn tree_search(&self, value: &i32) -> Option<BstNodeLink> {
        if let Some(key) = self.key {
            if key == *value {
                return Some(self.get_bst_nodelink_copy());
            }
            if *value < key && self.left.is_some() {
                return self.left.as_ref().unwrap().borrow().tree_search(value);
            } else if self.right.is_some() {
                return self.right.as_ref().unwrap().borrow().tree_search(value);
            }
        }
        //default if current node is NIL
        None
    }

    /**seek minimum by recurs
     * in BST minimum always on the left
     */
    pub fn minimum(&self) -> BstNodeLink {
        if self.key.is_some() {
            if let Some(left_node) = &self.left {
                return left_node.borrow().minimum();
            }
        }
        self.get_bst_nodelink_copy()
    }

    pub fn maximum(&self) -> BstNodeLink {
        if self.key.is_some() {
            if let Some(right_node) = &self.right {
                return right_node.borrow().maximum();
            }
        }
        self.get_bst_nodelink_copy()
    }

    /**
     * Return the root of a node, return self if not exist
     */
    pub fn get_root(node: &BstNodeLink) -> BstNodeLink {
        let parent = BstNode::upgrade_weak_to_strong(node.borrow().parent.clone());
        if parent.is_none() {
            return node.clone();
        }
        return BstNode::get_root(&parent.unwrap());
    }

    /**
     * NOTE: Buggy from pull request
     * Find node successor according to the book
     * Should return None, if x_node is the highest key in the tree
     */
    pub fn tree_successor(x_node: &BstNodeLink) -> Option<BstNodeLink> {
        // directly check if the node has a right child, otherwise go to the next block
        if let Some(right_node) = &x_node.borrow().right {
            return Some(right_node.borrow().minimum());
        } 
        
        // empty right child case
        else { 
            let mut x_node = x_node;
            let mut y_node = BstNode::upgrade_weak_to_strong(x_node.borrow().parent.clone());
            let mut temp: BstNodeLink;

            while let Some(ref exist) = y_node {
                if let Some(ref left_child) = exist.borrow().left {
                    if BstNode::is_node_match(left_child, x_node) {
                        return Some(exist.clone());
                    }
                }

                temp = y_node.unwrap();
                x_node = &temp;
                y_node = BstNode::upgrade_weak_to_strong(temp.borrow().parent.clone());
            }

            None    
        }
    }

    /**
     * Alternate simpler version of tree_successor that made use of is_nil checking
     */
    #[allow(dead_code)]
    pub fn tree_successor_simpler(x_node: &BstNodeLink) -> Option<BstNodeLink>{
        //create a shadow of x_node so it can mutate
        let mut x_node = x_node;
        let right_node = &x_node.borrow().right.clone();
        if BstNode::is_nil(right_node)!=true{
            return Some(right_node.clone().unwrap().borrow().minimum());
        }

        let mut y_node = BstNode::upgrade_weak_to_strong(x_node.borrow().parent.clone());
        let y_node_right = &y_node.clone().unwrap().borrow().right.clone();
        let mut y_node2: Rc<RefCell<BstNode>>;
        while BstNode::is_nil(&y_node) && BstNode::is_node_match_option(Some(x_node.clone()), y_node_right.clone()) {
            y_node2 = y_node.clone().unwrap();
            x_node = &y_node2;
            let y_parent = y_node.clone().unwrap().borrow().parent.clone().unwrap();
            y_node = BstNode::upgrade_weak_to_strong(Some(y_parent));
        }

        //in case our sucessor traversal yield root, means self is the highest key
        if BstNode::is_node_match_option(y_node.clone(), Some(BstNode::get_root(&x_node))) {
            return None;
        }

        //default return self / x_node
        return Some(y_node.clone().unwrap())
    }

    // always from root asumption
    pub fn tree_insert(&mut self, current_node_link: &BstNodeLink, value: i32) {
        if self.clone().key_is_bigger(value) { // check the value with the current key lower or higher
            if self.left.is_some() {
                return self.left.clone().unwrap().borrow_mut().tree_insert(&self.left.clone().unwrap(), value) // go to the lower node of the left subtree
            }
            return self.add_left_child(current_node_link, value); // if there is no value in the left, assign the value
        }else {
            if self.right.is_some() {
                return self.right.clone().unwrap().borrow_mut().tree_insert(&self.right.clone().unwrap(), value) // go to the lower node of the right subtree
            }
            return self.add_right_child(current_node_link, value); // if there is no value in the left, assign the value
        }
    }

    fn transplant(rootlink: BstNodeLink, u: &Rc<RefCell<BstNode>>, v: &Option<Rc<RefCell<BstNode>>>) {
        let parent_u = &BstNode::upgrade_weak_to_strong(u.borrow().parent.clone()); // None
        if parent_u.is_none() { // ngecek parent ada atau nggak
            if BstNode::is_nil(v) { // ngecek v ada atau nggak
                if let Some(left_u) = u.borrow().left.clone() {
                    left_u.borrow_mut().parent = None;
                }
                if let Some(right_u) = u.borrow().right.clone() {
                    right_u.borrow_mut().parent = None;
                }
            }
            if let Some(left_u) = u.borrow().left.clone() {
                left_u.borrow_mut().parent = Some(BstNode::downgrade(&v.clone().unwrap()));
            }
            if let Some(right_u) = u.borrow().right.clone() {
                right_u.borrow_mut().parent = Some(BstNode::downgrade(&v.clone().unwrap()));
            }
        }else if BstNode::is_node_match_option(Some(u.clone()), parent_u.clone().unwrap().borrow().left.clone()) { // u dari kiri parent
            if parent_u.clone().unwrap().try_borrow_mut().is_err() { // cek eror
                rootlink.borrow_mut().left = v.clone(); // asign parent.left to transplant node
            }else {
                parent_u.clone().unwrap().borrow_mut().left = v.clone(); // asign parent.left to transplant node
            }
        }else {
            if parent_u.clone().unwrap().try_borrow_mut().is_err() {
                rootlink.borrow_mut().right = v.clone();
            }else {
                parent_u.clone().unwrap().borrow_mut().right = v.clone();
            }
            parent_u.clone().unwrap().try_borrow_mut().unwrap().right = v.clone();
        }
        if let Some(exist) = &v {
            if let Some(new_parent) = parent_u.clone() {
                exist.borrow_mut().parent = Some(BstNode::downgrade(&new_parent)); // v parent to u.parent if exist
            }else {
                exist.borrow_mut().parent = None; // parent to none if not exist
            }
        }
    }

    pub fn tree_delete(rootlink: BstNodeLink, target: &BstNodeLink) -> BstNodeLink{
        println!("target :{:?}", target);
        if target.borrow().left.is_none() {
            BstNode::transplant(rootlink.clone(),target, &target.borrow().right);
        }else if target.borrow().right.is_none() {
            BstNode::transplant(rootlink.clone(),target, &target.borrow().left);
        }else {
            let right_minimum = &target.clone().borrow().right.clone().unwrap().borrow().minimum(); // 17
            // println!("node y : {:?}", right_minimum);
            let right_minimum_parent = &BstNode::upgrade_weak_to_strong(right_minimum.borrow().parent.clone()); // 18
            // println!("y parent : {:?}", right_minimum_parent);
            if !(BstNode::is_node_match_option(right_minimum_parent.clone(), Some(target.clone()))) { // harusnya gamasuk kesini
                BstNode::transplant(rootlink.clone(),right_minimum, &right_minimum.borrow().right.clone()); // 17 -> 17.kanan
                right_minimum.borrow_mut().right = target.borrow().right.clone(); //17.kanan -> 18
                println!("minimum right 1 : {:?}", right_minimum);
                right_minimum.borrow_mut().right.clone().unwrap().borrow_mut().parent = Some(BstNode::downgrade(&right_minimum.clone())); // 18.parent -> 17
                println!("minimum right 2 : {:?}", right_minimum);
            }
            // println!("state :{}", !(BstNode::is_node_match_option(right_minimum_parent.clone(), Some(target.clone()))));
            println!("minimum right 3 :{:?}", right_minimum);
            BstNode::transplant(rootlink.clone(),target, &Some(right_minimum.clone())); // ngubah 17 jadi 18
            right_minimum.borrow_mut().left = target.borrow().left.clone(); // masukin 17.kiri jadi 18.kiri
            right_minimum.borrow().left.clone().unwrap().borrow_mut().parent = Some(BstNode::downgrade(&right_minimum.clone())); // set 18.kiri.parent jadi 18 
            println!("minimum right 4 :{:?}", right_minimum);
            if let Some(_exist) = target.borrow().parent.clone() {
                return rootlink;
            }
            return right_minimum.clone();
        }
        rootlink
    }


    /**
     * private function return true if node doesn't has parent nor children nor key
     */
    fn is_nil(node: &Option<BstNodeLink>) -> bool {
        match node {
            None => true,
            Some(x) => {
                if x.borrow().parent.is_none()
                    || x.borrow().left.is_none()
                    || x.borrow().right.is_none()
                {
                    return true;
                }
                return false;
            }
        }
    }

    fn option_val(node: &Option<BstNodeLink>) -> Option<BstNodeLink> {
        if let Some(nodes) = node{
            return node.clone();
        }
        return None;
    }

    // helper function to compare both nodelink
    fn is_node_match_option(node1: Option<BstNodeLink>, node2: Option<BstNodeLink>) -> bool {
        if node1.is_none() && node2.is_none() {
            return true;
        }
        if let Some(node1v) = node1 {
            return node2.is_some_and(|x: BstNodeLink| x.borrow().key == node1v.borrow().key);
        }
        return false;
    }

    fn is_node_match(anode: &BstNodeLink, bnode: &BstNodeLink) -> bool {
        if anode.borrow().key == bnode.borrow().key {
            return true;
        }
        return false;
    }

    /**
     * As the name implied, used to upgrade parent node to strong nodelink
     */
    fn upgrade_weak_to_strong(node: Option<WeakBstNodeLink>) -> Option<BstNodeLink> {
        match node {
            None => None,
            Some(x) => Some(x.upgrade().unwrap()),
        }
    }

    fn key_is_bigger(self, value: i32) -> bool {
        if self.key.unwrap() > value {
            return true;
        }
        return false;
    }
}