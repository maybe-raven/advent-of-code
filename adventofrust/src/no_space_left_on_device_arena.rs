#![allow(dead_code)]
use std::{
    fmt::{Display, Formatter, Write},
    io::{self, Read},
    num::NonZeroUsize,
    ops::{Index, IndexMut},
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

struct Directory<'a> {
    name: &'a str,
    size: usize,
    total_size: Option<usize>,
    parent_index: Option<usize>,
    subdir_indices: Vec<usize>,
}

impl Directory<'_> {
    fn root() -> Self {
        Self {
            name: "/",
            size: 0,
            total_size: None,
            parent_index: None,
            subdir_indices: Vec::new(),
        }
    }
}

impl<'a> Directory<'a> {
    fn new(name: &'a str, parent_index: usize) -> Self {
        Self {
            name,
            size: 0,
            total_size: None,
            parent_index: Some(parent_index),
            subdir_indices: Vec::new(),
        }
    }
}

struct FileSystem<'a> {
    directories: Vec<Directory<'a>>,
    cwd_index: usize,
}

impl FileSystem<'_> {
    const ROOT_INDEX: usize = 0;

    fn new() -> Self {
        Self {
            directories: vec![Directory::root()],
            cwd_index: Self::ROOT_INDEX,
        }
    }

    fn cwd(&self) -> &Directory {
        self.directories.index(self.cwd_index)
    }

    fn cd_root(&mut self) {
        self.cwd_index = Self::ROOT_INDEX;
    }

    fn cd_parent(&mut self) {
        if let Some(parent_index) = self.cwd().parent_index {
            self.cwd_index = parent_index;
        }
    }

    fn cd_dir(&mut self, dir_name: &str) -> Result<(), String> {
        if let Some(i) = self.find_subdir_index(dir_name) {
            self.cwd_index = i;
            Ok(())
        } else {
            Err(format!(
                "directory {} does not contain sub-directory {}",
                self.cwd().name,
                dir_name
            ))
        }
    }

    fn find_subdir_index(&self, dir_name: &str) -> Option<usize> {
        self.cwd()
            .subdir_indices
            .iter()
            .find(|&&i| self[i].name == dir_name)
            .copied()
    }

    fn total_size(&mut self, dir_index: usize) -> usize {
        let dir = &self[dir_index];
        if let Some(cached_result) = dir.total_size {
            return cached_result;
        }

        let mut sum = dir.size;
        for i in dir.subdir_indices.clone() {
            sum += self.total_size(i);
        }

        self[dir_index].total_size = Some(sum);

        sum
    }

    fn clear_total_size(&mut self, dir_index: usize) {
        let dir = self.index_mut(dir_index);
        if dir.total_size.is_none() {
            return;
        }

        dir.total_size = None;

        let Some(parent_index) = dir.parent_index else { return; };

        self.clear_total_size(parent_index);
    }

    fn riddle_me_dirs(&self, dir_index: usize, target: usize, mut current_best: usize) -> usize {
        let dir = &self[dir_index];
        let total_size = dir.total_size.unwrap();

        if total_size < target {
            return current_best;
        }

        if total_size < current_best {
            current_best = total_size;
        }

        for &i in &dir.subdir_indices {
            current_best = self.riddle_me_dirs(i, target, current_best);
        }

        current_best
    }

    fn riddle_me_this(&mut self) -> Option<NonZeroUsize> {
        const TOTAL_DISK_SPACE: usize = 70000000;
        const REQUIRED_SPACE: usize = 30000000;

        let free_space = TOTAL_DISK_SPACE - self.total_size(Self::ROOT_INDEX);
        let result = self.riddle_me_dirs(Self::ROOT_INDEX, REQUIRED_SPACE - free_space, usize::MAX);

        if result == usize::MAX {
            None
        } else {
            NonZeroUsize::new(result)
        }
    }
}

impl<'a> FileSystem<'a> {
    fn cwd_mut(&mut self) -> &mut Directory<'a> {
        self.directories.index_mut(self.cwd_index)
    }

    fn update(&mut self, ls_result: &LsResult<'a>) {
        for &dir_name in &ls_result.subdirs {
            self.add_child(dir_name);
        }

        self.cwd_mut().size = ls_result.size;
        self.clear_total_size(self.cwd_index);
    }

    fn add_child(&mut self, dir_name: &'a str) {
        if self.find_subdir_index(dir_name).is_some() {
            return;
        }

        let i = self.directories.len();
        self.directories
            .push(Directory::new(dir_name, self.cwd_index));
        self.cwd_mut().subdir_indices.push(i);
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

    fn fmt_aux(&self, dir_index: usize, depth: usize, out: &mut Vec<String>) {
        let dir = &self[dir_index];
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

        for &i in &dir.subdir_indices {
            self.fmt_aux(i, depth + 2, out);
        }
    }
}

impl<'a> Index<usize> for FileSystem<'a> {
    type Output = Directory<'a>;

    fn index(&self, index: usize) -> &Self::Output {
        self.directories.index(index)
    }
}

impl<'a> IndexMut<usize> for FileSystem<'a> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.directories.index_mut(index)
    }
}

impl Display for FileSystem<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut strings = Vec::new();
        self.fmt_aux(Self::ROOT_INDEX, 0, &mut strings);

        f.write_str("file structure:\n")?;
        f.write_str(strings.join("\n").as_str())?;
        f.write_char('\n')?;
        f.write_str("current directory: ")?;
        f.write_str(self.cwd().name)
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

    working_directory.total_size(FileSystem::ROOT_INDEX);
    println!("{}", working_directory);
    if let Some(result) = working_directory.riddle_me_this() {
        println!("{}", result.get());
    } else {
        println!("shit doesn't work.");
    }

    Ok(())
}
