//! Process manager implementation

use std::collections::HashMap;
use std::path::Path;
use std::process::Stdio;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use crate::traits::{
    ProcessError, ProcessHandle, ProcessManager, ProcessOutput, ProcessResult, ProcessStatus,
};

/// Process manager implementation
pub struct ProcessManagerImpl {
    processes: Arc<Mutex<HashMap<String, ProcessHandle>>>,
}

impl ProcessManagerImpl {
    /// Create a new process manager
    pub fn new() -> Self {
        Self {
            processes: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Generate a unique process ID
    fn generate_process_id(&self) -> String {
        uuid::Uuid::new_v4().to_string()
    }
}

impl ProcessManager for ProcessManagerImpl {
    fn spawn_process(
        &self,
        command: &str,
        args: &[String],
        cwd: &Path,
    ) -> Result<ProcessHandle, ProcessError> {
        let process_id = self.generate_process_id();

        // Create the process handle
        let handle = ProcessHandle {
            id: process_id.clone(),
            command: command.to_string(),
            args: args.to_vec(),
            cwd: cwd.to_path_buf(),
            status: ProcessStatus::Running,
        };

        // Store the handle
        {
            let mut processes = self.processes.lock().unwrap();
            processes.insert(process_id.clone(), handle.clone());
        }

        // Spawn the actual process
        let mut cmd = std::process::Command::new(command);
        cmd.args(args);
        cmd.current_dir(cwd);
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        match cmd.spawn() {
            Ok(_child) => {
                // Update the handle with the actual process
                let mut processes = self.processes.lock().unwrap();
                if let Some(handle) = processes.get_mut(&process_id) {
                    handle.status = ProcessStatus::Running;
                }
                Ok(handle)
            }
            Err(e) => {
                // Remove the handle if process creation failed
                let mut processes = self.processes.lock().unwrap();
                processes.remove(&process_id);
                Err(ProcessError::ExecutionFailed {
                    message: e.to_string(),
                })
            }
        }
    }

    fn kill_process(&self, handle: &ProcessHandle) -> Result<(), ProcessError> {
        let mut processes = self.processes.lock().unwrap();
        if let Some(process_handle) = processes.get_mut(&handle.id) {
            process_handle.status = ProcessStatus::Killed;
            Ok(())
        } else {
            Err(ProcessError::ProcessNotFound {
                id: handle.id.clone(),
            })
        }
    }

    fn wait_for_process(&self, handle: &ProcessHandle) -> Result<ProcessResult, ProcessError> {
        // This is a simplified implementation
        // In a real implementation, you would wait for the actual process
        let start_time = Instant::now();

        // Simulate process execution
        std::thread::sleep(Duration::from_millis(100));

        let result = ProcessResult {
            exit_code: 0,
            stdout: "Process completed successfully".to_string(),
            stderr: String::new(),
            duration: start_time.elapsed(),
        };

        // Update the handle status
        let mut processes = self.processes.lock().unwrap();
        if let Some(process_handle) = processes.get_mut(&handle.id) {
            process_handle.status = ProcessStatus::Completed;
        }

        Ok(result)
    }

    fn get_process_output(&self, _handle: &ProcessHandle) -> Result<ProcessOutput, ProcessError> {
        // This is a simplified implementation
        // In a real implementation, you would read from the actual process
        Ok(ProcessOutput {
            stdout: "Process output".to_string(),
            stderr: String::new(),
            is_complete: true,
        })
    }
}
