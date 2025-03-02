use zellij_tile::prelude::*;

use std::collections::BTreeMap;

#[derive(Default)]
struct State {
    sessions: BTreeMap<String, bool>,
    current_session: Option<String>,
}

register_plugin!(State);

// TODO: maybe use the /cache directory?
const SESSION_RECORD: &str = "/tmp/sessions.txt";

// We store the most recent 2 session names in SESSION_RECORD:
// <last_session>
// <current_session>
fn read_sessions() -> Vec<String> {
    let res = std::fs::read_to_string(SESSION_RECORD);
    if let Ok(content) = res {
        content.lines().map(String::from).collect()
    } else {
        Vec::new()
    }
}

// Write the 2 most recent session names to SESSION_RECORD.
fn push_session_name(session_name: String) {
    let mut sessions = read_sessions();

    if sessions.is_empty() || sessions.last().map_or(true, |last| last != &session_name) {
        sessions.push(session_name.clone());

        if sessions.len() > 2 {
            sessions = sessions.split_off(sessions.len() - 2);
        }

        if let Some(parent) = std::path::Path::new(SESSION_RECORD).parent() {
            std::fs::create_dir_all(parent).expect("Failed to create directories");
        }
        let _ = std::fs::write(SESSION_RECORD, sessions.join("\n"));
    }
}

impl ZellijPlugin for State {
    fn load(&mut self, _configuration: BTreeMap<String, String>) {
        request_permission(&[
            PermissionType::ReadApplicationState,
            PermissionType::ChangeApplicationState,
        ]);
        subscribe(&[EventType::PermissionRequestResult, EventType::SessionUpdate]);
    }

    fn update(&mut self, event: Event) -> bool {
        let should_render = false;

        match event {
            Event::SessionUpdate(info, _ignore) => {
                let session_map: BTreeMap<String, bool> = info
                    .iter()
                    .map(|session| (session.name.clone(), session.is_current_session))
                    .collect();

                self.sessions = session_map.clone();

                for (name, is_current_session) in &session_map {
                    if !is_current_session {
                        continue;
                    }

                    if self.current_session.is_none() || self.current_session != Some(name.clone())
                    {
                        push_session_name(name.clone());
                    }

                    self.current_session = Some(name.clone());

                    break;
                }
            }
            Event::Visible(_visible) => {}
            _ => {}
        }

        should_render
    }
    fn pipe(&mut self, pipe_message: PipeMessage) -> bool {
        let should_render = false;

        if Some("switch-to-last-session") == pipe_message.payload.as_deref() {
            let sessions = read_sessions();
            if let Some(first_session) = sessions.first() {
                if self.sessions.contains_key(first_session) {
                    eprintln!("[switch] switch to session: {:?}", first_session);
                    switch_session(Some(first_session));
                }
            }
        }

        should_render
    }
}
