use bevy::prelude::*;

fn main(){
	App::new()
		.add_systems(Startup, setup)
		.add_systems(PostStartup, (
				get_all_loot_enchantments_from_treasure_box,
				get_all_loot_enchantments_from_treasure_box_linq
			).chain()
		)
		.run();
}

#[derive(Component, Debug)]
struct TreasureBox;
#[derive(Component, Debug)]
struct Item;
#[derive(Component, Debug)]
struct Enchantment;

fn setup(mut commands: Commands){
	commands.spawn(SpatialBundle::default()).with_children(|parent|{
		parent.spawn((SpatialBundle::default(), TreasureBox)).with_children(|parent|{
			parent.spawn((SpatialBundle::default(), Item)).with_children(|parent|{
				parent.spawn((SpatialBundle::default(), Enchantment));
			});
		});
	});
}

fn get_all_loot_enchantments_from_treasure_box(
    treasures: Query<(&TreasureBox, &Children)>,
    items: Query<(&Item, &Children)>,
    enchantments: Query<&Enchantment>,
) {
	println!("\nFrom for loops system:");

    for (treasure, treasure_box_children) in treasures.iter() {
        for treasure_box_child in treasure_box_children.iter() {
            if let Ok((item, items_children)) = items.get(*treasure_box_child) {
                for items_child in items_children.iter() {
                    if let Ok(enchantment) = enchantments.get(*items_child) {
                        println!(
                            "	Treasure {:?} Has Item {:?} Has enchantment: {:?}",
                            &treasure, &item, &enchantment
                        );
                    }
                }
            }
        }
    }
}

fn get_all_loot_enchantments_from_treasure_box_linq(
    treasures: Query<(&TreasureBox, &Children)>,
    items: Query<(&Item, &Children)>,
    enchantments: Query<&Enchantment>,
) {
	println!("\nFrom for linq-like system:");

	let enchantments: Vec<&Enchantment> = treasures
		.iter()
		.flat_map(|(_tresure_box, children)| items.iter_many(children))
		.flat_map(|(_item, children)| enchantments.iter_many(children))
		.collect();

    enchantments.iter().for_each(|e| println!("	Enchantment {:?}", *e));
}