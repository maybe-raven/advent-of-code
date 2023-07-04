#![allow(dead_code)]
use std::{
    cell::RefCell,
    fmt::{Display, Formatter},
    io::{self, Read},
    ops::Deref,
    rc::{Rc, Weak},
};

enum Command<'a> {
    CD(&'a str),
    LS(LsResult<'a>),
}

impl<'a> TryFrom<&'a str> for Command<'a> {
    type Error = String;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let ferrstr = || format!("{} is not a valid command.", s);
        let (command_str, command_result) = s.split_once('\n').ok_or_else(ferrstr)?;
        let mut s = command_str.split_whitespace();
        match s.next().ok_or_else(ferrstr)? {
            "cd" => Ok(Self::CD(s.next().ok_or_else(ferrstr)?)),
            "ls" => Ok(Self::LS(LsResult::try_from(command_result)?)),
            _ => Err(ferrstr()),
        }
    }
}

type DirectoryRef<'a> = Rc<RefCell<Directory<'a>>>;
type DirectoryWeak<'a> = Weak<RefCell<Directory<'a>>>;

pub struct Directory<'a> {
    name: &'a str,
    size: usize,
    total_size: Option<usize>,
    parent: Option<DirectoryWeak<'a>>,
    subdirs: Vec<DirectoryRef<'a>>,
}

impl<'a> Directory<'a> {
    fn root() -> Self {
        Self {
            name: "/",
            size: 0,
            total_size: None,
            parent: None,
            subdirs: Vec::new(),
        }
    }

    fn new(name: &'a str, parent: DirectoryWeak<'a>) -> Self {
        Self {
            name,
            size: 0,
            total_size: None,
            parent: Some(parent),
            subdirs: Vec::new(),
        }
    }

    fn has_subdir(&mut self, dir_name: &str) -> bool {
        self.subdirs.iter().any(|dir| dir.borrow().name == dir_name)
    }

    fn total_size(&mut self) -> usize {
        if let Some(cached_result) = self.total_size {
            return cached_result;
        }

        let total_size = self
            .subdirs
            .iter()
            .map(|x| x.borrow_mut().total_size())
            .sum::<usize>()
            + self.size;
        self.total_size = Some(total_size);
        total_size
    }

    fn clear_total_size(&mut self) {
        if self.total_size.is_none() {
            return;
        }

        self.total_size = None;

        let Some(parent) = self.parent.clone() else { return; };
        parent.upgrade().unwrap().borrow_mut().clear_total_size();
    }

    fn get_answer(&mut self, mut current_best: usize, target: usize) -> usize {
        let choose = |size, current_best| {
            if target < size && size < current_best {
                size
            } else {
                current_best
            }
        };

        current_best = choose(self.total_size(), current_best);

        for subdir in &self.subdirs {
            current_best = choose(
                subdir.borrow_mut().get_answer(current_best, target),
                current_best,
            );
        }

        current_best
    }
}

impl<'a> Display for Directory<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn aux<'a>(dir: impl Deref<Target = Directory<'a>>, depth: usize, out: &mut Vec<String>) {
            let s = if let Some(total_size) = dir.total_size {
                format!(
                    "{space:width$}- dir {} {} | {}",
                    dir.name,
                    dir.size,
                    total_size,
                    space = ' ',
                    width = depth
                )
            } else {
                format!(
                    "{space:width$}- dir {} {}",
                    dir.name,
                    dir.size,
                    space = ' ',
                    width = depth
                )
            };
            out.push(s);

            for subdir in &dir.subdirs {
                aux(subdir.borrow(), depth + 2, out);
            }
        }

        let mut strings = Vec::new();
        aux(self, 0, &mut strings);
        f.write_str(strings.join("\n").as_str())
    }
}

struct FileSystem<'a> {
    root: DirectoryRef<'a>,
    cwd: DirectoryRef<'a>,
}

impl<'a> FileSystem<'a> {
    fn new() -> Self {
        let root = Rc::new(RefCell::new(Directory::root()));
        Self {
            root: root.clone(),
            cwd: root.clone(),
        }
    }

