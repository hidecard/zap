use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
    path::{Path, PathBuf},
    rc::Rc,
};

use crate::{Function, Value};

pub(crate) type ModuleCacheEntry = (HashMap<String, Value>, HashMap<String, Rc<Function>>);

pub(crate) const DEFAULT_MEMORY_BUDGET_BYTES: u64 = 64 * 1024 * 1024;
pub(crate) const DEFAULT_MEMORY_BUDGET_TASKS: u64 = 1_024;
pub(crate) const DEFAULT_MEMORY_BUDGET_OUTPUT_BYTES: u64 = 8 * 1024 * 1024;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct MemoryBudget {
    max_bytes: u64,
    used_bytes: u64,
    max_tasks: u64,
    admitted_tasks: u64,
    max_output_bytes: u64,
    used_output_bytes: u64,
}

#[allow(dead_code)]
impl MemoryBudget {
    pub(crate) fn new() -> Self {
        Self {
            max_bytes: DEFAULT_MEMORY_BUDGET_BYTES,
            max_tasks: DEFAULT_MEMORY_BUDGET_TASKS,
            max_output_bytes: DEFAULT_MEMORY_BUDGET_OUTPUT_BYTES,
            ..Self::default()
        }
    }

    pub(crate) fn reserve_bytes(&mut self, bytes: u64) -> Result<(), String> {
        let next = self
            .used_bytes
            .checked_add(bytes)
            .ok_or_else(|| "memory budget exceeded: logical byte counter overflow".to_string())?;
        if next > self.max_bytes {
            return Err(format!(
                "memory budget exceeded: requested {bytes} bytes with {}/{} bytes used",
                self.used_bytes, self.max_bytes
            ));
        }
        self.used_bytes = next;
        Ok(())
    }

    pub(crate) fn release_bytes(&mut self, bytes: u64) {
        self.used_bytes = self.used_bytes.saturating_sub(bytes);
    }

    pub(crate) fn admit_task(&mut self) -> Result<(), String> {
        let next = self.admitted_tasks.saturating_add(1);
        if next > self.max_tasks {
            return Err(format!(
                "task budget exceeded: maximum is {} admitted tasks",
                self.max_tasks
            ));
        }
        self.admitted_tasks = next;
        Ok(())
    }

    pub(crate) fn complete_task(&mut self) {
        self.admitted_tasks = self.admitted_tasks.saturating_sub(1);
    }

    pub(crate) fn reserve_output(&mut self, bytes: u64) -> Result<(), String> {
        let next = self
            .used_output_bytes
            .checked_add(bytes)
            .ok_or_else(|| "output budget exceeded: logical byte counter overflow".to_string())?;
        if next > self.max_output_bytes {
            return Err(format!(
                "output budget exceeded: requested {bytes} bytes with {}/{} bytes used",
                self.used_output_bytes, self.max_output_bytes
            ));
        }
        self.used_output_bytes = next;
        Ok(())
    }

    pub(crate) fn usage(&self) -> (u64, u64, u64) {
        (self.used_bytes, self.admitted_tasks, self.used_output_bytes)
    }

    #[cfg(test)]
    pub(crate) fn set_limits(&mut self, max_bytes: u64, max_tasks: u64, max_output_bytes: u64) {
        self.max_bytes = max_bytes;
        self.max_tasks = max_tasks;
        self.max_output_bytes = max_output_bytes;
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) struct ObjectStore {
    live_objects: usize,
    object_allocations: u64,
    object_deallocations: u64,
}

impl ObjectStore {
    pub(crate) fn record_allocation(&mut self) {
        self.live_objects = self.live_objects.saturating_add(1);
        self.object_allocations = self.object_allocations.saturating_add(1);
    }

    pub(crate) fn record_deallocation(&mut self) {
        self.live_objects = self.live_objects.saturating_sub(1);
        self.object_deallocations = self.object_deallocations.saturating_add(1);
    }

    pub(crate) fn stats(&self) -> (usize, u64, u64) {
        (
            self.live_objects,
            self.object_allocations,
            self.object_deallocations,
        )
    }

    pub(crate) fn reset(&mut self) {
        *self = Self::default();
    }
}

/// Per-run mutable state that must not leak between independent executions.
///
/// The first runtime-state migration slice owns module caching, import-cycle
/// tracking, and execution-depth accounting instead of process-global state.
#[derive(Debug, Default)]
pub(crate) struct RuntimeState {
    workspace_root: Option<PathBuf>,
    memory_budget: MemoryBudget,
    object_store: Rc<RefCell<ObjectStore>>,
    module_loading: Vec<PathBuf>,
    module_cache: HashMap<PathBuf, ModuleCacheEntry>,
    execution_depth: Rc<Cell<usize>>,
}

impl RuntimeState {
    pub(crate) fn new() -> Self {
        Self {
            workspace_root: None,
            memory_budget: MemoryBudget::new(),
            object_store: Rc::new(RefCell::new(ObjectStore::default())),
            module_loading: Vec::new(),
            module_cache: HashMap::new(),
            execution_depth: Rc::new(Cell::new(0)),
        }
    }

