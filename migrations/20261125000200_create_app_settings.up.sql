CREATE TABLE app_settings (
    id BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
    setting_key VARCHAR(191) NOT NULL,
    value JSON NOT NULL,
    setting_group VARCHAR(100) NOT NULL DEFAULT 'general',
    description TEXT NULL,
    is_public BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6),
    updated_at TIMESTAMP(6) NOT NULL
        DEFAULT CURRENT_TIMESTAMP(6)
        ON UPDATE CURRENT_TIMESTAMP(6),

    PRIMARY KEY (id),
    UNIQUE KEY uq_app_settings_setting_key (setting_key),
    KEY idx_app_settings_setting_group (setting_group),
    KEY idx_app_settings_is_public (is_public)
);