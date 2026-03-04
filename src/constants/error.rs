pub const ERR001: &str = "ERR001 - DATABASE_POOL_FAILURE";
pub const ERR002: &str = "ERR002 - DATABASE_MIGRATION_FAILURE";
pub const ERR003: &str = "ERR003 - SERVER_FAIL_TO_START";

// ── Database / repository ──────────────────────────────────────────────────
pub const ERR004: &str = "ERR004 - NOT_FOUND";
pub const ERR005: &str = "ERR005 - UNIQUE_VIOLATION";
pub const ERR006: &str = "ERR006 - DB_ERROR";
pub const ERR007: &str = "ERR007 - DB_ERROR_DELETE";
pub const ERR008: &str = "ERR008 - DB_ERROR_INSERT";
pub const ERR009: &str = "ERR009 - DB_ERROR_FETCH";

// ── Authentication ─────────────────────────────────────────────────────────
pub const ERR010: &str = "ERR010 - EMAIL_ALREADY_EXISTS";
pub const ERR011: &str = "ERR011 - PASSWORD_HASH_FAILED";
pub const ERR012: &str = "ERR012 - PASSWORD_VERIFY_FAILED";
pub const ERR013: &str = "ERR013 - INVALID_CREDENTIALS";
pub const ERR014: &str = "ERR014 - INVALID_REFRESH_TOKEN";
pub const ERR015: &str = "ERR015 - INVALID_TOKEN_EXPIRY_FORMAT";
pub const ERR016: &str = "ERR016 - REFRESH_TOKEN_EXPIRED";
pub const ERR017: &str = "ERR017 - TOKEN_CREATE_FAILED";

// ── Token / JWT ────────────────────────────────────────────────────────────
pub const ERR018: &str = "ERR018 - INVALID_TOKEN";
pub const ERR019: &str = "ERR019 - EXPIRED_SIGNATURE";
pub const ERR020: &str = "ERR020 - INVALID_SIGNATURE";
pub const ERR021: &str = "ERR021 - UNAUTHORIZED";
pub const ERR022: &str = "ERR022 - MISSING_OR_INVALID_AUTHORIZATION_HEADER";

// ── Attachment / file ──────────────────────────────────────────────────────
pub const ERR023: &str = "ERR023 - ATTACHMENT_NOT_FOUND";
pub const ERR024: &str = "ERR024 - NO_FILE_PROVIDED";
pub const ERR025: &str = "ERR025 - EMPTY_FILE";
pub const ERR026: &str = "ERR026 - INVALID_FILE_TYPE";
pub const ERR027: &str = "ERR027 - INVALID_FILENAME";
pub const ERR028: &str = "ERR028 - FILE_EXISTS";
pub const ERR029: &str = "ERR029 - FILE_ALREADY_EXISTS";
pub const ERR030: &str = "ERR030 - FILE_UPLOAD_FAILED";
pub const ERR031: &str = "ERR031 - FILE_TOO_LARGE";

// ── Extractors / validation ────────────────────────────────────────────────
pub const ERR032: &str = "ERR032 - INVALID_PATH_PARAM";
pub const ERR033: &str = "ERR033 - INVALID_BODY_REQUEST";
pub const ERR034: &str = "ERR034 - INVALID_VALIDATION";
pub const ERR035: &str = "ERR035 - INVALID_MULTIPART_DATA";
pub const ERR036: &str = "ERR036 - INVALID_MULTIPART_FIELD";
pub const ERR037: &str = "ERR037 - FAILED_TO_READ_FILE";
pub const ERR038: &str = "ERR038 - FAILED_TO_READ_FIELD";
pub const ERR039: &str = "ERR039 - TOO_MANY_FILES";
pub const ERR040: &str = "ERR040 - INVALID_FIELD_SERIALIZATION";
pub const ERR041: &str = "ERR041 - INVALID_FIELD_FORMAT";

// ── Server / environment ───────────────────────────────────────────────────
pub const ERR042: &str = "ERR042 - REQUEST_TIMED_OUT";
pub const ERR043: &str = "ERR043 - UNEXPECTED_ERROR_OCCURRED";
pub const ERR044: &str = "ERR044 - RESOURCE_NOT_FOUND";
pub const ERR045: &str = "ERR045 - UNKNOWN_ENVIRONMENT";
pub const ERR046: &str = "ERR046 - INTERNAL_SERVER_ERROR";
pub const ERR047: &str = "ERR047 - ENV_SECRET_MISSING";
pub const ERR048: &str = "ERR048 - ENV_PORT_INVALID";
pub const ERR049: &str = "ERR049 - ENV_TIMEOUT_INVALID";
pub const ERR050: &str = "ERR050 - ENV_DATABASE_URL_MISSING";
pub const ERR051: &str = "ERR051 - ENV_DIRECTORY_CREATE_FAILED";
