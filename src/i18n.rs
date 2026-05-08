pub fn t(key: &str, lang: &str) -> &'static str {
    match (key, lang) {
        // Navigation
        ("nav_dashboard", "en") => "Dashboard",
        ("nav_dashboard", "zh") => "控制台",
        ("nav_projects", "en") => "Projects",
        ("nav_projects", "zh") => "项目管理",
        ("nav_users", "en") => "Users",
        ("nav_users", "zh") => "用户管理",
        ("nav_logout", "en") => "Logout",
        ("nav_logout", "zh") => "退出登录",

        // Login
        ("login_title", "en") => "Login to Rust CI",
        ("login_title", "zh") => "登录 Rust CI",
        ("username", "en") => "Username",
        ("username", "zh") => "用户名",
        ("password", "en") => "Password",
        ("password", "zh") => "密码",
        ("login_btn", "en") => "Sign In",
        ("login_btn", "zh") => "登 录",

        // Dashboard
        ("dash_recent_builds", "en") => "Recent Builds",
        ("dash_recent_builds", "zh") => "最近构建",
        ("dash_no_builds", "en") => "No builds found.",
        ("dash_no_builds", "zh") => "暂无构建记录。",
        ("col_id", "en") => "ID",
        ("col_id", "zh") => "ID",
        ("col_project", "en") => "Project",
        ("col_project", "zh") => "项目",
        ("col_status", "en") => "Status",
        ("col_status", "zh") => "状态",
        ("col_started_at", "en") => "Started At",
        ("col_started_at", "zh") => "开始时间",
        ("col_finished_at", "en") => "Finished At",
        ("col_finished_at", "zh") => "结束时间",
        ("col_action", "en") => "Action",
        ("col_action", "zh") => "操作",
        ("btn_view", "en") => "View",
        ("btn_view", "zh") => "查看",

        // Projects
        ("proj_add_title", "en") => "Add New Project",
        ("proj_add_title", "zh") => "添加新项目",
        ("proj_name", "en") => "Project Name",
        ("proj_name", "zh") => "项目名称",
        ("proj_script", "en") => "Shell Script Path",
        ("proj_script", "zh") => "Shell脚本路径",
        ("proj_log_dir", "en") => "Runtime Log Directory",
        ("proj_log_dir", "zh") => "运行日志目录",
        ("btn_add_proj", "en") => "Add Project",
        ("btn_add_proj", "zh") => "添加项目",
        ("proj_list_title", "en") => "Projects",
        ("proj_list_title", "zh") => "项目列表",
        ("col_webhook_key", "en") => "Webhook Key",
        ("col_webhook_key", "zh") => "Webhook Key",
        ("col_webhook_url", "en") => "Webhook URL",
        ("col_webhook_url", "zh") => "触发地址 (Webhook)",
        ("btn_edit", "en") => "Edit",
        ("btn_edit", "zh") => "编辑",
        ("btn_logs", "en") => "Logs",
        ("btn_logs", "zh") => "运行日志",
        ("proj_no_projects", "en") => "No projects found.",
        ("proj_no_projects", "zh") => "暂无项目。",

        // Project Edit
        ("proj_edit_title", "en") => "Edit Project",
        ("proj_edit_title", "zh") => "编辑项目",
        ("btn_save", "en") => "Save Changes",
        ("btn_save", "zh") => "保存更改",
        ("btn_cancel", "en") => "Cancel",
        ("btn_cancel", "zh") => "取消",

        // Runtime Logs Browser
        ("logs_title", "en") => "Runtime Logs",
        ("logs_title", "zh") => "运行日志",
        ("logs_dir", "en") => "Directory",
        ("logs_dir", "zh") => "所在目录",
        ("col_filename", "en") => "Filename",
        ("col_filename", "zh") => "文件名",
        ("col_size", "en") => "Size (Bytes)",
        ("col_size", "zh") => "大小 (字节)",
        ("btn_tailf", "en") => "tail -f -n 5000",
        ("btn_tailf", "zh") => "tail -f -n 5000",
        ("logs_no_files", "en") => "No files found in directory.",
        ("logs_no_files", "zh") => "目录中没有找到日志文件。",
        ("btn_back_proj", "en") => "Back to Projects",
        ("btn_back_proj", "zh") => "返回项目列表",

        // Build / Log Viewer
        ("build_viewer_title", "en") => "Build Log",
        ("build_viewer_title", "zh") => "构建日志",
        ("log_loading", "en") => "Loading logs...",
        ("log_loading", "zh") => "正在加载日志...",
        ("btn_back_dash", "en") => "Back to Dashboard",
        ("btn_back_dash", "zh") => "返回控制台",
        ("btn_back_logs", "en") => "Back to Logs",
        ("btn_back_logs", "zh") => "返回日志列表",

        // Users
        ("users_add_title", "en") => "Add/Edit User",
        ("users_add_title", "zh") => "添加/修改用户",
        ("users_is_admin", "en") => "Is Admin",
        ("users_is_admin", "zh") => "是否管理员",
        ("btn_save_user", "en") => "Save User",
        ("btn_save_user", "zh") => "保存用户",
        ("users_list_title", "en") => "Users",
        ("users_list_title", "zh") => "用户列表",
        ("users_no_users", "en") => "No users found.",
        ("users_no_users", "zh") => "暂无用户。",

        // Messages
        ("login_err", "en") => "Invalid username or password",
        ("login_err", "zh") => "用户名或密码错误",
        ("users_saved", "en") => "User saved successfully.",
        ("users_saved", "zh") => "用户保存成功。",
        ("proj_saved", "en") => "Project saved successfully.",
        ("proj_saved", "zh") => "项目保存成功。",

        // Fallbacks
        (k, _) => {
            tracing::warn!("Missing translation for key: {}", k);
            "UNKNOWN"
        }
    }
}
