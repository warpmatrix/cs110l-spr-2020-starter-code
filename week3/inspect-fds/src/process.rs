use crate::open_file::OpenFile;
use std::fs;
use crate::ps_utils;

#[derive(Debug, Clone, PartialEq)]
pub struct Process {
    pub pid: usize,
    pub ppid: usize,
    pub command: String,
}

impl Process {
    pub fn new(pid: usize, ppid: usize, command: String) -> Process {
        Process { pid, ppid, command }
    }

    /// This function returns a list of file descriptor numbers for this Process, if that
    /// information is available (it will return None if the information is unavailable). The
    /// information will commonly be unavailable if the process has exited. (Zombie processes
    /// still have a pid, but their resources have already been freed, including the file
    /// descriptor table.)
    pub fn list_fds(&self) -> Option<Vec<usize>> {
        let fd_path = format!("/proc/{}/fd", self.pid);
        let entries = fs::read_dir(fd_path).ok()?;
        let mut fds = vec![];
        for entry in entries {
            let entry = entry.ok()?;
            let fd_str = entry.file_name().into_string().ok()?;
            let fd = fd_str.parse::<usize>().ok()?;
            fds.push(fd);
        }
        Some(fds)
    }

    /// This function returns a list of (fdnumber, OpenFile) tuples, if file descriptor
    /// information is available (it returns None otherwise). The information is commonly
    /// unavailable if the process has already exited.
    pub fn list_open_files(&self) -> Option<Vec<(usize, OpenFile)>> {
        let mut open_files = vec![];
        for fd in self.list_fds()? {
            open_files.push((fd, OpenFile::from_fd(self.pid, fd)?));
        }
        Some(open_files)
    }

    pub fn print(&self) {
        println!(
            "========== \"{}\" (pid {}, ppid {}) ==========",
            self.command, self.pid, self.ppid
        );
        match self.list_open_files() {
            None => println!(
                "Warning: could not inspect file descriptors for this process! \
            It might have exited just as we were about to look at its fd table, \
            or it might have exited a while ago and is waiting for the parent \
            to reap it."
            ),
            Some(open_files) => {
                for (fd, file) in open_files {
                    println!(
                        "{:<4} {:<15} cursor: {:<4} {}",
                        fd,
                        format!("({})", file.access_mode),
                        file.cursor,
                        file.colorized_name(),
                    );
                }
            }
        }

        println!();
        let child_processes = ps_utils::get_child_processes(self.pid).expect(&format!(
            "Get process {}'s child processes failed",
            self.pid
        ));
        for child_process in child_processes {
            child_process.print();
        }
    }
}

#[cfg(test)]
mod test {
    use crate::ps_utils;
    use std::process::{Child, Command};

    fn start_c_program(program: &str) -> Child {
        Command::new(program)
            .spawn()
            .expect(&format!("Could not find {}. Have you run make?", program))
    }

    #[test]
    fn test_list_fds() {
        let mut test_subprocess = start_c_program("./multi_pipe_test");
        let process = ps_utils::get_target("multi_pipe_test").unwrap().unwrap();
        assert_eq!(
            process
                .list_fds()
                .expect("Expected list_fds to find file descriptors, but it returned None"),
            vec![0, 1, 2, 4, 5]
        );
        let _ = test_subprocess.kill();
    }

    #[test]
    fn test_list_fds_zombie() {
        let mut test_subprocess = start_c_program("./nothing");
        let process = ps_utils::get_target("nothing").unwrap().unwrap();
        assert!(
            process.list_fds().is_none(),
            "Expected list_fds to return None for a zombie process"
        );
        let _ = test_subprocess.kill();
    }
}
