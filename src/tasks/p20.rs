use std::cell::RefCell;
use std::rc::Rc;
use crate::tasks::helper::get_lines;

//node for a circular doubly linked list
struct Node
{
	node_value: i32,
	prev: Option<Rc<RefCell<Node>>>,
	next: Option<Rc<RefCell<Node>>>,
}

impl Node
{
	//creates a new node
	pub fn new(node_value: i32) -> Node
	{ Node { node_value, prev: None, next: None } }
}

//circular doubly linked list that keeps references to each node in the order added
struct CDLList
{
	nodes: Vec<Rc<RefCell<Node>>>,
}

impl CDLList
{
	//creates a new circular doubly linked list
	fn new() -> CDLList
	{
		CDLList { nodes: Vec::new() }
	}

	//adds a new node to the list
	fn insert(&mut self, value: i32)
	{
		let node = Rc::new(RefCell::new(Node::new(value)));
		if self.nodes.len() == 0
		{
			node.borrow_mut().prev = Some(node.clone());
			node.borrow_mut().next = Some(node.clone());
		}
		else
		{
			let last = self.nodes.last().unwrap().clone();
			let first = self.nodes.first().unwrap().clone();
			node.borrow_mut().prev = Some(last.clone());
			node.borrow_mut().next = Some(first.clone());
			last.borrow_mut().next = Some(node.clone());
			first.borrow_mut().prev = Some(node.clone());
		}

		self.nodes.push(node);
	}

	///moves each node a number of spaces equal to its value
	pub fn mix(&mut self)
	{
		for node in &self.nodes
		{
			//println!("moving node {}", node.borrow().node_value);
			let mut node = node.clone();
			let mut value = node.borrow().node_value;
			//as the list is circular, we can reduce the number of movements by
			//taking the remainder of the value and the length of the list
			if value >= 0
			{ value %= self.nodes.len() as i32; }
			else
			{ value = self.nodes.len() as i32 -((-value) % self.nodes.len() as i32) - 1; }

			//swap node with the node to the right of it
			for _ in 0..value
			{
				let mut next = node.borrow_mut().next.clone().unwrap();
				let mut prev = node.borrow_mut().prev.clone().unwrap();
				let mut next_next = next.borrow_mut().next.clone().unwrap();

				//println!("original: {}, {}, {}, {}", prev.borrow().node_value, node.borrow().node_value, next.borrow().node_value, next_next.borrow().node_value);

				node.borrow_mut().next = Some(next_next.clone());
				node.borrow_mut().prev = Some(next.clone());
				next.borrow_mut().next = Some(node.clone());
				next.borrow_mut().prev = Some(prev.clone());
				next_next.borrow_mut().prev = Some(node.clone());
				prev.borrow_mut().next = Some(next.clone());

				//println!("after move: {}, {}, {}, {}", prev.borrow().node_value, next.borrow().node_value, node.borrow().node_value, next_next.borrow().node_value);
			}

			//println!("{}", self.to_string());
		}
	}

	fn get_value_from_zero(&self, index: usize) -> i32
	{
		let mut node = self.nodes[0].clone();
		while node.borrow().node_value != 0
		{
			let next = node.borrow_mut().next.clone().unwrap();
			node = next;
		}

		for _ in 0..index%self.nodes.len()
		{
			let next = node.borrow_mut().next.clone().unwrap();
			node = next;
		}

		let x = node.borrow().node_value;
		x
	}

	fn to_string(&self) -> String
	{
		let mut s = String::new();
		let mut node = self.nodes[0].clone();
		for _ in 0..self.nodes.len()
		{
			s.push_str(&node.borrow().node_value.to_string());
			s.push_str(" ");
			let next = node.borrow_mut().next.clone().unwrap();

			node = next;
		}
		s
	}
}

///The input is a file containing a list of numbers, one per line.
///These numbers are part of a circular list.  After building the list,
///Each number is moved a number of spaces equal to its value.  This is
///done in the order that the numbers initially appear in the file.
///This function then returns the sum of the numbers 1000, 2000, and 3000 after
///the 0 in the list.
pub fn get_sum_indices(filename: &str) -> i32
{
	let lines = get_lines(filename);
	let mut list = CDLList::new();
	for line in lines
	{ list.insert(line.parse().unwrap()); }

	println!("{}", list.to_string());

	list.mix();

	println!("{}", list.to_string());

	let mut sum = 0;
	for i in 1..=3
	{ sum += list.get_value_from_zero(1000*i); }

	sum
}