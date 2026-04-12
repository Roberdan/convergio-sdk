//! Route-level RBAC for agent roles.

use crate::jwt::AgentRole;

/// Check if a role is allowed to access the given route path.
pub fn role_can_access(role: &AgentRole, path: &str) -> bool {
    match role {
        AgentRole::Coordinator => true,
        AgentRole::Dashboard => is_dashboard_route(path),
        AgentRole::Executor => is_executor_route(path),
        AgentRole::Kernel => is_kernel_route(path),
        AgentRole::Worker => is_worker_route(path),
    }
}

/// Prefix match that respects path segment boundaries.
/// `/api/build` matches `/api/build` and `/api/build/foo` but NOT `/api/buildx`.
fn path_prefix(path: &str, prefix: &str) -> bool {
    if path == prefix {
        return true;
    }
    path.starts_with(prefix) && path.as_bytes().get(prefix.len()) == Some(&b'/')
}

fn is_executor_route(path: &str) -> bool {
    path.starts_with("/api/plan-db/task/")
        || path.starts_with("/api/plan-db/agent/")
        || path.starts_with("/api/plan-db/checkpoint/")
        || path.starts_with("/api/plan-db/context/")
        || path.starts_with("/api/plan-db/json/")
        || path.starts_with("/api/plan-db/list")
        || path.starts_with("/api/plan-db/execution-tree/")
        || path.starts_with("/api/plan-db/readiness/")
        || path_prefix(path, "/api/plan-db/kb")
        || path_prefix(path, "/api/build")
        || path_prefix(path, "/api/test")
        || path_prefix(path, "/api/health")
        || path.starts_with("/api/tracking/")
        || path.starts_with("/api/workspace/")
        || path.starts_with("/api/memory/")
        || path.starts_with("/api/ipc/")
        || path.starts_with("/api/delegate/")
        || path.starts_with("/api/delegation/")
        || path_prefix(path, "/api/notify")
}

fn is_kernel_route(path: &str) -> bool {
    path.starts_with("/api/kernel/")
        || path_prefix(path, "/api/notify")
        || path_prefix(path, "/api/health")
        || path.starts_with("/api/node/")
        || path_prefix(path, "/api/heartbeat")
        || path.starts_with("/api/voice/")
        || path.starts_with("/api/memory/")
}

fn is_worker_route(path: &str) -> bool {
    path.starts_with("/api/plan-db/task/")
        || path.starts_with("/api/plan-db/list")
        || path.starts_with("/api/plan-db/context/")
        || path.starts_with("/api/delegate/status")
        || path.starts_with("/api/delegation/")
        || path_prefix(path, "/api/health")
        || path_prefix(path, "/api/heartbeat")
        || path.starts_with("/api/ipc/")
        || path.starts_with("/api/tracking/")
}

fn is_dashboard_route(path: &str) -> bool {
    path_prefix(path, "/api/health")
        || path_prefix(path, "/api/overview")
        || path_prefix(path, "/api/agents")
        || path_prefix(path, "/api/plans")
        || path.starts_with("/api/plan-db/")
        || path_prefix(path, "/api/mesh")
        || path.starts_with("/api/tasks/")
        || path_prefix(path, "/api/notifications")
        || path_prefix(path, "/api/projects")
        || path_prefix(path, "/api/events")
        || path_prefix(path, "/api/peers")
        || path.starts_with("/api/ipc/")
        || path.starts_with("/api/kernel/")
        || path.starts_with("/api/node/")
        || path_prefix(path, "/api/memory")
        || path.starts_with("/api/audit/")
        || path.starts_with("/ws/")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coordinator_unrestricted() {
        assert!(role_can_access(&AgentRole::Coordinator, "/api/anything"));
    }

    #[test]
    fn executor_can_access_tasks() {
        assert!(role_can_access(&AgentRole::Executor, "/api/plan-db/task/1"));
        assert!(role_can_access(&AgentRole::Executor, "/api/health"));
    }

    #[test]
    fn worker_restricted() {
        assert!(role_can_access(&AgentRole::Worker, "/api/health"));
        assert!(!role_can_access(&AgentRole::Worker, "/api/plan-db/agent/1"));
    }

    #[test]
    fn dashboard_read_only() {
        assert!(role_can_access(&AgentRole::Dashboard, "/api/plans"));
        assert!(!role_can_access(&AgentRole::Dashboard, "/api/build"));
    }

    #[test]
    fn path_prefix_boundary() {
        // Must NOT match across segment boundaries
        assert!(!role_can_access(&AgentRole::Executor, "/api/buildx"));
        assert!(!role_can_access(&AgentRole::Executor, "/api/testing"));
        // MUST match exact and sub-paths
        assert!(role_can_access(&AgentRole::Executor, "/api/build"));
        assert!(role_can_access(&AgentRole::Executor, "/api/build/123"));
        assert!(role_can_access(&AgentRole::Executor, "/api/test"));
        assert!(role_can_access(&AgentRole::Executor, "/api/test/abc"));
    }
}
