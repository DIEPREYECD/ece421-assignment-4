use self::LinkedList::*;
use im::list::*;

#[derive(Debug, PartialEq)]
pub enum LinkedList<T>{
	Tail,
	Head(List<T>),
}

impl<T> LinkedList<T>{
	pub fn new(x: T) -> Self{
		Head(cons(x, List::new()))
	}

	pub fn empty() -> Self{
		Tail
	}

	pub fn push(self, x: T) -> Self{
		match self{
			Tail => LinkedList::new(x),
			Head(l) => Head(cons(x, l)),
		}
	}
	pub fn push_back(&mut self, x: T){
		match self{
			Tail => *self = LinkedList::new(x),
			Head(l) => {
				l.push_back(x);
			}
		}
	}
}
#[cfg(test)]
mod tests{
	use super::*;

	#[test]
	fn it_works(){
		let mut l = LinkedList::new(3);
		l = l.push(4);
		assert_eq!(l,Head(cons(4, cons(3, List::new()))));
		l.push_back(2);
		assert_eq!(l,Head(cons(4, cons(3, cons(2, List::new())))));
	}
}

fn main(){
}
