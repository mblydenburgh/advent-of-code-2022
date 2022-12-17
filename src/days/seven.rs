use crate::days::utils::read_lines;
use std::{rc::Rc, collections::HashMap, cell::RefCell};

#[derive(Debug)]
struct Directory {
    name: String,
    parent_dir: Option<Rc<Directory>>,
    sub_dir: RefCell<HashMap<String, Rc<Directory>>>,
    size: RefCell<usize>
}

impl Directory {
    fn get_size(&self) -> usize {
        *self.size.borrow() + self.sub_dir.borrow().values().fold(0, |a, b| a + b.get_size())
    }
    
}

pub fn day_7() -> String {
    if let Ok(lines) = read_lines("day-7-data.txt") {
        let init_dir: Rc<Directory> = Rc::new(Directory {
            name: "/".to_string(),
            size: RefCell::new(0),
            parent_dir: None,
            sub_dir: RefCell::new(HashMap::new())
        });
        let mut cwd = Rc::clone(&init_dir);
        for line in lines {
            if let Ok(value) = line {
                let words = value.split(" ").collect::<Vec<&str>>();
                match (words[0], words[1]) {
                    ("$", "cd") => match words[2] {
                            "/" => cwd = Rc::clone(&init_dir),
                            ".." => cwd = Rc::clone(cwd.as_ref().parent_dir.as_ref().unwrap()),
                            dirname  => {
                                let new_dir = cwd.sub_dir.borrow().get(dirname).unwrap().clone();
                                cwd = new_dir;
                            }
                    },
                    ("$", "ls") => {
                        //ignore
                    },
                    ("dir", dirname) => {
                        // handle dir
                        let new_dir = Directory {
                           name: dirname.to_string(),
                            size: RefCell::new(0),
                            parent_dir: Some(Rc::clone(&cwd)),
                            sub_dir: RefCell::new(HashMap::new())
                        };
                        cwd.sub_dir.borrow_mut().insert(dirname.to_string(), Rc::new(new_dir));
                    },
                    (size, _filename) => {
                        //handle file
                        *cwd.size.borrow_mut() += size.parse::<usize>().unwrap();
                    }
                }
            } 
        }
        // sum all folders whose size is <= 100,000
        let mut total = 0;
        let mut dirs = vec![Rc::clone(&init_dir)];
        
        while let Some(dir) = dirs.pop() {
            for d in dir.sub_dir.borrow().values() {
                dirs.push(Rc::clone(&d));
            }

            let size = dir.get_size();
            if size <= 100000 {
                total += size;
            }
        }
        return format!("Part 1: {}", total)
    }

    return "No answer found".to_string()
}
