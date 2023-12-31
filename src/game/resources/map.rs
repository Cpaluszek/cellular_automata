use bevy::{
    prelude::{Color, Entity, IVec2, Resource},
    utils::{HashMap, HashSet},
};

#[derive(Resource)]
pub struct BoardSize {
    pub size: u32,
}

#[derive(Default, Resource)]
pub struct CellColor {
    pub color: Color,
}

#[derive(Clone, Resource)]
pub struct CellMap {
    cells: HashMap<IVec2, Entity>,
}

impl Default for CellMap {
    fn default() -> Self {
        Self {
            cells: Default::default(),
        }
    }
}

impl CellMap {
    /// Retrieves every cell entity matching `coords`.
    /// If some coordinates are not stored in the cell map they will be ignored.
    // pub fn get_cell_entities<'a>(
    //     &'a self,
    //     coords: &'a [C::Coordinates],
    // ) -> impl Iterator<Item = &Entity> + 'a {
    //     coords.iter().filter_map(|c| self.cells.get(c))
    // }

    /// Adds a `Cell` entity to the map at `coordinates`.
    ///
    /// # Note:
    ///
    /// This operation is done automatically when you add a `Cell` component to
    /// an entity.
    ///
    /// # Returns
    ///
    /// If the map did not have this key present, `None` is returned.
    /// If the map did have this key present, the value is updated, and the old
    /// value is returned
    pub fn insert_cell(&mut self, coordinates: IVec2, entity: Entity) -> Option<Entity> {
        self.cells.insert(coordinates, entity)
    }

    /// Removes a cell from the map, returning the `Entity` value if it was
    /// present.
    ///
    /// # Note:
    ///
    /// Use this method to remove cell entities from the map if you remove a
    /// `Cell` component from an `Entity` or *despawn* an `Entity` with a
    /// `Cell` component.
    pub fn remove_cell(&mut self, coordinates: &IVec2) -> Option<Entity> {
        self.cells.remove(coordinates)
    }

    /// Removes a cell entities from the map
    pub fn remove_entities(&mut self, entities: impl Iterator<Item = Entity>) {
        let entities: HashSet<_> = entities.collect();
        if entities.is_empty() {
            return;
        }
        self.cells.retain(|_, entity| !entities.contains(entity));
    }

    // Retrieves a cell entity using its `coordinates`
    pub fn get_cell(&self, coordinates: &IVec2) -> Option<Entity> {
        self.cells.get(coordinates).copied()
    }

    // Clears the entire map
    // pub fn clear(&mut self) {
    //     self.cells.clear();
    // }

    pub fn cell_count(&self) -> usize {
        self.cells.len()
    }
}
