use std::time::Duration;
use std::thread;
use std::process::Command;

use nix::sys::wait::wait;
use nix::unistd::ForkResult::{Child, Parent};
use nix::unistd::{fork, getpid, getppid};
use nix::sched::{self, CloneFlags};

// fork() example
// fn main() {
//     println!("Hello, world!");
//     let pid = fork();

//     match pid.expect("Fork Failed: Unable to create child process!") {
//         Child => println!(
//             "Hello from child process with pid: {} and parent pid:{}",
//             getpid(),
//             getppid()
//         ),
//         Parent { child } => {
//             wait();
//             println!(
//                 "Hello from parent process with pid: {} and child pid:{}",
//                 getpid(),
//                 child
//             );
//         }
//     }
// }

// clone() example
fn child() -> isize {
    Command::new("ip")
        .arg("link")
        .spawn()
        .expect("ip command failed to start");
    thread::sleep(Duration::from_secs(1));
    // return 0
    0
}

fn main() {
    const STACK_SIZE: usize = 1024 * 1024;
    let ref mut stack = [0; STACK_SIZE];

    let flags = CloneFlags::CLONE_NEWUSER 
        | CloneFlags::CLONE_NEWPID 
        | CloneFlags::CLONE_NEWNET 
        | CloneFlags::CLONE_NEWNS
        | CloneFlags::CLONE_DETACHED;

    let p = sched::clone(Box::new(child), stack, flags, None)
                .expect("Failed to spawn the child");
    println!("pid: {}", p);
}
