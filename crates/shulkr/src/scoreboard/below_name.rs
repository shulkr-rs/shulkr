use crate::{
    entity::Player,
    protocol::packet::{
        DisplayObjectivePacket, ResetScorePacket, UpdateObjectivesAction, UpdateObjectivesPacket,
        UpdateScorePacket,
    },
    scoreboard::RenderType,
    text::TextComponent,
    util::{HashMap, RwLock, Viewable, Viewers},
};

pub struct BelowName {
    objective: String,
    text: TextComponent,
    scores: RwLock<HashMap<String, i32>>,
    viewers: Viewers,
}

impl BelowName {
    pub fn new(objective: impl Into<String>, text: impl Into<TextComponent>) -> Self {
        Self {
            objective: objective.into(),
            text: text.into(),
            scores: RwLock::new(HashMap::default()),
            viewers: Viewers::new(),
        }
    }

    pub fn set_score(&self, player_name: impl Into<String>, score: i32) {
        let player_name = player_name.into();
        self.scores.write().insert(player_name.clone(), score);
        self.viewers.broadcast_packet(&UpdateScorePacket {
            entity_name: player_name,
            objective_name: self.objective.clone(),
            value: score,
            display_name: None,
            number_format: None,
        });
    }

    pub fn remove_score(&self, player_name: impl Into<String>) {
        let player_name = player_name.into();
        self.scores.write().remove(&player_name);
        self.viewers.broadcast_packet(&ResetScorePacket {
            entity_name: player_name,
            objective_name: self.objective.clone(),
        });
    }
}

impl Viewable for BelowName {
    fn add_viewer(&self, player: Player) {
        player.send_packet(&UpdateObjectivesPacket {
            objective_name: self.objective.clone(),
            action: UpdateObjectivesAction::CreateScoreboard {
                value: self.text.clone(),
                ty: RenderType::Integer,
                number_format: None,
            },
        });

        let scores = self.scores.read();
        for (entity_name, score) in scores.iter() {
            player.send_packet(&UpdateScorePacket {
                entity_name: entity_name.clone(),
                objective_name: self.objective.clone(),
                value: *score,
                display_name: None,
                number_format: None,
            });
        }
        drop(scores);

        player.send_packet(&DisplayObjectivePacket {
            position: DisplayObjectivePacket::BELOW_NAME,
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
