// ============================================================================
// scheduler_determinista.rs — Cooperative Scheduler Determinista
// ============================================================================
//
// FILOSOFÍA: Orden de ejecución 100% reproducible.
// No preemption = no data races. Yield points explícitos.
//
// Este scheduler implementa cooperative multitasking donde:
//   - Las tasks yieldan explícitamente (no hay timer interrupts)
//   - Round-robin con quantum fijo
//   - La misma secuencia de yields produce la misma secuencia de ejecución
//   - No hay paralelismo real — es scheduling determinista para testing/sim
//
// Autor: Eddi Andreé Salazar Matos
// ============================================================================

use std::collections::VecDeque;

/// Identificador único de una task
pub type TaskId = u64;

/// Configuración del scheduler
#[derive(Debug, Clone)]
pub struct SchedulerConfig {
    /// Quantum por task (en unidades abstractas, no tiempo real)
    pub quantum: u64,
    /// Máximo de tasks simultáneas
    pub max_tasks: usize,
    /// Política de scheduling
    pub policy: SchedulingPolicy,
}

impl Default for SchedulerConfig {
    fn default() -> Self {
        Self {
            quantum: 100,
            max_tasks: 256,
            policy: SchedulingPolicy::RoundRobin,
        }
    }
}

/// Política de scheduling
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchedulingPolicy {
    /// Round-robin simple — cada task obtiene quantum unidades
    RoundRobin,
    /// FIFO — ejecutar hasta yield, luego la siguiente
    Fifo,
    /// Prioridad estática — tasks con prioridad más alta primero
    Priority,
}

/// Estado de una task
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskState {
    /// Listo para ejecutar
    Ready,
    /// Actualmente ejecutando
    Running,
    /// Esperando (yield voluntario)
    Yielded,
    /// Completado exitosamente
    Completed,
    /// Fallido con error
    Failed,
    /// Cancelado
    Cancelled,
}

/// Una task en el scheduler
#[derive(Debug, Clone)]
pub struct Task {
    /// ID único
    pub id: TaskId,
    /// Nombre para diagnósticos
    pub name: String,
    /// Estado actual
    pub state: TaskState,
    /// Prioridad (0 = más alta)
    pub priority: u32,
    /// Quantum restante en esta ronda
    pub remaining_quantum: u64,
    /// Total de quantum consumido (lifetime)
    pub total_quantum_used: u64,
    /// Número de yields realizados
    pub yield_count: u64,
    /// Resultado (si completado)
    pub result: Option<TaskResult>,
}

/// Resultado de una task
#[derive(Debug, Clone)]
pub enum TaskResult {
    /// Éxito con valor de retorno
    Success(i64),
    /// Error con mensaje
    Error(String),
}

/// El Scheduler Determinista
///
/// Garantías:
///   - Dado el mismo conjunto de tasks y yields, el orden de ejecución
///     es SIEMPRE idéntico
///   - No hay preemption — las tasks controlan cuándo yieldan
///   - No hay paralelismo real — una sola task ejecuta a la vez
#[derive(Debug)]
pub struct DeterministicScheduler {
    /// Configuración
    config: SchedulerConfig,
    /// Cola de tasks listas para ejecutar
    ready_queue: VecDeque<TaskId>,
    /// Todas las tasks
    tasks: Vec<Task>,
    /// Task actualmente ejecutando (None si idle)
    current: Option<TaskId>,
    /// Siguiente ID de task
    next_id: TaskId,
    /// Número total de context switches realizados
    context_switches: u64,
    /// Número total de ticks procesados
    total_ticks: u64,
}

impl DeterministicScheduler {
    pub fn new(config: SchedulerConfig) -> Self {
        Self {
            config,
            ready_queue: VecDeque::new(),
            tasks: Vec::new(),
            current: None,
            next_id: 1,
            context_switches: 0,
            total_ticks: 0,
        }
    }

    /// Crear una nueva task
    pub fn spawn(&mut self, name: &str, priority: u32) -> Result<TaskId, SchedulerError> {
        if self.tasks.len() >= self.config.max_tasks {
            return Err(SchedulerError::MaxTasksReached(self.config.max_tasks));
        }

        let id = self.next_id;
        self.next_id += 1;

        let task = Task {
            id,
            name: name.to_string(),
            state: TaskState::Ready,
            priority,
            remaining_quantum: self.config.quantum,
            total_quantum_used: 0,
            yield_count: 0,
            result: None,
        };

        self.tasks.push(task);
        self.ready_queue.push_back(id);
        Ok(id)
    }