    pub(crate) fn reset_for_run(&mut self) {
        self.workspace_root = None;
        self.memory_budget = MemoryBudget::new();
        self.object_store.borrow_mut().reset();
        self.module_loading.clear();
        self.module_cache.clear();
        self.execution_depth.set(0);
    }

    pub(crate) fn workspace_root(&self) -> Option<&Path> {
        self.workspace_root.as_deref()
    }

    pub(crate) fn set_workspace_root(&mut self, root: PathBuf) {
        self.workspace_root = Some(root);
    }

    #[cfg(test)]
    pub(crate) fn memory_budget(&self) -> &MemoryBudget {
        &self.memory_budget
    }

    #[allow(dead_code)]
    pub(crate) fn memory_budget_mut(&mut self) -> &mut MemoryBudget {
        &mut self.memory_budget
    }

    pub(crate) fn object_store(&self) -> &Rc<RefCell<ObjectStore>> {
        &self.object_store
    }

    pub(crate) fn module_loading(&self) -> &[PathBuf] {
        &self.module_loading
    }

    pub(crate) fn module_loading_mut(&mut self) -> &mut Vec<PathBuf> {
        &mut self.module_loading
    }

    pub(crate) fn module_cache(&self) -> &HashMap<PathBuf, ModuleCacheEntry> {
        &self.module_cache
    }

    pub(crate) fn module_cache_mut(&mut self) -> &mut HashMap<PathBuf, ModuleCacheEntry> {
        &mut self.module_cache
    }

    #[cfg(test)]
    pub(crate) fn execution_depth(&self) -> usize {
        self.execution_depth.get()
    }

    #[cfg(test)]
    pub(crate) fn increment_execution_depth(&mut self) {
        self.execution_depth.set(self.execution_depth.get() + 1);
    }

    pub(crate) fn execution_depth_handle(&self) -> Rc<Cell<usize>> {
        Rc::clone(&self.execution_depth)
    }
}

/// Explicit execution context passed through the runtime entrypoint.
#[derive(Debug, Default)]
pub(crate) struct ExecutionContext {
    state: RuntimeState,
}

impl ExecutionContext {
    pub(crate) fn new() -> Self {
        Self {
            state: RuntimeState::new(),
        }
    }

    pub(crate) fn reset_for_run(&mut self) {
        self.state.reset_for_run();
    }

    pub(crate) fn state(&self) -> &RuntimeState {
        &self.state
    }

    pub(crate) fn state_mut(&mut self) -> &mut RuntimeState {
        &mut self.state
    }
}

#[cfg(test)]
mod tests {
    use super::ExecutionContext;
    use std::path::Path;

    #[test]
    fn independent_contexts_do_not_share_runtime_state() {
        let mut first = ExecutionContext::new();
        let second = ExecutionContext::new();

        first
            .state_mut()
            .module_loading_mut()
            .push("first.zp".into());
        first.state_mut().increment_execution_depth();

        assert!(second.state().module_loading().is_empty());
        assert!(second.state().workspace_root().is_none());
        assert_eq!(second.state().execution_depth(), 0);
        assert_eq!(first.state().execution_depth(), 1);
    }

    #[test]
    fn memory_budget_and_object_store_are_isolated_and_reset() {
        let mut first = ExecutionContext::new();
        let second = ExecutionContext::new();
        first.state_mut().memory_budget_mut().set_limits(10, 1, 5);
        first
            .state_mut()
            .memory_budget_mut()
            .reserve_bytes(4)
            .expect("reservation should fit");
        assert!(first
            .state_mut()
            .memory_budget_mut()
            .reserve_bytes(7)
            .is_err());
        first.state_mut().memory_budget_mut().release_bytes(2);
        assert_eq!(first.state().memory_budget().usage(), (2, 0, 0));
        first
            .state_mut()
            .memory_budget_mut()
            .admit_task()
            .expect("first task should fit");
        assert!(first.state_mut().memory_budget_mut().admit_task().is_err());
        first.state_mut().memory_budget_mut().complete_task();
        first
            .state_mut()
            .memory_budget_mut()
            .reserve_output(5)
            .expect("output reservation should fit");
        assert_eq!(second.state().memory_budget().usage(), (0, 0, 0));

        let store = first.state().object_store().clone();
        store.borrow_mut().record_allocation();
        assert_eq!(store.borrow().stats(), (1, 1, 0));
        first.reset_for_run();
        assert_eq!(first.state().memory_budget().usage(), (0, 0, 0));
        assert_eq!(store.borrow().stats(), (0, 0, 0));
    }

    #[test]
    fn reset_for_run_clears_module_and_depth_state() {
        let mut context = ExecutionContext::new();
        context
            .state_mut()
            .set_workspace_root(Path::new("/workspace").into());
        context
            .state_mut()
            .module_loading_mut()
            .push("module.zp".into());
        context.state_mut().increment_execution_depth();
        context.reset_for_run();

        assert!(context.state().workspace_root().is_none());
        assert!(context.state().module_loading().is_empty());
        assert!(context.state().module_cache().is_empty());
        assert_eq!(context.state().execution_depth(), 0);
    }
}
