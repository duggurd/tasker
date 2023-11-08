use std::error::Error;
use std::iter::Cloned;
use std::{env::args, fs};
use std::fs::OpenOptions;
use std::io::{Write};

fn main() {
    let argv = args();
    let arg_s: Vec<String> = argv.collect();
    
    let main_cmd = MainCmd::from_str(&arg_s[1]);

    let mut doit = Doit::default();
    doit.parse();
    
    match main_cmd {
        MainCmd::Task => {
            if arg_s.len() <= 2 {
                panic!("Not enough arguments in task");
            } else {
                doit.add_task(arg_s[2].to_string(),  arg_s.get(3).cloned() );
            }
        }
        MainCmd::Stage => {println!("stagin task X")}
        MainCmd::Done => {println!("Done with task X")}
        MainCmd::List => {println!("Listing tasks")}
    }
}


#[derive(Debug)]
pub struct Task {
    title: String,
    id: String
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

pub struct Doit {
    tasks: Vec<Task>,
    tasks_path: String,
    log_path: String
}

impl Doit {
       
    pub fn default() -> Self {
        Doit { 
            tasks: vec![], 
            tasks_path: "./doit.txt".to_string(), 
            log_path: "./doit.log".to_string() 
        }
    }   
    pub fn new(tasks_path: Option<String>, log_path: Option<String>) -> Self {
        let default = Doit::default();

        Doit { 
            tasks: vec![], 
            tasks_path: tasks_path.unwrap_or(default.tasks_path), 
            log_path: log_path.unwrap_or(default.log_path)
        }
    }

    pub fn add_task(&mut self, title: String, id: Option<String>) {
        
        /*Provided id? Or generate new id? */
        let new_id = match id {
            Some(id) => {id}
            None => {(999 - self.tasks.len()).to_string()}
        };

        /*Is new id unique?*/
        for task in &self.tasks {
            if task.id == new_id {
                println!("Task with ID: {} already exists!", new_id);
                return;
            }
        }

        let new_task = Task {
            title:title, 
            id:new_id
        };

        new_task.write(&self.tasks_path);

        if let Err(e) = self.log(MainCmd::Task, new_task) {
            eprintln!("Error loggin: {}", e);
        }

        // self.tasks.push(new_task)


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
