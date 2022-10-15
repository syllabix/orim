use std::collections::HashMap;

use actix::prelude::*;

use super::{
    component::{ChatMessage, DrawnLine, Widget},
    message::{Action, Update},
};

#[derive(Debug, Clone)]
pub struct Space {
    _id: usize,
    pub users: HashMap<usize, Recipient<Update>>,
    widgets: HashMap<String, Widget>,
    chat: Vec<ChatMessage>,
    lines: HashMap<String, DrawnLine>,
}

impl Space {
    pub fn new(id: usize) -> Space {
        Space {
            _id: id,
            users: HashMap::new(),
            widgets: HashMap::new(),
            chat: vec![],
            lines: HashMap::new(),
        }
    }

    pub fn register(&mut self, user_id: usize, addr: Recipient<Update>) {
        self.users.insert(user_id, addr);
    }

    pub fn unregister(&mut self, user_id: usize) -> Option<Recipient<Update>> {
        self.users.remove(&user_id)
    }

    pub fn upsert(&mut self, action: Action) -> Action {
        match action {
            Action::Chat { payload } => {
                let msg = payload.clone();
                self.chat.push(msg);
                Action::Chat { payload }
            }
            Action::Draw { payload } => {
                let msg = payload.clone();
                let line = self.lines.entry(msg.id.clone()).or_insert(DrawnLine {
                    id: msg.id,
                    color: msg.color,
                    points: vec![],
                    action: msg.action,
                });
                line.points.push(msg.point.x);
                line.points.push(msg.point.y);
                Action::Draw { payload }
            }
            Action::Widget { payload } => {
                let id = payload.id.clone();
                self.widgets.insert(id.clone(), payload);
                Action::Widget {
                    payload: self.widgets.get(&id).unwrap().to_owned(),
                }
            }
        }
    }

    pub fn get_widgets(&self) -> Vec<Widget> {
        self.widgets.values().cloned().collect()
    }

    pub fn get_chat_history(&self) -> Vec<ChatMessage> {
        self.chat.iter().cloned().collect()
    }

    pub fn get_drawings(&self) -> Vec<DrawnLine> {
        self.lines.values().cloned().collect()
    }
}
