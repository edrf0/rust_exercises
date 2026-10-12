use std::collections::HashSet;

/// 1. Check if the user's permission set contains the target permission.
pub fn has_permission(user_perms: &HashSet<String>, permission: &str) -> bool {
    user_perms.contains(permission)
}

/// 2. Return a new HashSet containing only the permissions present in both users' sets.
pub fn find_shared_permissions(user_a: &HashSet<String>, user_b: &HashSet<String>) -> HashSet<String> {
    user_a.intersection(user_b).cloned().collect()
}

/// 3. Return a new HashSet containing required permissions that the user lacks
/// (i.e., elements in `required` that are not in `user_perms`).
pub fn find_missing_permissions(user_perms: &HashSet<String>, required: &HashSet<String>) -> HashSet<String> {
    required.difference(user_perms).cloned().collect()
}

/// 4. Extend the user's permission set with all permissions from an incoming role set.
pub fn merge_role_permissions(user_perms: &mut HashSet<String>, role_perms: &HashSet<String>) {
    user_perms.extend(role_perms.clone());
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rbac_system() {
        let mut alice_perms = HashSet::from([
            "read:posts".to_string(),
            "write:posts".to_string(),
        ]);

        let bob_perms = HashSet::from([
            "read:posts".to_string(),
            "delete:posts".to_string(),
        ]);

        let required = HashSet::from([
            "read:posts".to_string(),
            "write:posts".to_string(),
            "admin:all".to_string(),
        ]);

        assert!(has_permission(&alice_perms, "read:posts"));
        assert!(!has_permission(&alice_perms, "delete:posts"));

        let shared = find_shared_permissions(&alice_perms, &bob_perms);
        assert_eq!(shared.len(), 1);
        assert!(shared.contains("read:posts"));

        let missing = find_missing_permissions(&alice_perms, &required);
        assert_eq!(missing.len(), 1);
        assert!(missing.contains("admin:all"));

        let editor_role = HashSet::from(["publish:posts".to_string()]);
        merge_role_permissions(&mut alice_perms, &editor_role);
        assert!(has_permission(&alice_perms, "publish:posts"));
    }
}