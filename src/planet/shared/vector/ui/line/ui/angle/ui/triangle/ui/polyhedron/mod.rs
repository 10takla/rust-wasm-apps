mod impls;

use std::rc::Rc;

use crate::planet::shared::vector::ui::line::ui::angle::ui::triangle::Triangle;

pub struct Polyhedron(Vec<Rc<Triangle>>);


