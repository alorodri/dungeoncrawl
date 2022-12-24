use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(MovingRandomly)]
pub fn random_move(
    ecs: &mut SubWorld,
    #[resource] map: &Map,
    commands: &mut CommandBuffer,
) {
    let mut movers = <(Entity, &Point, &MovingRandomly)>::query();
    movers
        .iter(ecs)
        .for_each(|(entity, pos, _)| {
            let mut rng = RandomNumberGenerator::new();
            let mut possible_destinations: Vec<Point> = Vec::new();
            
            if map.can_enter_tile(*pos + Point::new(-1, 0)) {
                possible_destinations.push(*pos + Point::new(-1, 0));
            }
            if map.can_enter_tile(*pos + Point::new(1, 0)) {
                possible_destinations.push(*pos + Point::new(1, 0));
            }
            if map.can_enter_tile(*pos + Point::new(0, -1)) {
                possible_destinations.push(*pos + Point::new(0, -1));
            }
            if map.can_enter_tile(*pos + Point::new(0, 1)) {
                possible_destinations.push(*pos + Point::new(0, 1));
            }

            if possible_destinations.len() != 0 {
                let destination = possible_destinations[rng.range(0, possible_destinations.len())];
                commands.push((
                    (),
                    WantsToMove {
                        destination,
                        entity: *entity
                    }
                ));
            }
        })
}