    fn add_child(&mut self, dir_name: &'a str) {
        let mut cwd = self.cwd.borrow_mut();
        if !cwd.has_subdir(dir_name) {
            cwd.subdirs.push(Rc::new(RefCell::new(Directory::new(
                dir_name,
                Rc::downgrade(&self.cwd),
            ))));
        }
    }

    fn update(&mut self, ls_result: &LsResult<'a>) {
        for &dir_name in &ls_result.subdirs {
            self.add_child(dir_name)
        }

        let mut cwd = self.cwd.borrow_mut();
        cwd.size = ls_result.size;
        cwd.clear_total_size();
    }

    fn cd_root(&mut self) {
        self.cwd = self.root.clone();
    }

    fn cd_parent(&mut self) {
        let Some(parent) = self.cwd.borrow().parent.clone() else { return; };
        self.cwd = parent
            .upgrade()
            .expect("we own the root node so this should always be valid as long as we're valid.");
    }

    fn cd_dir(&mut self, dir_name: &str) -> Result<(), String> {
        let Some(new_cwd) = self
            .cwd
            .borrow()
            .subdirs
            .iter()
            .find(|x| x.borrow().name == dir_name)
            .cloned()
        else {
            return Err(format!("directory {} does not contain sub-directory {}", self.cwd.borrow().name, dir_name));
        };

        self.cwd = new_cwd;
        Ok(())
    }

    fn execute(&mut self, command: Command<'a>) -> Result<(), String> {
        match command {
            Command::CD("/") => {
                self.cd_root();
                Ok(())
            }
            Command::CD("..") => {
                self.cd_parent();
                Ok(())
            }
            Command::CD(dir_name) => self.cd_dir(dir_name),
            Command::LS(ls_result) => {
                self.update(&ls_result);
                Ok(())
            }
        }
    }

    fn get_answer(&mut self) -> usize {
        const TOTAL_DISK_SPACE: usize = 70000000;
        const REQUIRED_SPACE: usize = 30000000;

        let mut root = self.root.borrow_mut();
        let free_space = TOTAL_DISK_SPACE - root.total_size();
        root.get_answer(REQUIRED_SPACE, REQUIRED_SPACE - free_space)
    }
}

impl Display for FileSystem<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("file structure:\n")?;
        writeln!(f, "{}", self.root.borrow())?;
        f.write_str("current directory: ")?;
        f.write_str(self.cwd.borrow().name)
    }
}

struct LsResult<'a> {
    size: usize,
    subdirs: Vec<&'a str>,
}

impl<'a> LsResult<'a> {
    fn new() -> Self {
        Self {
            size: 0,
            subdirs: Vec::new(),
        }
    }

    fn process_line(&mut self, line: &'a str) -> Option<()> {
        let mut iter = line.split_whitespace();
        let head = iter.next()?;
        if head == "dir" {
            self.subdirs.push(iter.next()?);
            Some(())
        } else if let Ok(item_size) = head.parse::<usize>() {
            self.size += item_size;
            Some(())
        } else {
            None
        }
    }
}

impl<'a> TryFrom<&'a str> for LsResult<'a> {
    type Error = String;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let mut ls_result = LsResult::new();

        for line in s.lines() {
            if ls_result.process_line(line).is_none() {
                return Err(format!(
                    "malformed `ls` result: \"{}\": failed to parse line: \"{}\"",
                    s, line
                ));
            }
        }

        Ok(ls_result)
    }
}

pub fn main() -> Result<(), String> {
    let mut working_directory = FileSystem::new();

    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .map_err(|e| e.to_string())?;

    for s in input.split('$').skip_while(|s| s.is_empty()) {
        let command = Command::try_from(s)?;
        working_directory.execute(command)?;
    }

    working_directory.root.borrow_mut().total_size();
    println!("{}", working_directory);
    println!("{}", working_directory.get_answer());

    Ok(())
}
