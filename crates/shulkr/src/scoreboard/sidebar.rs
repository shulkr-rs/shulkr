use parking_lot::{Mutex, RwLock};

use crate::{
    entity::Player,
    protocol::packet::{
        DisplayObjectivePacket, ResetScorePacket, UpdateObjectivesAction, UpdateObjectivesPacket,
        UpdateScorePacket,
    },
    scoreboard::{NumberFormat, RenderType},
    text::TextComponent,
    util::{HashMap, Viewable, Viewers},
};

pub struct Sidebar {
    objective: String,
    title: Mutex<TextComponent>,
    lines: RwLock<HashMap<String, SidebarLine>>,
    viewers: Viewers,
}

impl Sidebar {
    pub fn new(objective: impl Into<String>, title: impl Into<TextComponent>) -> Self {
        Self {
            objective: objective.into(),
            title: Mutex::new(title.into()),
            lines: RwLock::new(HashMap::default()),
            viewers: Viewers::new(),
        }
    }

    pub fn title(&self) -> TextComponent {
        self.title.lock().clone()
    }

    pub fn set_title(&self, title: impl Into<TextComponent>) {
        *self.title.lock() = title.into();

        for player in self.viewers.iter() {
            player.send_packet(&UpdateObjectivesPacket {
                objective_name: self.objective.clone(),
                action: UpdateObjectivesAction::UpdateScoreboard {
                    value: self.title(),
                    ty: RenderType::Integer,
                    number_format: None,
                },
            });
        }
    }

    pub fn insert_line(&self, id: impl Into<String>, mut line: SidebarLine) {
        let id = id.into();

        line.enity_name = Some(id.clone());
        line.objective_name = Some(self.objective.clone());

        self.viewers.broadcast_packet(&line.create_packet());

        let mut lines = self.lines.write();
        lines.insert(id, line);
    }

    pub fn get_line(&self, id: impl Into<String>) -> Option<SidebarLine> {
        let lines = self.lines.read();
        lines.get(&id.into()).cloned()
    }

    pub fn update_line<F>(&self, id: impl Into<String>, mut f: F)
    where
        F: FnMut(&mut SidebarLine),
    {
        let id = id.into();
        let mut lines = self.lines.write();

        let Some(mut line) = lines.remove(&id) else {
            return;
        };
        f(&mut line);

        lines.insert(id, line);

        for line in lines.values() {
            self.viewers.broadcast_packet(&line.create_packet());
        }
    }

    pub fn remove_line(&self, id: impl Into<String>) -> Option<SidebarLine> {
        let mut lines = self.lines.write();
        let line = lines.remove(&id.into());

        if let Some(line) = &line {
            self.viewers.broadcast_packet(&line.destory_packet());
        }

        line
    }
}

impl Viewable for Sidebar {
    fn add_viewer(&self, player: Player) {
        player.send_packet(&UpdateObjectivesPacket {
            objective_name: self.objective.clone(),
            action: UpdateObjectivesAction::CreateScoreboard {
                value: self.title(),
                ty: RenderType::Integer,
                number_format: None,
            },
        });

        let lines = self.lines.read();
        for line in lines.values() {
            player.send_packet(&line.create_packet());
        }
        drop(lines);

        player.send_packet(&DisplayObjectivePacket {
            position: DisplayObjectivePacket::SIDEBAR,
            score_name: self.objective.clone(),
        });

        self.viewers.add_viewer(player);
    }

    fn remove_viewer(&self, player: Player) {
        player.send_packet(&UpdateObjectivesPacket {
            objective_name: self.objective.clone(),
            action: UpdateObjectivesAction::RemoveScoreboard,
        });

        self.viewers.remove_viewer(player);
    }

    fn viewers(&self) -> &Viewers {
        &self.viewers
    }
}

#[derive(Clone)]
pub struct SidebarLine {
    text: TextComponent,
    score: i32,
    format: Option<NumberFormat>,

    enity_name: Option<String>,
    objective_name: Option<String>,
}

impl SidebarLine {
    pub fn new(text: impl Into<TextComponent>, score: i32, format: Option<NumberFormat>) -> Self {
        Self {
            text: text.into(),
            score,
            format,

            enity_name: None,
            objective_name: None,
        }
    }

    pub fn set_text(&mut self, text: impl Into<TextComponent>) {
        self.text = text.into()
    }

    fn create_packet(&self) -> UpdateScorePacket {
        UpdateScorePacket {
            entity_name: self.enity_name.clone().unwrap(),
            objective_name: self.objective_name.clone().unwrap(),
            value: self.score,
            display_name: Some(self.text.clone()),
            number_format: self.format.clone(),
        }
    }

    fn destory_packet(&self) -> ResetScorePacket {
        ResetScorePacket {
            entity_name: self.enity_name.clone().unwrap(),
            objective_name: self.objective_name.clone().unwrap(),
        }
    }
}
