use std::cmp::Ordering;

pub struct Queue {
    pub items: Vec<i8>,
}

impl Queue {
    pub fn new() -> Self {
        Queue { items: Vec::new() }
    }

    pub fn is_empty(&mut self) -> bool {
        let length = self.items.len();
        length == 0
    }

    pub fn size(&mut self) -> usize {
        self.items.len()
    }

    pub fn display(&mut self) {
        let length = self.items.len() - 1;
        for (index, element) in self.items.iter().enumerate() {
            match index.cmp(&length) {
                Ordering::Less => {
                    print!("{element}, ")
                }
                Ordering::Equal => {
                    println!("{element}");
                }
                Ordering::Greater => {}
            }
        }
    }

    pub fn enqueue(&mut self, item: i8) {
        self.items.push(item);
    }

    pub fn dequeue(&mut self) -> Option<i8> {
        if self.items.is_empty() {
            None
        } else {
            Some(self.items.remove(0))
        }
    }
}
