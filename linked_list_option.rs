// use List::*;

#[derive(Debug)]
struct List<T> {
    list: Node<T>,
}

type Node<T> = Option<Box<Link<T>>>;
 
#[derive(Debug)]
struct Link<T> {
  head: T,
  tail: Node<T>,
}

impl<T> List<T> {
  pub fn new() -> Self {
    return List{ list: None };
  }

  pub fn push(&mut self, elem: T) {
    self.list = Some(Box::new(Link{ head: elem, tail: std::mem::replace(&mut self.list, None) }));
  }

  pub fn pop(&mut self) -> Option<T> {
    match std::mem::replace(&mut self.list, None) {
      Some(next_box) => {
        let next = *next_box;
        self.list = next.tail;
        Some(next.head)
      }
      _ => None
    }
  }

  // If the list has at least 2 elements, swaps the first two.
  pub fn bubble(&mut self) -> bool {
    if let Some(first) = self.pop() {
      if let Some(second) = self.pop() {
        self.push(first);
        self.push(second);
        return true;
      } else {
        self.push(first);
      }
    }
    false
  }


  // Any tail of 'singleton' is silently discarded.
  pub fn push_singleton(&mut self, mut singleton: Box<Link<T>>) {
    std::mem::swap(&mut self.list, &mut singleton.tail);
    self.list = Some(singleton);
  }

  pub fn pop_singleton(&mut self) -> Node<T> {
    match std::mem::replace(&mut self.list, None) {
      Some(mut next_box) => {
        std::mem::swap(&mut self.list, &mut next_box.tail);
        Some(next_box)
      }
      _ => None
    }
  }

  pub fn bubble2(&mut self) -> bool {
    if let Some(first_box) = self.pop_singleton() {
      if let Some(second_box) = self.pop_singleton() {
        self.push_singleton(first_box);
        self.push_singleton(second_box);
        return true;
      } else {
        self.push_singleton(first_box);
      }
    }
    false
  }


  pub fn bubble3(&mut self) -> bool {
    if let Some(mut first_box) = std::mem::replace(&mut self.list, None) {
      if let Some(mut second_box) = std::mem::replace(&mut first_box.tail, None) {
        first_box.tail = std::mem::replace(&mut second_box.tail, None);
        second_box.tail = Some(first_box);
        *self = List{ list: Some(second_box) };
        return true;
      } else {
        *self = List{ list: Some(first_box) };
      }
    }
    false
  }
}

impl<T: ToString> ToString for List<T> {
  fn to_string(self: &List<T>) -> String {
    let mut out = String::from("[");
    match &self.list {
      &None => {},
      &Some(ref b) => {
        let mut b = b;
        out.push_str(b.head.to_string().as_str());
        while let &Some(ref x) = &b.tail {
          out.push_str(", ");
          out.push_str(x.head.to_string().as_str());
          b = x;
        }
      }
    }
    out.push_str("]");
    out
  }
}

impl<T> Iterator for List<T> {
    type Item = T;
    
    fn next(&mut self) -> Option<T> {
      match std::mem::replace(&mut self.list, None) {
        None => None,
        Some(b) => {
          let content = *b;
          *self = List{ list: content.tail };
          Some(content.head)
        },
      }
    }
}


fn sample() -> List<i64> {
  let mut l = List::new();
  l.push(3);
  l.push(2);
  l.push(1);
  l
}

fn main() {
   println!("{}", List::<i64>::new().to_string());
   println!("{}", sample().to_string());
   for i in sample().map(|x| 1<<x) {
     println!("{}", i);
   }
   println!("{}", sample().fold(0, |acc, len| acc + len));
   {
     let mut list = sample();
     list.bubble();
     println!("{}", list.to_string());
     list.bubble2();
     println!("{}", list.to_string());
     list.bubble3();
     println!("{}", list.to_string());
   }
   {
     let mut list = List::new();
     list.push(1);
     list.bubble();
     println!("{}", list.to_string());
     list.bubble2();
     println!("{}", list.to_string());
     list.bubble3();
     println!("{}", list.to_string());
   }
}
