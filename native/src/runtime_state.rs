use std::{
    cell::Cell,
    collections::HashMap,
    path::{Path, PathBuf},
    rc::Rc,
};

use crate::{Function, Value};

pub(crate) type ModuleCacheEntry = (HashMap<String, Value>, HashMap<String, Rc<Function>>);

/// Per-run mutable state that must not leak between independent executions.
///
/// The first runtime-state migration slice owns module caching, import-cycle
/// tracking, and execution-depth accounting instead of process-global state.
#[derive(Debug, Default)]
pub(crate) struct RuntimeState {
    workspace_root: Option<PathBuf>,
    module_loading: Vec<PathBuf>,
    module_cache: HashMap<PathBuf, ModuleCacheEntry>,
    execution_depth: Rc<Cell<usize>>,
}

impl RuntimeState {
    pub(crate) fn new() -> Self {
        Self {
            workspace_root: None,
            module_loading: Vec::new(),
            module_cache: HashMap::new(),
            execution_depth: Rc::new(Cell::new(0)),
        }
    }

    pub(crate) fn reset_for_run(&mut self) {
        self.workspace_root = None;
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