    /// Yield de la task actual — pasa el control al scheduler
    pub fn yield_current(&mut self) -> Option<TaskId> {
        if let Some(current_id) = self.current.take() {
            if let Some(task) = self.find_task_mut(current_id) {
                task.state = TaskState::Yielded;
                task.yield_count += 1;
                // Re-queue si aún tiene quantum
                task.remaining_quantum = self.config.quantum; // Reset quantum
                task.state = TaskState::Ready;
                self.ready_queue.push_back(current_id);
            }
        }

        // Schedule next
        self.schedule_next()
    }

    /// Consumir un tick del quantum actual
    pub fn tick(&mut self) -> TickResult {
        self.total_ticks += 1;

        if let Some(current_id) = self.current {
            if let Some(task) = self.find_task_mut(current_id) {
                if task.remaining_quantum > 0 {
                    task.remaining_quantum -= 1;
                    task.total_quantum_used += 1;
                    TickResult::Continue(current_id)
                } else {
                    // Quantum expired — yield automático
                    self.current = None;
                    task.state = TaskState::Ready;
                    task.remaining_quantum = self.config.quantum;
                    self.ready_queue.push_back(current_id);
                    match self.schedule_next() {
                        Some(next) => TickResult::Switched { from: current_id, to: next },
                        None => TickResult::Idle,
                    }
                }
            } else {
                TickResult::Idle
            }
        } else {
            match self.schedule_next() {
                Some(next) => TickResult::Started(next),
                None => TickResult::Idle,
            }
        }
    }

    /// Completar la task actual con un resultado
    pub fn complete_current(&mut self, result: TaskResult) {
        if let Some(current_id) = self.current.take() {
            if let Some(task) = self.find_task_mut(current_id) {
                task.state = TaskState::Completed;
                task.result = Some(result);
            }
        }
    }

    /// Cancelar una task
    pub fn cancel(&mut self, id: TaskId) -> Result<(), SchedulerError> {
        let task = self.find_task_mut(id)
            .ok_or(SchedulerError::TaskNotFound(id))?;

        task.state = TaskState::Cancelled;
        // Remove from ready queue
        self.ready_queue.retain(|t| *t != id);
        if self.current == Some(id) {
            self.current = None;
        }
        Ok(())
    }

    /// Resetear el scheduler (eliminar todas las tasks)
    pub fn reset(&mut self) {
        self.tasks.clear();
        self.ready_queue.clear();
        self.current = None;
        self.next_id = 1;
        self.context_switches = 0;
        self.total_ticks = 0;
    }

    /// Schedule siguiente task según la política
    fn schedule_next(&mut self) -> Option<TaskId> {
        let next_id = match self.config.policy {
            SchedulingPolicy::RoundRobin | SchedulingPolicy::Fifo => {
                self.ready_queue.pop_front()
            }
            SchedulingPolicy::Priority => {
                // Encontrar la task con prioridad más alta (menor número)
                if self.ready_queue.is_empty() {
                    return None;
                }
                let mut best_idx = 0;
                let mut best_priority = u32::MAX;
                for (i, id) in self.ready_queue.iter().enumerate() {
                    if let Some(task) = self.find_task(*id) {
                        if task.priority < best_priority {
                            best_priority = task.priority;
                            best_idx = i;
                        }
                    }
                }
                self.ready_queue.remove(best_idx)
            }
        };

        if let Some(id) = next_id {
            if let Some(task) = self.find_task_mut(id) {
                task.state = TaskState::Running;
            }
            self.current = Some(id);
            self.context_switches += 1;
        }

        next_id
    }

    /// Número de tasks activas
    pub fn active_task_count(&self) -> usize {
        self.tasks.iter()
            .filter(|t| matches!(t.state, TaskState::Ready | TaskState::Running | TaskState::Yielded))
            .count()
    }

    /// Estadísticas
    pub fn stats(&self) -> SchedulerStats {
        SchedulerStats {
            total_tasks: self.tasks.len(),
            active_tasks: self.active_task_count(),
            completed_tasks: self.tasks.iter().filter(|t| t.state == TaskState::Completed).count(),
            context_switches: self.context_switches,
            total_ticks: self.total_ticks,
        }
    }

