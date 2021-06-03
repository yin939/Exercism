use std::cell::RefCell;

/// `InputCellID` is a unique identifier for an input cell.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InputCellID(usize);

/// `ComputeCellID` is a unique identifier for a compute cell.
/// Values of type `InputCellID` and `ComputeCellID` should not be mutually assignable,
/// demonstrated by the following tests:
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input: react::ComputeCellID = r.create_input(111);
/// ```
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input = r.create_input(111);
/// let compute: react::InputCellID = r.create_compute(&[react::CellID::Input(input)], |_| 222).unwrap();
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ComputeCellID(usize);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CallbackID(usize);

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CellID {
    Input(InputCellID),
    Compute(ComputeCellID),
}

struct ComputeCell<T> {
    dependencies: Vec<CellID>,
    compute_func: Box<dyn Fn(&[T]) -> T>,
    value: RefCell<T>,
}

#[derive(Debug, PartialEq)]
pub enum RemoveCallbackError {
    NonexistentCell,
    NonexistentCallback,
}

type CallbackHandle<'a, T> = (CallbackID, ComputeCellID, Box<dyn FnMut(T) + 'a>);
pub struct Reactor<'a, T> {
    input_cells: Vec<T>,
    compute_cells: Vec<ComputeCell<T>>,
    callbacks: Vec<CallbackHandle<'a, T>>,
}

// You are guaranteed that Reactor will only be tested against types that are Copy + PartialEq.
impl<'a, T: Copy + PartialEq> Reactor<'a, T> {
    pub fn new() -> Self {
        Self {
            input_cells: Vec::new(),
            compute_cells: Vec::new(),
            callbacks: Vec::new(),
        }
    }

    // Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, initial: T) -> InputCellID {
        // ID 用数组长度表示
        let id = InputCellID(self.input_cells.len());
        // 在当前位置，存入数据
        self.input_cells.push(initial);
        id
    }

    // Creates a compute cell with the specified dependencies and compute function.
    // The compute function is expected to take in its arguments in the same order as specified in
    // `dependencies`.
    // You do not need to reject compute functions that expect more arguments than there are
    // dependencies (how would you check for this, anyway?).
    //
    // If any dependency doesn't exist, returns an Err with that nonexistent dependency.
    // (If multiple dependencies do not exist, exactly which one is returned is not defined and
    // will not be tested)
    //
    // Notice that there is no way to *remove* a cell.
    // This means that you may assume, without checking, that if the dependencies exist at creation
    // time they will continue to exist as long as the Reactor exists.
    pub fn create_compute<F: 'static + Fn(&[T]) -> T>(
        &mut self,
        dependencies: &[CellID],
        compute_func: F,
    ) -> Result<ComputeCellID, CellID> {
        let id = self.compute_cells.len();

        let initial_value = self.recompute_cell(dependencies, &compute_func)?;
        let cells = dependencies.to_vec();
        self.compute_cells.push(ComputeCell {
            dependencies: cells,
            compute_func: Box::new(compute_func),
            value: RefCell::new(initial_value),
        });
        Ok(ComputeCellID(id))
    }

    // Retrieves the current value of the cell, or None if the cell does not exist.
    //
    // You may wonder whether it is possible to implement `get(&self, id: CellID) -> Option<&Cell>`
    // and have a `value(&self)` method on `Cell`.
    //
    // It turns out this introduces a significant amount of extra complexity to this exercise.
    // We chose not to cover this here, since this exercise is probably enough work as-is.
    pub fn value(&self, id: CellID) -> Option<T> {
        match id {
            CellID::Input(id) => self.input_cells.get(id.0).copied(),
            CellID::Compute(id) => self
                .compute_cells
                .get(id.0)
                .map(|c| *RefCell::borrow(&c.value)),
        }
    }

    // Sets the value of the specified input cell.
    //
    // Returns false if the cell does not exist.
    //
    // Similarly, you may wonder about `get_mut(&mut self, id: CellID) -> Option<&mut Cell>`, with
    // a `set_value(&mut self, new_value: T)` method on `Cell`.
    //
    // As before, that turned out to add too much extra complexity.
    pub fn set_value(&mut self, id: InputCellID, new_value: T) -> bool {
        match self.input_cells.get_mut(id.0) {
            None => false,
            Some(value) => {
                // 重新计算 value
                *value = new_value;
                self.recompute();
                true
            }
        }
    }

    fn recompute(&mut self) {
        for (cell_id, cell) in self.compute_cells.iter().enumerate() {
            let old_value = *cell.value.borrow();
            let new_value = self
                .recompute_cell(cell.dependencies.as_slice(), &cell.compute_func)
                .unwrap();

            if new_value != old_value {
                for (_, compute_id, callback) in &mut self.callbacks {
                    if cell_id == compute_id.0 {
                        (*callback)(new_value);
                    }
                }
                *cell.value.borrow_mut() = new_value;
            }
        }
    }

    fn recompute_cell(
        &self,
        dependencies: &[CellID],
        compute_func: &dyn Fn(&[T]) -> T,
    ) -> Result<T, CellID> {
        let mut params = Vec::new();
        for &dependency in dependencies {
            params.push(self.value(dependency).ok_or(dependency)?);
        }
        Ok(compute_func(&params))
    }

    // Adds a callback to the specified compute cell.
    //
    // Returns the ID of the just-added callback, or None if the cell doesn't exist.
    //
    // Callbacks on input cells will not be tested.
    //
    // The semantics of callbacks (as will be tested):
    // For a single set_value call, each compute cell's callbacks should each be called:
    // * Zero times if the compute cell's value did not change as a result of the set_value call.
    // * Exactly once if the compute cell's value changed as a result of the set_value call.
    //   The value passed to the callback should be the final value of the compute cell after the
    //   set_value call.
    pub fn add_callback<F: FnMut(T) + 'a>(
        &mut self,
        id: ComputeCellID,
        callback: F,
    ) -> Option<CallbackID> {
        self.compute_cells.get(id.0)?;

        let callback_id = CallbackID(
            self.callbacks
                .iter()
                .map(|(id, _, _)| id.0)
                .max()
                .map(|id| id + 1)
                .unwrap_or(0),
        );

        self.callbacks.push((callback_id, id, Box::new(callback)));
        Some(callback_id)
    }

    // Removes the specified callback, using an ID returned from add_callback.
    //
    // Returns an Err if either the cell or callback does not exist.
    //
    // A removed callback should no longer be called.
    pub fn remove_callback(
        &mut self,
        cell: ComputeCellID,
        callback: CallbackID,
    ) -> Result<(), RemoveCallbackError> {
        self.compute_cells
            .get(cell.0)
            .ok_or(RemoveCallbackError::NonexistentCell)?;

        let index = self
            .callbacks
            .iter()
            .enumerate()
            .find(|(_idx, (callback_id, _, _))| *callback_id == callback)
            .map(|(idx, _)| idx)
            .ok_or(RemoveCallbackError::NonexistentCallback)?;

        let _ = self.callbacks.remove(index);

        Ok(())
    }
}
