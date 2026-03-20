#![allow(dead_code)]
use crate::{context_tracker::ContextTracker,
            models::{Conversation,
                     ConversationScenario,
                     ConversationTurn}};
use agent_models::base_agent::BaseAgent;
use std::collections::HashMap;

pub struct ConversationManager<'a> {
    agent: &'a dyn BaseAgent,
    context_tracker: ContextTracker,
}

impl<'a> ConversationManager<'a> {
    pub fn new(agent: &'a dyn BaseAgent) -> Self {
        Self {
            agent,
            context_tracker: ContextTracker::new(),
        }
    }

    pub fn run_conversation(&self, scenario: &ConversationScenario) -> Conversation {
        let mut conversation = Conversation {
            conversation_id: scenario.conversation_id.clone(),
            scenario: scenario.clone(),
            turns: Vec::new(),
            start_time: chrono::Utc::now(),
            end_time: None,
            overall_success: false,
            context_retention_scores: Vec::new(),
        };

        let mut conversation_context: HashMap<String, serde_json::Value> = HashMap::new();

        for (idx, turn_config) in scenario.turns.iter().enumerate() {
            let turn_id = idx as u32 + 1;
            let mut env = turn_config.initial_environment.clone().unwrap_or_default();
            env.extend(conversation_context.clone());

            let trajectory = self.agent.execute_task(&turn_config.user_input, Some(env));

            conversation_context = self.context_tracker.update_context(&conversation_context, &trajectory);

            let retention = self
                .context_tracker
                .calculate_retention(&turn_config.expected_context_keys, &conversation_context);
            conversation.context_retention_scores.push(retention);

            conversation.turns.push(ConversationTurn {
                turn_id,
                user_input: turn_config.user_input.clone(),
                scenario_id: turn_config.scenario_id.clone(),
                context_from_previous: conversation_context.clone(),
                trajectory: Some(trajectory),
                evaluation_result: None,
            });
        }

        conversation.end_time = Some(chrono::Utc::now());
        conversation.overall_success = self.check_overall_success(&conversation, &scenario.overall_success_criteria);
        conversation
    }

    fn check_overall_success(&self, conversation: &Conversation, criteria: &HashMap<String, serde_json::Value>) -> bool {
        if criteria.is_empty() {
            return conversation.turns.iter().all(|t| t.trajectory.as_ref().is_some_and(|tr| tr.success));
        }

        for (key, expected) in criteria {
            match key.as_str() {
                | "min_success_rate" =>
                    if let Some(min) = expected.as_f64() {
                        let successful = conversation.turns.iter().filter(|t| t.trajectory.as_ref().is_some_and(|tr| tr.success)).count();
                        let rate = if !conversation.turns.is_empty() {
                            successful as f64 / conversation.turns.len() as f64
                        } else {
                            0.0
                        };
                        if rate < min {
                            return false;
                        }
                    },
                | "all_turns_success" =>
                    if expected.as_bool().unwrap_or(false) && !conversation.turns.iter().all(|t| t.trajectory.as_ref().is_some_and(|tr| tr.success)) {
                        return false;
                    },
                | _ => {},
            }
        }
        true
    }
}
