
use std::collections::HashMap;

use godot::prelude::*;
use ps_env::PSValue;

mod preprocessor;
mod ps_env;
mod interpreter;
mod reader;

#[derive(GodotClass)]
#[class(base=Node)]
struct PipeScript {
    lines: Option<Vec<Vec<PSValue>>>,
    variable_hash: Option<HashMap<String, usize>>,
    env: Option<Vec<PSValue>>,

    base: Base<Node>
}

#[godot_api]
impl INode for PipeScript {
    fn init(base: Base<Node>) -> Self {
        PipeScript { lines: None, variable_hash: None, env: None, base }
    }
}

#[godot_api]
impl PipeScript {
    #[func]
    fn set_object_variable(&mut self, key_gd: String, obj_ref: Gd<Object>) {
        assert!(self.lines.is_some(), "Cannot insert variables, source isn't ready yet! Make sure to invoke `pipescript.interpret(source: String)` first.");
        let (variable_hash, env) = (
            self.variable_hash.as_mut().unwrap(),
            self.env.as_mut().unwrap()
        );
        let key_str = key_gd.to_string();

        let key = if let Some(id) = variable_hash.get(&key_str) {
            PSValue::VarIndex(*id)
        } else {
            let id = env.len();
            env.push(PSValue::None);
            variable_hash.insert(
                key_str,
                id
            );
            PSValue::VarIndex(id)
        };

        ps_env::set_variable(&key, PSValue::GodotObject(obj_ref), env);
    }

    #[func]
    fn execute(&mut self) {
        assert!(self.lines.is_some(), "Cannot execute, source isn't ready yet! Make sure to invoke `pipescript.interpret(source: String)` first.");
        let (lines, variable_hash, env) = (
            self.lines.as_mut().unwrap(),
            self.variable_hash.as_mut().unwrap(),
            self.env.as_mut().unwrap()
        );
        // Pre-process
        preprocessor::preprocess(lines, env, variable_hash);
        // Execute the commands
        interpreter::execute_commands(lines, env);
    }

    #[func]
    fn debug_print_commands(&self) {
        assert!(self.lines.is_some(), "Cannot execute, source isn't ready yet! Make sure to invoke `pipescript.interpret(source: String)` first.");
        let lines = self.lines.as_ref().unwrap();
        godot_print!("--- PRINT CMDS ---");
        for idx in 0..lines.len() {
            let mut print_msg = format!(" {}: ", idx);
            for str in lines.get(idx).unwrap() {
                print_msg += str.to_string().as_str();
                print_msg += " ";
            };
            godot_print!("{}", print_msg);
        };
        godot_print!("--- ---------- ---");
    }

    #[func]
    fn interpret(&mut self, source: String) {
        let (lines, variable_hash, env) = reader::source_to_instructions(
            source.to_string()
        );

        self.lines = Some(lines);
        self.variable_hash = Some(variable_hash);
        self.env = Some(env);
    }
}

