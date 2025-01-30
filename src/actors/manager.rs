use crate::actors::message::{
    ActivateTask, BroadcastNotification, CreateActor, ForwardToActor, GetActorCount,
    InteractWithActor, InteractWithUser, QueryActorState, TrackTaskProgress,
};
use crate::actors::user_actor::UserActor;
use actix::prelude::*;
use std::collections::HashMap;
use uuid::Uuid;

pub struct Manager {
    actors: HashMap<String, Addr<UserActor>>, // Map user_id to their UserActor
}

impl Manager {
    pub fn new() -> Self {
        Manager {
            actors: HashMap::new(),
        }
    }
}

impl Actor for Manager {
    type Context = Context<Self>;
}

impl Handler<BroadcastNotification> for Manager {
    type Result = Result<(), String>;

    fn handle(&mut self, msg: BroadcastNotification, _: &mut Context<Self>) -> Self::Result {
        for (actor_id, actor_addr) in &self.actors {
            if msg.recipients.is_empty() || msg.recipients.contains(actor_id) {
                let notification = msg.message.clone();
                let recipient_actor = actor_addr.clone();
                recipient_actor.do_send(InteractWithActor {
                    actor_id: actor_id.clone(),
                    message: notification,
                });
            }
        }
        Ok(())
    }
}

impl Handler<QueryActorState> for Manager {
    type Result = Result<String, String>;

    fn handle(&mut self, msg: QueryActorState, _: &mut Context<Self>) -> Self::Result {
        if let Some(actor) = self.actors.get(&msg.actor_id) {
            Ok(format!("Actor {} is active", msg.actor_id))
        } else {
            Err(format!("Actor {} not found", msg.actor_id))
        }
    }
}

impl Handler<InteractWithActor> for UserActor {
    type Result = Result<(), String>;

    fn handle(&mut self, msg: InteractWithActor, _: &mut Context<Self>) -> Self::Result {
        println!(
            "Actor {} received broadcast message: {}",
            self.id, msg.message
        );
        Ok(())
    }
}
impl Handler<ActivateTask> for Manager {
    type Result = Result<(), String>;

    fn handle(&mut self, msg: ActivateTask, _: &mut Context<Self>) -> Self::Result {
        println!("Activating task: {:?}", msg);
        // Add logic to store or manage tasks
        Ok(())
    }
}
impl Handler<TrackTaskProgress> for Manager {
    type Result = Result<u8, String>;

    fn handle(&mut self, msg: TrackTaskProgress, _: &mut Context<Self>) -> Self::Result {
        println!("Tracking progress for task: {:?}", msg.task_id);
        // Simulate progress as an example
        Ok(50) // Example: Return 50% progress
    }
}

impl Handler<CreateActor> for Manager {
    type Result = Result<Uuid, String>;

    fn handle(&mut self, msg: CreateActor, _: &mut Context<Self>) -> Self::Result {
        let actor_id = Uuid::new_v4();
        if self.actors.contains_key(&actor_id.to_string()) {
            return Err(format!(
                "Actor for user {} already exists with actor_id {}",
                msg.user_id, actor_id
            ));
        }
        let actor = UserActor::new(
            actor_id,
            msg.user_id.clone(),
            msg.name,
            msg.personality,
            msg.picture_url,
            msg.expertise,
            msg.goals,
            msg.knowledge_base,
        )
        .start();

        self.actors.insert(actor_id.to_string(), actor);
        Ok(actor_id)
    }
}

impl Handler<GetActorCount> for Manager {
    type Result = usize;

    fn handle(&mut self, _: GetActorCount, _: &mut Context<Self>) -> Self::Result {
        self.actors.len()
    }
}

impl Handler<ForwardToActor> for Manager {
    type Result = ResponseFuture<Result<String, String>>;

    fn handle(&mut self, msg: ForwardToActor, _: &mut Context<Self>) -> Self::Result {
        println!("Handle Forward To Actor");
        let user_id = msg.user_id.clone();
        let actor_id = msg.actor_id.clone();
        let query = msg.query;

        if let Some(actor) = self.actors.get(&actor_id) {
            let actor_addr = actor.clone();
            Box::pin(async move {
                actor_addr
                    .send(InteractWithUser { user_id, query })
                    .await
                    .unwrap_or_else(|_| Err("Actor failed to respond".to_string()))
            })
        } else {
            Box::pin(async move {
                Err(format!(
                    "No actor found for user {} and actor {}",
                    user_id, actor_id
                ))
            })
        }
    }
}
