use crate::prelude::*;
use legion::world::SubWorld;

#[system]
#[read_component(Health)]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Amulet)]
pub fn end_turn(#[resource] turn_state: &mut TurnState, ecs: &SubWorld) {
	let mut player_hp = <(&Health, &Point)>::query().filter(component::<Player>());
	let mut amulet = <&Point>::query().filter(component::<Amulet>());
	let amulet_pos = amulet
		.iter(ecs)
		.next()
		.unwrap();
	let current_state = turn_state.clone();
	let mut new_state = match turn_state {
		TurnState::AwaitingInput => return,
		TurnState::PlayerTurn => TurnState::MonsterTurn,
		TurnState::MonsterTurn => TurnState::AwaitingInput,
		_ => current_state
	};
	player_hp.iter(ecs).for_each(|(hp, pos)| {
		if hp.current < 1 {
			new_state = TurnState::GameOver;
		}
		if pos == amulet_pos {
			new_state = TurnState::Victory
		}
	});
	*turn_state = new_state;
}
