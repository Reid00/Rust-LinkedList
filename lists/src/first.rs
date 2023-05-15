
// stack 的模式后进先出创建链表
// head 指向链表的最后一个元素

pub struct List {
    head: Link
}

#[derive(Clone)]
pub enum Link {
    Empty,
    Data(Box<Node>)
}

#[derive(Clone)]
pub struct Node {
    elem: i32,
    next: Link
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let node = Node{
            elem: elem,
            next: std::mem::replace(&mut self.head, Link::Empty),
            // next: self.head,     move owner
        };

        self.head = Link::Data(Box::new(node));
    }

    pub fn pop(&mut self) -> Option<i32>{
        match std::mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::Data(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }

        // let result;
        // match &self.head {
        //     Link::Empty => {
        //         result = None;
        //     },
        //     Link::Data(node) => {
        //         result = Some(node.elem);
        //         self.head = node.next;
        //     }
        // }
        // result
    }
}



impl Drop for List {

    fn drop(&mut self) {
        let mut cur_link = std::mem::replace(&mut self.head, Link::Empty);

        while let Link::Data(mut boxed_node) = cur_link {
            cur_link = std::mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
    
}






#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics(){
        let mut list = List::new();

        // check list is empty
        assert_eq!(list.pop(), None);

        // push
        list.push(1);
        list.push(2);
        list.push(3);

        // check normal remove
        assert_eq!(Some(3), list.pop());
        assert_eq!(Some(2), list.pop());

        // push more data
        list.push(4);
        list.push(5);

        // normal remove
        assert_eq!(Some(5), list.pop());
        assert_eq!(Some(4), list.pop());

        // check exhaustion
        assert_eq!(Some(1), list.pop());
        assert_eq!(None, list.pop());



    }
}