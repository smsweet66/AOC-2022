use std::cell::RefCell;
use std::rc::Rc;
use crate::tasks::helper::get_lines;

#[derive(Default)]
pub struct Folder
{
    name: String,
    children_folders: Vec<Rc<RefCell<Folder>>>,
    children_files: Vec<Rc<RefCell<File>>>,
    parent: Option<Rc<RefCell<Folder>>>,
}

impl Folder
{
    pub fn get_size(&self) -> u64
    {
        let mut sum = 0;
        for child in &self.children_files
        { sum += child.borrow().size; }

        for child in &self.children_folders
        { sum += child.borrow().get_size(); }

        return sum;
    }

    pub fn get_sum_small_directories(&self) -> u64
    {
        let mut sum = 0;
        if self.get_size() < 100000
        { sum += self.get_size(); }

        for child in &self.children_folders
        { sum += child.borrow().get_sum_small_directories(); }

        return sum;
    }

    pub fn in_order_folder_traversal(&self) -> Vec<(String, u64)>
    {
        let mut result = Vec::new();
        for child in &self.children_folders
        {
            let mut child_result = child.borrow().in_order_folder_traversal();
            result.append(&mut child_result);
        }

        result.push((self.name.clone(), self.get_size()));

        return result;
    }

    pub fn from_file(filename: &str) -> Rc<RefCell<Folder>>
    {
        let lines = get_lines(filename);
        let root = Rc::new(RefCell::new(Folder::default()));
        root.borrow_mut().name = "/".to_string();
        root.borrow_mut().parent = None;
        let mut current = Rc::clone(&root);
        for line in lines
        {
            let mut parts = line.split_whitespace();
            match parts.next()
            {
                Some("$") => {
                    match parts.next()
                    {
                        Some("cd") => {
                            let folder_name = parts.next().unwrap();
                            if folder_name == "/"
                            { current = Rc::clone(&root); }
                            else if folder_name == ".."
                            {
                                let parent = current.borrow().parent.as_ref().unwrap().clone();
                                current = Rc::clone(&parent);
                            }
                            else
                            {
                                let mut next_folder = None;
                                for child in &current.borrow().children_folders
                                {
                                    if child.borrow().name == folder_name
                                    {
                                        next_folder = Some(Rc::clone(child));
                                        break;
                                    }
                                }
                                if next_folder.is_some()
                                { current = next_folder.unwrap(); }
                            }
                        }
                        _ => {}
                    }
                }
                Some("dir") => {
                    let mut folder = Folder::default();
                    folder.name = parts.next().unwrap().to_string();
                    folder.parent = Option::from(Rc::clone(&current));
                    current.borrow_mut().children_folders.push(Rc::new(RefCell::new(folder)));
                }
                Some(value) => {
                    let mut file = File::default();
                    file.size = value.parse::<u64>().unwrap();
                    file.name = parts.next().unwrap().to_string();
                    file.parent = Option::from(Rc::clone(&current));
                    current.borrow_mut().children_files.push(Rc::new(RefCell::new(file)));
                }
                None => {}
            }
        }

        return root;
    }
}

#[derive(Default)]
struct File
{
    name: String,
    size: u64,
    parent: Option<Rc<RefCell<Folder>>>,
}