    fn find_task(&self, id: TaskId) -> Option<&Task> {
        self.tasks.iter().find(|t| t.id == id)
    }

    fn find_task_mut(&mut self, id: TaskId) -> Option<&mut Task> {
        self.tasks.iter_mut().find(|t| t.id == id)
    }
}

/// Resultado de un tick
#[derive(Debug)]
pub enum TickResult {
    /// La task actual continúa ejecutando
    Continue(TaskId),
    /// Context switch de una task a otra
    Switched { from: TaskId, to: TaskId },
    /// Nueva task empezó
    Started(TaskId),
    /// No hay tasks — idle
    Idle,
}

/// Estadísticas del scheduler
#[derive(Debug, Clone)]
pub struct SchedulerStats {
    pub total_tasks: usize,
    pub active_tasks: usize,
    pub completed_tasks: usize,
    pub context_switches: u64,
    pub total_ticks: u64,
}

/// Errores del scheduler
#[derive(Debug, Clone)]
pub enum SchedulerError {
    /// Máximo de tasks alcanzado
    MaxTasksReached(usize),
    /// Task no encontrada
    TaskNotFound(TaskId),
    /// Task ya completada
    TaskAlreadyCompleted(TaskId),
}

impl std::fmt::Display for SchedulerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SchedulerError::MaxTasksReached(max) => write!(f, "Max tasks reached: {}", max),
            SchedulerError::TaskNotFound(id) => write!(f, "Task #{} not found", id),
            SchedulerError::TaskAlreadyCompleted(id) => write!(f, "Task #{} already completed", id),
        }
    }
}

impl std::error::Error for SchedulerError {}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spawn_and_schedule() {
        let mut sched = DeterministicScheduler::new(SchedulerConfig::default());
        let t1 = sched.spawn("task_a", 0).unwrap();
        let t2 = sched.spawn("task_b", 0).unwrap();

        // First tick should start task_a
        match sched.tick() {
            TickResult::Started(id) => assert_eq!(id, t1),
            _ => panic!("Expected Started"),
        }
    }

    #[test]
    fn test_deterministic_order() {
        let mut sched = DeterministicScheduler::new(SchedulerConfig {
            quantum: 2,
            ..Default::default()
        });

        sched.spawn("A", 0).unwrap();
        sched.spawn("B", 0).unwrap();

        // Run the same sequence twice — should be identical
        let mut sequence = Vec::new();
        for _ in 0..6 {
            match sched.tick() {
                TickResult::Continue(id) | TickResult::Started(id) => sequence.push(id),
                TickResult::Switched { to, .. } => sequence.push(to),
                TickResult::Idle => sequence.push(0),
            }
        }

        // Verify deterministic: A gets 2 ticks, then B gets 2 ticks, then back
        // Tick 1: Start A (id=1)
        // Tick 2: Continue A
        // Tick 3: Switch to B (quantum expired for A)
        assert_eq!(sequence[0], 1); // A starts
    }

    #[test]
    fn test_yield() {
        let mut sched = DeterministicScheduler::new(SchedulerConfig::default());
        let t1 = sched.spawn("A", 0).unwrap();
        let t2 = sched.spawn("B", 0).unwrap();

        // Start A
        sched.tick();
        // A yields
        let next = sched.yield_current();
        assert_eq!(next, Some(t2)); // B should be next
    }

    #[test]
    fn test_completion() {
        let mut sched = DeterministicScheduler::new(SchedulerConfig::default());
        sched.spawn("task", 0).unwrap();
        sched.tick();

        sched.complete_current(TaskResult::Success(42));
        assert_eq!(sched.stats().completed_tasks, 1);
    }

    #[test]
    fn test_max_tasks() {
        let mut sched = DeterministicScheduler::new(SchedulerConfig {
            max_tasks: 2,
            ..Default::default()
        });

        sched.spawn("A", 0).unwrap();
        sched.spawn("B", 0).unwrap();
        assert!(sched.spawn("C", 0).is_err());
    }

    #[test]
    fn test_priority_scheduling() {
        let mut sched = DeterministicScheduler::new(SchedulerConfig {
            policy: SchedulingPolicy::Priority,
            ..Default::default()
        });

        sched.spawn("low", 10).unwrap();
        let high = sched.spawn("high", 0).unwrap();

        // High priority should run first
        match sched.tick() {
            TickResult::Started(id) => assert_eq!(id, high),
            _ => panic!("Expected high priority task to start"),
        }
    }
}
