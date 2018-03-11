// use List::*;

#[derive(Debug)]
struct List<T> {
    list: Option<Box<Link<T>>>,
}
 
#[derive(Debug)]
struct Link<T> {
  head: T,
  tail: List<T>,
}

impl<T> List<T> {
  pub fn new() -> Self {
    return List{ list: None };
  }

  pub fn push(&mut self, elem: T) {
    let l = List{ list: Some(Box::new(Link{ head: elem, tail: std::mem::replace(self, List::new()) })) };
    *self = l;
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
        while let &Some(ref x) = &b.tail.list {
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
      let mut other = List{ list: None };
      std::mem::swap(self, &mut other);
      match other.list {
        None => None,
        Some(b) => {
          let content = *b;
          *self = content.tail;
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
}
