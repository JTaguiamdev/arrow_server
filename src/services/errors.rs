#[derive(Debug)]
pub enum AuthServiceError {
    HashingError,
    VerificationError,
}

impl std::error::Error for AuthServiceError {}

impl std::fmt::Display for AuthServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthServiceError::HashingError => write!(f, "Password hashing failed"),
            AuthServiceError::VerificationError => write!(f, "Password verification failed"),
        }
    }
}

#[derive(Debug)]
pub enum RoleError {
    RoleNotFound,
    PermissionDenied,
    RoleAssignmentFailed,
    RoleCreationFailed,
    PermissionAssignmentFailed,
}

impl std::error::Error for RoleError {}

impl std::fmt::Display for RoleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RoleError::RoleNotFound => write!(f, "Role not found"),
            RoleError::PermissionDenied => write!(f, "Permission denied"),
            RoleError::RoleAssignmentFailed => write!(f, "Role assignment failed"),
            RoleError::RoleCreationFailed => write!(f, "Role creation failed"),
            RoleError::PermissionAssignmentFailed => write!(f, "Permission assignment failed"),
        }
    }
}
