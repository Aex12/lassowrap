use clap::{Parser, ValueEnum};
use std::num::ParseIntError;
use std::path::PathBuf;
use std::process::Command;
use std::str::FromStr;
use windows::Win32::System::Threading::{
    ABOVE_NORMAL_PRIORITY_CLASS, BELOW_NORMAL_PRIORITY_CLASS, HIGH_PRIORITY_CLASS,
    IDLE_PRIORITY_CLASS, NORMAL_PRIORITY_CLASS, PROCESS_CREATION_FLAGS, PROCESS_SET_INFORMATION,
    REALTIME_PRIORITY_CLASS,
};
use windows::Win32::System::Threading::{OpenProcess, SetPriorityClass, SetProcessAffinityMask};

#[derive(Debug, Clone, ValueEnum)]
#[value(rename_all = "lower")]
enum Priority {
    Low,
    BelowNormal,
    Normal,
    AboveNormal,
    High,
    Realtime,
}

impl Into<PROCESS_CREATION_FLAGS> for Priority {
    fn into(self) -> PROCESS_CREATION_FLAGS {
        match self {
            Priority::Low => IDLE_PRIORITY_CLASS,
            Priority::BelowNormal => BELOW_NORMAL_PRIORITY_CLASS,
            Priority::Normal => NORMAL_PRIORITY_CLASS,
            Priority::AboveNormal => ABOVE_NORMAL_PRIORITY_CLASS,
            Priority::High => HIGH_PRIORITY_CLASS,
            Priority::Realtime => REALTIME_PRIORITY_CLASS,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Affinity(usize);

impl Into<usize> for Affinity {
    fn into(self) -> usize {
        self.0
    }
}

impl FromStr for Affinity {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let val = if let Some(hex) = s.strip_prefix("0x") {
            usize::from_str_radix(hex, 16)?
        } else {
            s.parse::<usize>()?
        };
        Ok(Affinity(val))
    }
}

#[derive(Debug, Parser)]
#[command()]
struct Params {
    /// Process priority
    #[arg(short, long, value_enum)]
    priority: Option<Priority>,

    /// CPU affinity mask (e.g. 0xFFFF000)
    #[arg(short, long)]
    affinity: Option<Affinity>,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,

    /// The program to run
    executable: PathBuf,

    /// Arguments to pass to the program (captures everything that follows)
    #[arg(trailing_var_arg(true))]
    executable_parameters: Vec<String>,
}

fn main() {
    let params = Params::parse();
    if params.verbose {
        println!("{:#?}", params);
    }

    let mut child = Command::new(&params.executable)
        .args(&params.executable_parameters)
        .spawn()
        .expect("Failed to launch program");

    let pid = child.id();

    if params.verbose {
        println!("Launched process with PID: {}", pid);
    }

    #[cfg(windows)]
    unsafe {
        let handle = OpenProcess(PROCESS_SET_INFORMATION, false, pid)
            .expect("Failed to open process handle");

        if let Some(affinity) = params.affinity {
            SetProcessAffinityMask(handle, affinity.into()).expect("Failed to set affinity mask");
        }

        if let Some(priority) = params.priority {
            SetPriorityClass(handle, priority.into()).expect("Failed to set priority");
        }
    }

    let status = child.wait().expect("Failed to wait on child");
    std::process::exit(status.code().unwrap_or(1));
}
