use crate::game_of_life::{Cell, Pos, Universe};

fn setup() -> Universe {
    Universe::new(Some(Pos(2, 2, 2)))
}

#[cfg(test)]
mod check_set_cell {
    use super::*;

    #[test]
    fn ok_result() -> Result<(), String> {
        let mut universe = setup();
        let pos = Pos(1, 1, 1);
        let result = universe.set_cell(pos);
        assert_eq!(universe.get_cells(), vec![Cell::Dead, Cell::Dead,
                                              Cell::Dead, Cell::Dead,
                                              Cell::Dead, Cell::Dead,
                                              Cell::Dead, Cell::Alive]);
        result
    }

    #[test]
    fn error_result() -> Result<(), String> {
        let pos = Pos(1, 1, 1);
        let mut universe = Universe::new(Some(pos));
        let pos = Pos(2, 2, 2);
        let result = universe.set_cell(pos);
        if let Err(_) = result {
            Ok(())
        } else {
            Err("клетка создана внутри поля".to_string())
        }
    }
}

#[test]
fn get_index() {
    let universe = Universe::new(Some(Pos(3, 4, 3)));
    let pos = Pos(2, 3, 2);
    let index = universe.get_index(pos);
    assert_eq!(index, universe.cells.len() - 1);

    let index = universe.get_index(Pos(0, 0, 0));
    assert_eq!(index, 0);
}

#[test]
fn check_live_neighbor_count() {
    let mut universe = Universe::new(Some(Pos(5, 4, 5)));
    let cell1 = Pos(1, 1, 1);
    let check_neighbors = |count, universe: &mut Universe, pos| {
        assert_eq!(count, universe.live_neighbor_count(pos));
    };

    // 0 живых соседей у cell1
    // println!("{universe}");
    check_neighbors(0, &mut universe, cell1);

    // 12 живых соседей у cell1
    let mut set_layer = |z| {
        universe.set_cell(Pos(1, 0, z)).expect("TODO: panic message");
        universe.set_cell(Pos(0, 1, z)).expect("TODO: panic message");
        universe.set_cell(Pos(1, 2, z)).expect("TODO: panic message");
        universe.set_cell(Pos(2, 1, z)).expect("TODO: panic message");
        universe.set_cell(Pos(2, 2, z)).expect("TODO: panic message");
        universe.set_cell(Pos(0, 0, z)).expect("TODO: panic message");
    };
    set_layer(1);
    set_layer(2);
    // println!("{universe}");
    check_neighbors(12, &mut universe, cell1);


    // 4 живых сосдеей у cell2
    let cell2 = Pos(4, 3, 4);
    // println!("{universe}");
    check_neighbors(0, &mut universe, cell2);
}

#[test]
fn tick() {
    let mut universe = Universe::new(Some(Pos(5, 5, 5)));
    // [Pos(2, 1, 2), Pos(2, 2, 2), Pos(2, 3, 2)]
    //     .into_iter().for_each(|x| {
    //     universe.set_cell(x);
    //     // universe.set_spec_cell(x);
    // });
    // println!("{universe}");
    // universe.tick();
    // let live_count: i128 = universe.cells.iter().map(|&x| x as i128).sum();
    // assert_eq!(live_count, 9);
    // println!("{universe}");
    // universe.by_step(3);
    universe.set_cell(Pos(1, 2, 2)).unwrap();
    universe.set_cell(Pos(2, 2, 2)).unwrap();
    universe.set_cell(Pos(3, 2, 2)).unwrap();
    println!("{universe}");
    universe.tick();
    println!("{universe}");
    universe.tick();
    println!("{universe}");
    universe.tick();
    println!("{universe}");
    universe.tick();
    println!("{universe}");

}



// other_tests
#[test]
fn other_tests() {
     Universe::new(Some(Pos(2, 2, 2))).set_spec_cell(Pos(2, 2, 2));

}