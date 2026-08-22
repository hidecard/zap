use std::fmt;
use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
    path::{Path, PathBuf},
    rc::Rc,
};

use crate::async_runtime::{AsyncRuntime, Cancellable, CancellationToken};
use crate::{Function, Value};

pub(crate) type ModuleCacheEntry = (HashMap<String, Value>, HashMap<String, Rc<Function>>);

pub(crate) const DEFAULT_MEMORY_BUDGET_BYTES: u64 = 64 * 1024 * 1024;
pub(crate) const DEFAULT_MEMORY_BUDGET_TASKS: u64 = 1_024;
pub(crate) const DEFAULT_MEMORY_BUDGET_OUTPUT_BYTES: u64 = 8 * 1024 * 1024;
pub(crate) const LOGICAL_OBJECT_BASE_BYTES: u64 = 64;
pub(crate) const LOGICAL_OBJECT_FIELD_BYTES: u64 = 32;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct MemoryBudget {
    max_bytes: u64,
    used_bytes: u64,
    max_tasks: u64,
    admitted_tasks: u64,
    max_output_bytes: u64,
    used_output_bytes: u64,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct MemoryBudgetStats {
    pub(crate) max_bytes: u64,
    pub(crate) used_bytes: u64,
    pub(crate) max_tasks: u64,
    pub(crate) admitted_tasks: u64,
    pub(crate) max_output_bytes: u64,
    pub(crate) used_output_bytes: u64,
}

#[allow(dead_code)]
impl MemoryBudget {
    pub(crate) fn new() -> Self {
        Self {
            max_bytes: DEFAULT_MEMORY_BUDGET_BYTES,
            max_tasks: DEFAULT_MEMORY_BUDGET_TASKS,
            max_output_bytes: DEFAULT_MEMORY_BUDGET_OUTPUT_BYTES,
            used_bytes: 0,
            admitted_tasks: 0,
            used_output_bytes: 0,
        }
    }

    pub(crate) fn reserve_object(
        &mut self,
        class_name_bytes: usize,
        field_count: usize,
    ) -> Result<(), String> {
        let class_name_bytes = u64::try_from(class_name_bytes)
            .map_err(|_| "memory budget exceeded: object class name is too large".to_string())?;
        let field_count = u64::try_from(field_count)
            .map_err(|_| "memory budget exceeded: object field count is too large".to_string())?;
        let field_bytes = field_count
            .checked_mul(LOGICAL_OBJECT_FIELD_BYTES)
            .ok_or_else(|| "memory budget exceeded: object field charge overflow".to_string())?;
        let charge = LOGICAL_OBJECT_BASE_BYTES
            .checked_add(class_name_bytes)
            .and_then(|value| value.checked_add(field_bytes))
            .ok_or_else(|| "memory budget exceeded: object charge overflow".to_string())?;
        self.reserve_bytes(charge)
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

    pub(crate) fn stats(&self) -> MemoryBudgetStats {
        MemoryBudgetStats {
            max_bytes: self.max_bytes,
            used_bytes: self.used_bytes,
            max_tasks: self.max_tasks,
            admitted_tasks: self.admitted_tasks,
            max_output_bytes: self.max_output_bytes,
            used_output_bytes: self.used_output_bytes,
        }
    }

    #[cfg(test)]
    pub(crate) fn set_limits(&mut self, max_bytes: u64, max_tasks: u64, max_output_bytes: u64) {
        self.max_bytes = max_bytes;
        self.max_tasks = max_tasks;
        self.max_output_bytes = max_output_bytes;
    }
}

impl Default for MemoryBudget {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) struct ObjectStore {
    live_objects: usize,
    object_allocations: u64,
    object_deallocations: u64,
    cleanup_attempts: u64,
    cleanup_successes: u64,
    cleanup_failures: u64,
    validation_runs: u64,
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

    pub(crate) fn record_cleanup_attempt(&mut self) {
        self.cleanup_attempts = self.cleanup_attempts.saturating_add(1);
    }

    pub(crate) fn record_cleanup_success(&mut self) {
        self.cleanup_successes = self.cleanup_successes.saturating_add(1);
    }

    pub(crate) fn record_cleanup_failure(&mut self) {
        self.cleanup_failures = self.cleanup_failures.saturating_add(1);
    }

    pub(crate) fn record_validation(&mut self) {
        self.validation_runs = self.validation_runs.saturating_add(1);
    }

    pub(crate) fn stats(&self) -> (usize, u64, u64) {
        (
            self.live_objects,
            self.object_allocations,
            self.object_deallocations,
        )
    }

    pub(crate) fn lifecycle_stats(&self) -> (u64, u64, u64, u64) {
        (
            self.cleanup_attempts,
            self.cleanup_successes,
            self.cleanup_failures,
            self.validation_runs,
        )
    }
}

pub(crate) type LanguageTaskId = u64;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum LanguageTaskError {
    Cancelled,
    TimedOut,
}

#[derive(Default)]
struct LanguageScheduler {
    runtime: AsyncRuntime,
    outputs: HashMap<LanguageTaskId, Rc<RefCell<Option<Result<Value, LanguageTaskError>>>>>,
    tokens: HashMap<LanguageTaskId, CancellationToken>,
    timed_out: std::collections::HashSet<LanguageTaskId>,
    next_id: LanguageTaskId,
}

impl fmt::Debug for LanguageScheduler {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("LanguageScheduler")
            .field("pending_tasks", &self.runtime.pending_tasks())
            .field("tracked_outputs", &self.outputs.len())
            .field("next_id", &self.next_id)
            .finish()
    }
}

impl LanguageScheduler {
    fn schedule(&mut self, value: Value) -> Result<LanguageTaskId, String> {
        let id = self.next_id.saturating_add(1);
        self.next_id = id;
        let output = Rc::new(RefCell::new(None));
        let task_output = output.clone();
        let token = CancellationToken::new();
        let task_token = token.clone();
        self.runtime
            .spawn_limited(async move {
                let result = Cancellable::new(async move { value }, task_token)
                    .await
                    .map_err(|_| LanguageTaskError::Cancelled);
                *task_output.borrow_mut() = Some(result);
            })
            .map_err(|_| "language task scheduler rejected the task".to_string())?;
        self.outputs.insert(id, output);
        self.tokens.insert(id, token);
        Ok(id)
    }

    fn run_until_idle(&mut self) {
        self.runtime.run_until_idle();
    }

    fn is_ready(&self, id: LanguageTaskId) -> bool {
        self.outputs
            .get(&id)
            .is_some_and(|output| output.borrow().is_some())
    }

    fn cancel(&mut self, id: LanguageTaskId) -> bool {
        if self.is_ready(id) {
            return false;
        }
        let Some(token) = self.tokens.get(&id) else {
            return false;
        };
        token.cancel();
        true
    }

    fn join(
        &mut self,
        id: LanguageTaskId,
        poll_budget: Option<usize>,
    ) -> Result<Value, LanguageTaskError> {
        match poll_budget {
            Some(budget) => {
                self.runtime.run_with_budget(budget);
                if !self.is_ready(id) {
                    self.timed_out.insert(id);
                    if let Some(token) = self.tokens.get(&id) {
                        token.cancel();
                    }
                    self.runtime.run_with_budget(1);
                }
            }
            None => self.run_until_idle(),
        }
        let output = self.outputs.get(&id).cloned();
        let value = output.and_then(|output| output.borrow_mut().take());
        self.outputs.remove(&id);
        self.tokens.remove(&id);
        let timed_out = self.timed_out.remove(&id);
        match value {
            Some(Ok(value)) => Ok(value),
            Some(Err(LanguageTaskError::Cancelled)) if timed_out => {
                Err(LanguageTaskError::TimedOut)
            }
            Some(Err(error)) => Err(error),
            None if timed_out => Err(LanguageTaskError::TimedOut),
            None => Err(LanguageTaskError::Cancelled),
        }
    }
}

/// Per-run mutable state that must not leak between independent executions.
///
/// The first runtime-state migration slice owns module caching, import-cycle
/// tracking, and execution-depth accounting instead of process-global state.
#[derive(Debug)]
pub(crate) struct RuntimeState {
    workspace_root: Option<PathBuf>,
    memory_budget: MemoryBudget,
    object_store: Rc<RefCell<ObjectStore>>,
    module_loading: Vec<PathBuf>,
    module_cache: HashMap<PathBuf, ModuleCacheEntry>,
    execution_depth: Rc<Cell<usize>>,
    language_scheduler: LanguageScheduler,
}

impl Default for RuntimeState {
    fn default() -> Self {
        Self::new()
    }
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
            language_scheduler: LanguageScheduler::default(),
        }
    }

    pub(crate) fn reset_for_run(&mut self) {
        self.workspace_root = None;
        self.memory_budget = MemoryBudget::new();
        // Detach the old store so objects retained by the previous run cannot
        // mutate the counters belonging to the reset run when they are dropped.
        self.object_store = Rc::new(RefCell::new(ObjectStore::default()));
        self.module_loading.clear();
        self.module_cache.clear();
        self.execution_depth.set(0);
        self.language_scheduler = LanguageScheduler::default();
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

    pub(crate) fn memory_budget_stats(&self) -> MemoryBudgetStats {
        self.memory_budget.stats()
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

    pub(crate) fn schedule_language_task(
        &mut self,
        value: Value,
    ) -> Result<LanguageTaskId, String> {
        self.memory_budget.admit_task()?;
        match self.language_scheduler.schedule(value) {
            Ok(id) => Ok(id),
            Err(error) => {
                self.memory_budget.complete_task();
                Err(error)
            }
        }
    }

    #[cfg(test)]
    pub(crate) fn run_language_tasks_until_idle(&mut self) {
        self.language_scheduler.run_until_idle();
    }

    pub(crate) fn language_task_is_ready(&self, id: LanguageTaskId) -> bool {
        self.language_scheduler.is_ready(id)
    }

    pub(crate) fn cancel_language_task(&mut self, id: LanguageTaskId) -> bool {
        self.language_scheduler.cancel(id)
    }

    pub(crate) fn join_language_task(
        &mut self,
        id: LanguageTaskId,
        poll_budget: Option<usize>,
    ) -> Result<Value, LanguageTaskError> {
        let result = self.language_scheduler.join(id, poll_budget);
        self.memory_budget.complete_task();
        result
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
    use crate::Value;
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
        assert_eq!(
            first.state().memory_budget_stats(),
            super::MemoryBudgetStats {
                max_bytes: 10,
                used_bytes: 2,
                max_tasks: 1,
                admitted_tasks: 0,
                max_output_bytes: 5,
                used_output_bytes: 5,
            }
        );
        first.reset_for_run();
        assert_eq!(first.state().memory_budget().usage(), (0, 0, 0));
        assert_eq!(first.state().object_store().borrow().stats(), (0, 0, 0));
        assert_eq!(store.borrow().stats(), (1, 1, 0));
    }

    #[test]
    fn logical_object_charge_is_bounded_and_deterministic() {
        let mut context = ExecutionContext::new();
        context
            .state_mut()
            .memory_budget_mut()
            .set_limits(100, 1, 100);
        context
            .state_mut()
            .memory_budget_mut()
            .reserve_object(4, 1)
            .expect("the deterministic object charge should fit");
        assert_eq!(context.state().memory_budget().usage(), (100, 0, 0));
        assert!(context
            .state_mut()
            .memory_budget_mut()
            .reserve_object(1, 0)
            .is_err());
        assert_eq!(context.state().memory_budget().usage(), (100, 0, 0));
    }

    #[test]
    fn language_scheduler_is_executor_backed_and_reset_safe() {
        let mut context = ExecutionContext::new();
        let id = context
            .state_mut()
            .schedule_language_task(Value::Number(7))
            .expect("language task should be admitted");
        assert!(!context.state().language_task_is_ready(id));
        assert_eq!(context.state().memory_budget().usage(), (0, 1, 0));
        context.state_mut().run_language_tasks_until_idle();
        assert!(context.state().language_task_is_ready(id));
        assert_eq!(
            context.state_mut().join_language_task(id, None),
            Ok(Value::Number(7))
        );
        assert_eq!(context.state().memory_budget().usage(), (0, 0, 0));

        let pending_id = context
            .state_mut()
            .schedule_language_task(Value::Number(9))
            .expect("second language task should be admitted");
        context.reset_for_run();
        assert!(!context.state().language_task_is_ready(pending_id));
        assert_eq!(context.state().memory_budget().usage(), (0, 0, 0));
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
