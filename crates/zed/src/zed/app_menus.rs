use collab_ui::collab_panel;
use gpui::{Menu, MenuItem, OsAction};
use settings_ui::keybindings;
use terminal_view::terminal_panel;

pub fn app_menus() -> Vec<Menu> {
    use zed_actions::Quit;

    vec![
        Menu {
            name: "Zed".into(),
            items: vec![
                MenuItem::action("关于 Zed…", zed_actions::About),
                MenuItem::action("检查更新", auto_update::Check),
                MenuItem::separator(),
                MenuItem::submenu(Menu {
                    name: "Settings".into(),
                    items: vec![
                        MenuItem::action("打开设置", super::OpenSettings),
                        MenuItem::action("打开按键绑定", keybindings::OpenKeymapEditor),
                        MenuItem::action("打开默认设置", super::OpenDefaultSettings),
                        MenuItem::action(
                            "Open Default Key Bindings",
                            zed_actions::OpenDefaultKeymap,
                        ),
                        MenuItem::action("Open Project Settings", super::OpenProjectSettings),
                        MenuItem::action(
                            "Select Settings Profile...",
                            zed_actions::settings_profile_selector::Toggle,
                        ),
                        MenuItem::action(
                            "Select Theme...",
                            zed_actions::theme_selector::Toggle::default(),
                        ),
                    ],
                }),
                MenuItem::separator(),
                MenuItem::submenu(Menu {
                    name: "Services".into(),
                    items: vec![],
                }),
                MenuItem::separator(),
                MenuItem::action("扩展", zed_actions::Extensions::default()),
                MenuItem::action("安装 CLI", install_cli::Install),
                MenuItem::separator(),
                #[cfg(target_os = "macos")]
                MenuItem::action("隐藏 Zed", super::Hide),
                #[cfg(target_os = "macos")]
                MenuItem::action("隐藏其他", super::HideOthers),
                #[cfg(target_os = "macos")]
                MenuItem::action("显示全部", super::ShowAll),
                MenuItem::separator(),
                MenuItem::action("Quit Zed", Quit),
            ],
        },
        Menu {
            name: "File".into(),
            items: vec![
                MenuItem::action("新建", workspace::NewFile),
                MenuItem::action("新建窗口", workspace::NewWindow),
                MenuItem::separator(),
                #[cfg(not(target_os = "macos"))]
                MenuItem::action("Open File...", workspace::OpenFiles),
                MenuItem::action(
                    if cfg!(not(target_os = "macos")) {
                        "Open Folder..."
                    } else {
                        "Open…"
                    },
                    workspace::Open,
                ),
                MenuItem::action(
                    "打开最近...",
                    zed_actions::OpenRecent {
                        create_new_window: false,
                    },
                ),
                MenuItem::action(
                    "Open Remote...",
                    zed_actions::OpenRemote {
                        create_new_window: false,
                        from_existing_connection: false,
                    },
                ),
                MenuItem::separator(),
                MenuItem::action("将文件夹添加到项目…", workspace::AddFolderToProject),
                MenuItem::separator(),
                MenuItem::action("保存", workspace::Save { save_intent: None }),
                MenuItem::action("另存为…", workspace::SaveAs),
                MenuItem::action("保存全部", workspace::SaveAll { save_intent: None }),
                MenuItem::separator(),
                MenuItem::action(
                    "关闭编辑器",
                    workspace::CloseActiveItem {
                        save_intent: None,
                        close_pinned: true,
                    },
                ),
                MenuItem::action("关闭窗口", workspace::CloseWindow),
            ],
        },
        Menu {
            name: "Edit".into(),
            items: vec![
                MenuItem::os_action("撤销", editor::actions::Undo, OsAction::Undo),
                MenuItem::os_action("重做", editor::actions::Redo, OsAction::Redo),
                MenuItem::separator(),
                MenuItem::os_action("剪切", editor::actions::Cut, OsAction::Cut),
                MenuItem::os_action("复制", editor::actions::Copy, OsAction::Copy),
                MenuItem::action("Copy and Trim", editor::actions::CopyAndTrim),
                MenuItem::os_action("粘贴", editor::actions::Paste, OsAction::Paste),
                MenuItem::separator(),
                MenuItem::action("查找", search::buffer_search::Deploy::find()),
                MenuItem::action("在项目中查找", workspace::DeploySearch::find()),
                MenuItem::separator(),
                MenuItem::action(
                    "切换行注释",
                    editor::actions::ToggleComments::default(),
                ),
            ],
        },
        Menu {
            name: "Selection".into(),
            items: vec![
                MenuItem::os_action(
                    "全选",
                    editor::actions::SelectAll,
                    OsAction::SelectAll,
                ),
                MenuItem::action("扩展选择", editor::actions::SelectLargerSyntaxNode),
                MenuItem::action("收缩选择", editor::actions::SelectSmallerSyntaxNode),
                MenuItem::separator(),
                MenuItem::action("在上方添加光标", editor::actions::AddSelectionAbove),
                MenuItem::action("在下方添加光标", editor::actions::AddSelectionBelow),
                MenuItem::action(
                    "选择下一个出现",
                    editor::actions::SelectNext {
                        replace_newest: false,
                    },
                ),
                MenuItem::separator(),
                MenuItem::action("向上移动行", editor::actions::MoveLineUp),
                MenuItem::action("向下移动行", editor::actions::MoveLineDown),
                MenuItem::action("复制选择", editor::actions::DuplicateLineDown),
            ],
        },
        Menu {
            name: "View".into(),
            items: vec![
                MenuItem::action(
                    "Zoom In",
                    zed_actions::IncreaseBufferFontSize { persist: false },
                ),
                MenuItem::action(
                    "Zoom Out",
                    zed_actions::DecreaseBufferFontSize { persist: false },
                ),
                MenuItem::action(
                    "Reset Zoom",
                    zed_actions::ResetBufferFontSize { persist: false },
                ),
                MenuItem::separator(),
                MenuItem::action("切换左侧面板", workspace::ToggleLeftDock),
                MenuItem::action("切换右侧面板", workspace::ToggleRightDock),
                MenuItem::action("切换底部面板", workspace::ToggleBottomDock),
                MenuItem::action("关闭所有面板", workspace::CloseAllDocks),
                MenuItem::submenu(Menu {
                    name: "Editor Layout".into(),
                    items: vec![
                        MenuItem::action("向上拆分", workspace::SplitUp),
                        MenuItem::action("向下拆分", workspace::SplitDown),
                        MenuItem::action("向左拆分", workspace::SplitLeft),
                        MenuItem::action("向右拆分", workspace::SplitRight),
                    ],
                }),
                MenuItem::separator(),
                MenuItem::action("项目面板", project_panel::ToggleFocus),
                MenuItem::action("大纲面板", outline_panel::ToggleFocus),
                MenuItem::action("协作面板", collab_panel::ToggleFocus),
                MenuItem::action("终端面板", terminal_panel::ToggleFocus),
                MenuItem::separator(),
                MenuItem::action("诊断", diagnostics::Deploy),
                MenuItem::separator(),
            ],
        },
        Menu {
            name: "Go".into(),
            items: vec![
                MenuItem::action("后退", workspace::GoBack),
                MenuItem::action("前进", workspace::GoForward),
                MenuItem::separator(),
                MenuItem::action("命令面板...", zed_actions::command_palette::Toggle),
                MenuItem::separator(),
                MenuItem::action("跳转到文件...", workspace::ToggleFileFinder::default()),
                // MenuItem::action("在项目中跳转到符号", project_symbols::Toggle),
                MenuItem::action(
                    "Go to Symbol in Editor...",
                    zed_actions::outline::ToggleOutline,
                ),
                MenuItem::action("跳转到行/列...", editor::actions::ToggleGoToLine),
                MenuItem::separator(),
                MenuItem::action("转到定义", editor::actions::GoToDefinition),
                MenuItem::action("Go to Declaration", editor::actions::GoToDeclaration),
                MenuItem::action("转到类型定义", editor::actions::GoToTypeDefinition),
                MenuItem::action("查找所有引用", editor::actions::FindAllReferences),
                MenuItem::separator(),
                MenuItem::action("下一个问题", editor::actions::GoToDiagnostic::default()),
                MenuItem::action(
                    "Previous Problem",
                    editor::actions::GoToPreviousDiagnostic::default(),
                ),
            ],
        },
        Menu {
            name: "Run".into(),
            items: vec![
                MenuItem::action(
                    "Spawn Task",
                    zed_actions::Spawn::ViaModal {
                        reveal_target: None,
                    },
                ),
                MenuItem::action("Start Debugger", debugger_ui::Start),
                MenuItem::separator(),
                MenuItem::action("Edit tasks.json...", crate::zed::OpenProjectTasks),
                MenuItem::action("Edit debug.json...", zed_actions::OpenProjectDebugTasks),
                MenuItem::separator(),
                MenuItem::action("Continue", debugger_ui::Continue),
                MenuItem::action("Step Over", debugger_ui::StepOver),
                MenuItem::action("Step Into", debugger_ui::StepInto),
                MenuItem::action("Step Out", debugger_ui::StepOut),
                MenuItem::separator(),
                MenuItem::action("Toggle Breakpoint", editor::actions::ToggleBreakpoint),
                MenuItem::action("Edit Breakpoint", editor::actions::EditLogBreakpoint),
                MenuItem::action("Clear all Breakpoints", debugger_ui::ClearAllBreakpoints),
            ],
        },
        Menu {
            name: "Window".into(),
            items: vec![
                MenuItem::action("最小化", super::Minimize),
                MenuItem::action("缩放", super::Zoom),
                MenuItem::separator(),
            ],
        },
        Menu {
            name: "Help".into(),
            items: vec![
                MenuItem::action(
                    "View Release Notes",
                    auto_update_ui::ViewReleaseNotesLocally,
                ),
                MenuItem::action("查看遥测数据", zed_actions::OpenTelemetryLog),
                MenuItem::action("查看依赖项许可证", zed_actions::OpenLicenses),
                MenuItem::action("显示欢迎页", workspace::Welcome),
                MenuItem::action("提供反馈...", zed_actions::feedback::GiveFeedback),
                MenuItem::separator(),
                MenuItem::action(
                    "文档",
                    super::OpenBrowser {
                        url: "https://zed.dev/docs".into(),
                    },
                ),
                MenuItem::action(
                    "Zed Twitter",
                    super::OpenBrowser {
                        url: "https://twitter.com/zeddotdev".into(),
                    },
                ),
                MenuItem::action(
                    "加入团队",
                    super::OpenBrowser {
                        url: "https://zed.dev/jobs".into(),
                    },
                ),
            ],
        },
    ]
}
