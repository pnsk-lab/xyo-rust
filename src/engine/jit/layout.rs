use super::*;
use std::collections::{HashMap, HashSet};

impl<'ctx, 'm> JitCompiler<'ctx, 'm> {
    pub(super) fn compile_program(&mut self, program: &Program) -> Result<CompilationLayout> {
        self.build_target_layout(program);
        self.predeclare_scripts(&program.scripts);
        self.build_script_target_layout(program);
        self.predeclare_procedures(&program.procedures);
        self.build_procedure_warp_flags(&program.procedures);
        self.build_message_layout(program);
        self.build_key_press_layout(program);
        self.build_clone_layout(program);

        for (index, procedure) in program.procedures.iter().enumerate() {
            self.compile_procedure(index, procedure)?;
        }

        for script in &program.scripts {
            self.compile_script(script)?;
        }

        let entry_script_ids = program
            .scripts
            .iter()
            .filter(|script| matches!(script.trigger, ScriptTrigger::GreenFlag))
            .filter_map(|script| self.script_id_by_name.get(&script.name).copied())
            .collect::<Vec<_>>();

        Ok(CompilationLayout {
            script_names_by_id: self.script_names_by_id.clone(),
            entry_script_ids,
            broadcast_messages: self.broadcast_messages.clone(),
            broadcast_targets: self.broadcast_targets.clone(),
            key_press_options: self.key_press_options.clone(),
            key_press_targets: self.key_press_targets.clone(),
            clone_targets: self.clone_targets.clone(),
            script_target_ids: self.script_target_ids.clone(),
            target_count: self.target_index_by_name.len(),
        })
    }

    fn predeclare_scripts(&mut self, scripts: &[Script]) {
        let fn_type = self
            .context
            .void_type()
            .fn_type(&[self.ptr_type.into()], false);
        for (script_id, script) in scripts.iter().enumerate() {
            let function = self.module.add_function(&script.name, fn_type, None);
            self.script_functions.insert(script.name.clone(), function);
            self.script_id_by_name
                .insert(script.name.clone(), script_id as u64);
            self.script_names_by_id.push(script.name.clone());
        }
    }

    fn build_target_layout(&mut self, program: &Program) {
        self.target_index_by_name.clear();
        for (target_index, target_name) in program.target_names.iter().enumerate() {
            self.target_index_by_name
                .insert(target_name.clone(), target_index as u64);
        }
        if self.target_index_by_name.is_empty() {
            self.target_index_by_name.insert("default".to_string(), 0);
        }
    }

    fn build_script_target_layout(&mut self, program: &Program) {
        self.script_target_ids = program
            .scripts
            .iter()
            .map(|script| {
                self.target_index_by_name
                    .get(&script.target_name)
                    .copied()
                    .unwrap_or(0)
            })
            .collect();
    }

    fn predeclare_procedures(&mut self, procedures: &[Procedure]) {
        for (index, procedure) in procedures.iter().enumerate() {
            let mut params: Vec<BasicMetadataTypeEnum<'ctx>> =
                Vec::with_capacity(1 + procedure.arg_names.len());
            params.push(self.ptr_type.into());
            params.extend(
                procedure
                    .arg_names
                    .iter()
                    .map(|_| BasicMetadataTypeEnum::from(self.f64_type)),
            );

            let function = self.module.add_function(
                &procedure.name,
                self.f64_type.fn_type(&params, false),
                None,
            );
            self.procedure_functions.insert(index, function);
        }
    }

    fn build_procedure_warp_flags(&mut self, procedures: &[Procedure]) {
        for (index, procedure) in procedures.iter().enumerate() {
            self.procedure_warp_flags.insert(index, procedure.warp);
        }
    }

    fn build_clone_layout(&mut self, program: &Program) {
        self.clone_targets = vec![Vec::new(); self.target_index_by_name.len()];
        for (script_index, script) in program.scripts.iter().enumerate() {
            if !matches!(script.trigger, ScriptTrigger::CloneStart) {
                continue;
            }
            let Some(target_index) = self.target_index_by_name.get(&script.target_name).copied()
            else {
                continue;
            };
            if let Some(bucket) = self.clone_targets.get_mut(target_index as usize) {
                bucket.push(script_index as u64);
            }
        }
    }

    fn build_message_layout(&mut self, program: &Program) {
        let mut messages = HashSet::new();

        for script in &program.scripts {
            if let ScriptTrigger::Broadcast(message) = &script.trigger {
                messages.insert(message.clone());
            }
        }

        let mut ordered = messages.into_iter().collect::<Vec<_>>();
        ordered.sort();
        self.broadcast_messages = ordered.clone();

        self.message_index_by_name.clear();
        for (index, message) in ordered.iter().enumerate() {
            self.message_index_by_name
                .insert(message.clone(), index as u64);
        }

        self.broadcast_targets = vec![Vec::new(); ordered.len()];
        for script in &program.scripts {
            let ScriptTrigger::Broadcast(message) = &script.trigger else {
                continue;
            };
            let Some(message_index) = self.message_index_by_name.get(message).copied() else {
                continue;
            };
            let Some(script_id) = self.script_id_by_name.get(&script.name).copied() else {
                continue;
            };
            self.broadcast_targets[message_index as usize].push(script_id);
        }
    }

    fn build_key_press_layout(&mut self, program: &Program) {
        let mut options = HashSet::new();
        for script in &program.scripts {
            if let ScriptTrigger::KeyPressed(key) = &script.trigger {
                options.insert(normalize_key_name(key));
            }
        }

        let mut ordered = options.into_iter().collect::<Vec<_>>();
        ordered.sort();
        self.key_press_options = ordered.clone();

        let option_index_by_name = ordered
            .iter()
            .enumerate()
            .map(|(index, key)| (key.clone(), index))
            .collect::<HashMap<_, _>>();

        self.key_press_targets = vec![Vec::new(); ordered.len()];
        for script in &program.scripts {
            let ScriptTrigger::KeyPressed(key) = &script.trigger else {
                continue;
            };
            let normalized = normalize_key_name(key);
            let Some(option_index) = option_index_by_name.get(&normalized).copied() else {
                continue;
            };
            let Some(script_id) = self.script_id_by_name.get(&script.name).copied() else {
                continue;
            };
            self.key_press_targets[option_index].push(script_id);
        }
    }
}

fn normalize_key_name(raw: &str) -> String {
    let lowered = raw.trim().to_ascii_lowercase();
    match lowered.as_str() {
        "left" | "left arrow" => "left arrow".to_string(),
        "right" | "right arrow" => "right arrow".to_string(),
        "up" | "up arrow" => "up arrow".to_string(),
        "down" | "down arrow" => "down arrow".to_string(),
        "space" | "spacebar" => "space".to_string(),
        _ => lowered,
    }
}
