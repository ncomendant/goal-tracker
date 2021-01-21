use std::collections::{HashSet, HashMap};

type GoalId = usize;

pub enum AppError {
    GoalIdNotFound(GoalId),
    ChildrenGoalsExist,
}

struct Goal {
    id: GoalId,
    title: String,
    completed: bool,
}

struct Model {
    next_goal_id: usize,
    goals: HashMap<GoalId, Goal>,
    children: HashMap<GoalId, HashSet<GoalId>>, // parent_goal_id, children_goal_ids
}

fn make_goal<'a>(model: &'a mut Model, title: &str) -> &'a Goal {
    let goal_id = model.next_goal_id;
    model.next_goal_id += 1;

    let goal = Goal {
        id: goal_id,
        title: title.to_string(),
        completed: false,
    };

    model.goals.insert(goal_id, goal);
    model.goals.get(&goal_id).expect("failed to get newly-created goal")
}

fn add_requirement(model: &mut Model, parent_goal_id: GoalId, child_goal_id: GoalId) -> Result<(), AppError> {
    if model.goals.get(&parent_goal_id).is_none() {
        Err(AppError::GoalIdNotFound(parent_goal_id))
    } else if model.goals.get(&child_goal_id).is_none() {
        Err(AppError::GoalIdNotFound(child_goal_id))
    } else {
        let mut children = model.children.entry(parent_goal_id).or_insert(HashSet::new());
        children.insert(child_goal_id);
        Ok(())
    }
}

fn remove_requirement(model: &mut Model, parent_goal_id: GoalId, child_goal_id: GoalId) -> bool {
    if let Some(children) = model.children.get_mut(&parent_goal_id) {
        if let Some(index) = children.iter().position(|id| *id == child_goal_id) {
            children.remove(&index);
            return true;
        }
    }
    false
}

fn delete_goal(model: &mut Model, goal_id: &GoalId) -> Result<Goal, AppError> {
    if let Some(children) = model.children.get(goal_id) {
        if !children.is_empty() {
            return Err(AppError::ChildrenGoalsExist);
        }
    }
    model.children.remove(goal_id);
    model.goals.remove(goal_id).ok_or(AppError::GoalIdNotFound(*goal_id))
}