use std::fmt;
use std::fs::{self, OpenOptions};
use std::io::{Write, self};

#[derive(Debug, Clone)]
struct DuplicateTaskError;

impl fmt::Display for DuplicateTaskError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Task already exists")
    }
}


#[derive(Debug)]
pub struct Task {
    pub title: String,
    pub id: String
}

impl Task {
    pub fn new(title: &String, id: &String) -> Self {
        let t = Task {
            title: title.clone(), 
            id: id.clone() 
        };
        println!("Creating task with title: {} and id: {}", title, id);
        t
    }
    pub fn write(&self, p: &String) {

        
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(p)
            .unwrap();
        
        if let Err(e) = writeln!(file, "{}:{}", self.id, self.title) {
            eprintln!("Couldnt write to file: {}", e);
        }
    }
    pub fn parse(string: String) -> Option<Self> {
        let splt: Vec<&str> = string.split(":").collect();
        
        if splt.len() != 2 { () }
        
        Some(Task { 
            title: splt[1].to_string(), 
            id: splt[0].to_string() 
        })
    }
}

/// [`Doit`] main struct for task management.
pub struct Doit {
    pub tasks: Vec<Task>,
    pub tasks_path: String,
    pub log_path: String
}

impl Default for Doit {
    /// Default config for [`Doit`]
    /// # Examples 
    /// 
    /// ```rust
    /// use doit::Doit;
    /// let doit = Doit::default();
    /// assert_eq!(doit.tasks_path, "./doit.txt".to_string());
    /// assert_eq!(doit.log_path, "./doit.log".to_string());
    /// ```
    fn default() -> Self {
        Doit { 
            tasks: Vec::<Task>::new(), 
            tasks_path: "./doit.txt".to_string(), 
            log_path: "./doit.log".to_string() 
        }
    }   
}

impl Doit {
    pub fn new(tasks_path: Option<String>, log_path: Option<String>) -> Self {
        Doit { 
            tasks: Vec::<Task>::new(), 
            
            tasks_path: tasks_path.unwrap_or(
                Doit::default().tasks_path
            ), 
            
            log_path: log_path.unwrap_or(
                Doit::default().log_path
            )
        }
    }
    
    /// Crate a new task. Write it it storage and logs.
    pub fn add_task(&mut self, title: String, id: Option<String>) -> Result<(), io::Error> {
        
        /*Provided id? Or generate new id? */
        let new_id = match id {
            Some(id) => {id}
            None => {(999 - self.tasks.len()).to_string()}
        };

        /*Is new id unique?*/
        for task in &self.tasks {
            if task.id == new_id {
                println!("Task with ID: {} already exists!", new_id);
                return Ok(())
            }
        }

        let new_task = Task {
            title:title, 
            id:new_id
        };

        new_task.write(&self.tasks_path);

        self.log(MainCmd::Task, new_task)


    }

    pub fn parse(&mut self) {
        
        /*Tasks as strings*/
        let raw_tasks = fs::read_to_string(&self.tasks_path).unwrap();
        
        /*Each task has its own line*/
        let tasks_it: Vec<&str> = raw_tasks.split("\n").collect();
        
        for task in tasks_it {
            let temp: Vec<&str> = task.split(":").collect();
            
            if temp.len() != 2 {
                continue;
            }
    
            self.tasks.push(Task{
                title: temp[1].to_string(), 
                id: temp[0].to_string()
            })
        }
        // println!("{:?}", tasks);
    }

    pub fn log(&self, cmd: MainCmd, task: Task) -> Result<(), std::io::Error>{
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(&self.log_path)
            .unwrap();

        match cmd {
            MainCmd::Task => {
                writeln!(file, "CreateTask:{}:{}", task.id, task.title)
            }
            MainCmd::Stage => {
                writeln!(file, "StageTask:{}", task.id)
            }
            MainCmd::Done => {
                writeln!(file, "DoneTask:{}", task.id)
            }
            MainCmd::List => { Ok(()) }
        }
    }
}


pub enum MainCmd {
    Task,
    Stage,
    Done,
    List
}

impl MainCmd {
    pub fn from_str(str: &String) -> Self {
        match str.to_lowercase().as_str() {
            "task" => {MainCmd::Task}
            "stage" => {MainCmd::Stage}
            "done" => {MainCmd::Done}
            "list" => {MainCmd::List}
            _ => {panic!("Invalid command {}!", str)}
        }
    }
}