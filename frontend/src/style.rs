pub const EDITOR_GROUP_ROOT: &str = "flex min-h-0 flex-1 flex-col overflow-hidden rounded-[var(--app-radius)] border border-app-border bg-app-surface";

pub const EDITOR_GROUP_HEADER: &str = "flex shrink-0 items-center justify-between gap-2 border-b border-app-border bg-app-surface-raised px-3 py-2";

pub const EDITOR_GROUP_CONTENT: &str = "flex min-h-0 flex-1 flex-col overflow-hidden p-app-gap";

pub const EDITOR_GROUP_TITLE: &str =
    "text-[12px] font-semibold tracking-[0.06em] text-app-text-muted uppercase";

pub const EDITOR_GROUP_ACTIONS: &str = "ml-auto flex items-center gap-1.5";

pub const CARD_ROOT: &str = "flex h-16 w-[90px] shrink-0 flex-col overflow-hidden rounded-[var(--app-radius)] border border-app-border bg-app-surface-raised px-[10px] py-2 max-[900px]:h-auto max-[900px]:min-h-16 max-[900px]:min-w-[70px] max-[900px]:max-w-24 max-[900px]:w-auto max-[900px]:px-2 max-[900px]:py-1.5";

pub const CARD_SELECTABLE: &str = "cursor-pointer select-none hover:border-app-accent";

pub const CARD_SELECTED: &str =
    "border-app-accent bg-[color-mix(in_srgb,var(--app-accent)_15%,var(--app-surface-raised))]";

pub const CARD_MOBILE_CIRCLE: &str = "max-[900px]:h-[30px] max-[900px]:w-[30px] max-[900px]:min-h-[30px] max-[900px]:min-w-[30px] max-[900px]:max-w-[30px] max-[900px]:items-center max-[900px]:justify-center max-[900px]:rounded-full max-[900px]:border-2 max-[900px]:bg-transparent max-[900px]:p-0";

pub const CARD_MOBILE_CIRCLE_SELECTED: &str = "max-[900px]:bg-[color-mix(in_srgb,var(--app-accent)_18%,var(--app-surface-raised))] max-[900px]:shadow-[0_0_0_2px_color-mix(in_srgb,var(--app-accent)_30%,transparent)]";

pub const CARD_TITLE: &str = "mb-1 overflow-hidden text-ellipsis whitespace-nowrap text-[12px] font-semibold text-app-text max-[900px]:mb-0.5 max-[900px]:text-[11px]";

pub const CARD_CONTENT: &str = "flex flex-col gap-1";

pub const ICON_BUTTON: &str = "inline-flex h-9 w-9 shrink-0 items-center justify-center rounded-[var(--app-radius)] border border-app-border bg-app-surface-raised p-0 text-app-text transition-colors hover:bg-app-border disabled:cursor-not-allowed disabled:opacity-40 disabled:hover:bg-app-surface-raised [&_svg]:pointer-events-none [&_.icon-spinner]:animate-spin [&_.icon-text]:text-base [&_.icon-text]:leading-none [&_.icon-text]:font-bold";

pub const MODAL_OVERLAY: &str =
    "fixed inset-0 z-[1000] flex items-center justify-center bg-black/55 p-6";

pub const MODAL_ROOT: &str = "flex h-[min(720px,92vh)] w-[min(980px,96vw)] flex-col overflow-hidden rounded-[var(--app-radius)] border border-app-border bg-app-surface";

pub const MODAL_HEADER: &str = "flex shrink-0 items-center justify-between gap-app-gap border-b border-app-border bg-app-surface-raised px-3 py-2.5";

pub const MODAL_TITLE_ROW: &str = "flex min-w-0 items-baseline gap-2.5";

pub const MODAL_TITLE: &str = "font-bold";

pub const MODAL_HINT: &str = "whitespace-nowrap text-[12px] font-normal text-app-text-muted";

pub const MODAL_CLOSE_BUTTON: &str = "h-8 w-8 rounded-[var(--app-radius)] border border-app-border bg-app-surface text-app-text text-[18px] leading-none transition-colors hover:bg-app-border";

pub const MODAL_BODY: &str = "flex min-h-0 flex-1 flex-col gap-app-gap overflow-auto p-app-gap";

pub const SIDEBAR_ROOT: &str = "flex min-w-0 w-[var(--app-toggle-width)] shrink-0 flex-row overflow-hidden border-l border-app-border bg-app-surface transition-[width] duration-200 ease-in-out max-[900px]:fixed max-[900px]:right-0 max-[900px]:top-0 max-[900px]:bottom-0 max-[900px]:z-20 max-[900px]:h-dvh max-[900px]:w-[var(--app-toggle-width)] max-[900px]:shadow-[-8px_0_16px_rgba(0,0,0,0.25)]";

pub const SIDEBAR_ROOT_OPEN: &str =
    "w-[calc(var(--app-sidebar-width)+var(--app-toggle-width))] max-[900px]:w-screen";

pub const SIDEBAR_TOGGLE_WRAP: &str = "flex h-full basis-[var(--app-toggle-width)] flex-col border-r border-app-border max-[900px]:absolute max-[900px]:left-0 max-[900px]:top-0 max-[900px]:bottom-0 max-[900px]:z-[2] max-[900px]:bg-app-surface";

pub const SIDEBAR_TOGGLE_BUTTON: &str = "flex flex-1 items-center justify-center bg-transparent px-0 py-2 text-app-text transition-colors hover:bg-app-surface-raised";

pub const SIDEBAR_CONTENT: &str = "flex min-w-0 flex-1 flex-col overflow-auto w-[var(--app-sidebar-width)] p-app-gap max-[900px]:w-screen max-[900px]:pl-[calc(var(--app-gap)+var(--app-toggle-width))]";

pub const BRICK_VIEW_IMAGE: &str = "block";

pub const COLOR_CARD_CLASS: &str = "max-[900px]:border-2";
pub const COLOR_CARD_CONTENT_CLASS: &str = "mt-auto";
pub const COLOR_CARD_SWATCH: &str = "hidden h-5 w-5 rounded-full max-[900px]:block";
pub const COLOR_CARD_SWATCH_ROW: &str = "flex w-full items-stretch gap-1 max-[900px]:hidden";
pub const COLOR_CARD_SWATCH_VALUE: &str = "flex min-w-0 flex-1 items-center justify-center overflow-hidden rounded-[3px] px-1 py-px text-center text-[10px] leading-none max-[900px]:px-[3px] max-[900px]:py-px max-[900px]:text-[10px]";

pub const BRICK_TYPE_CARD_PREVIEW_WRAP: &str = "h-[22px] overflow-hidden max-[900px]:h-[18px]";
pub const BRICK_TYPE_CARD_PREVIEW_IMAGE: &str = "block h-full w-full object-contain object-left";

pub const DRAG_DROP_LIST: &str = "drag-drop-list";
pub const DRAG_DROP_LIST_ITEM: &str = "drag-drop-list__item";
pub const DRAG_DROP_LIST_ITEM_SELECTED: &str = "drag-drop-list__item--selected";

pub const BRICK_CATALOG_GROUPS: &str = "flex flex-col gap-app-gap";
pub const BRICK_CATALOG_GROUP_WRAP: &str = "flex-none";
pub const BRICK_CATALOG_EDITOR_GROUP_CLASS: &str = "flex-none";
pub const BRICK_CATALOG_EDITOR_GROUP_CONTENT: &str = "flex-none overflow-visible";
pub const BRICK_CATALOG_GRID: &str = "flex flex-wrap gap-2.5";
pub const BRICK_CATALOG_CARD: &str = "h-[92px] w-[120px]";
pub const BRICK_CATALOG_PREVIEW_FRAME: &str = "flex h-10 items-center justify-center overflow-hidden rounded-[4px] border border-app-border bg-app-surface";
pub const BRICK_CATALOG_PREVIEW_IMAGE: &str = "block h-full w-full object-contain";
pub const BRICK_CATALOG_INVALID: &str = "text-[11px] text-app-text-muted";

pub const BRICK_PREVIEW_GRID: &str = "grid flex-none max-[900px]:max-h-[45vh]";
pub const BRICK_PREVIEW_IMAGE: &str = "col-start-1 row-start-1 h-full w-full overflow-hidden";
pub const BRICK_PREVIEW_Y_SLIDER: &str =
    "col-start-2 row-start-1 min-h-0 min-w-0 flex-1 [direction:ltr] [writing-mode:vertical-lr]";
pub const BRICK_PREVIEW_X_SLIDER: &str = "col-start-1 row-start-2 min-h-0 min-w-0 flex-1";

pub const TUTORIAL_EMPTY_STATE: &str = "py-6 text-center text-[12px] text-app-text-muted";
pub const TUTORIAL_LIST: &str = "flex min-h-0 flex-1 flex-col overflow-y-auto p-[2px]";
pub const TUTORIAL_LIST_ITEM: &str = "relative mx-[-4px] my-[2%] cursor-pointer rounded-[var(--app-radius)] border-2 border-transparent px-1 py-0 transition-colors select-none hover:border-app-border [&+&]:mt-[-2.5%] [&[draggable='true']]:cursor-grab active:[&[draggable='true']]:cursor-grabbing";
pub const TUTORIAL_LIST_ITEM_SELECTED: &str = "border-app-accent";
pub const TUTORIAL_ROW: &str = "flex items-center gap-1.5";
pub const TUTORIAL_EXPORT_LABEL: &str = "flex items-center justify-center px-1";
pub const TUTORIAL_EXPORT_CHECKBOX: &str = "h-4 w-4 cursor-pointer";
pub const TUTORIAL_MOVE_BUTTONS: &str = "flex flex-col justify-center gap-0.5";
pub const TUTORIAL_MOVE_BUTTON: &str = "block rounded-[var(--app-radius)] border border-app-border bg-app-bg px-1.5 py-0.5 text-[14px] transition-colors hover:border-app-accent disabled:cursor-not-allowed disabled:opacity-40";
pub const TUTORIAL_BRICK_VIEW: &str = "flex-1";

pub const TUTORIAL_SETTINGS_ACTIONS: &str = "flex flex-row flex-wrap gap-1.5";

pub const BRICK_SETTINGS_EDITOR_GROUP_CLASS: &str = "w-full";
pub const BRICK_SETTINGS_EDITOR_GROUP_CONTENT: &str = "min-h-0 flex-1 overflow-y-auto";
pub const CONTENT_GROUP_CLASS: &str = "flex-none overflow-hidden";
pub const CONTENT_GROUP_CONTENT_CLASS: &str = "flex-none overflow-hidden max-[900px]:items-start";
pub const CONTENT_GROUP_SHELL: &str = "relative flex min-w-0 flex-1";
pub const CONTENT_GROUP_TEXTAREA: &str = "min-h-[8lh] w-full flex-1 resize-none rounded-[var(--app-radius)] border border-app-border bg-app-surface-raised px-2.5 py-1.5 text-[14px] text-app-text outline-none focus:border-app-accent max-[900px]:max-h-[6lh] max-[900px]:min-h-[4lh]";
pub const CONTENT_GROUP_CONTEXT_MENU: &str = "fixed z-[200] flex min-w-40 flex-col gap-1 rounded-[var(--app-radius)] border border-app-border bg-app-surface p-1.5 shadow-[0_12px_24px_rgba(0,0,0,0.18)]";
pub const CONTENT_GROUP_CONTEXT_ITEM: &str = "w-full cursor-pointer rounded-[calc(var(--app-radius)-2px)] bg-transparent px-2.5 py-2 text-left text-app-text transition-colors hover:bg-app-surface-raised disabled:cursor-not-allowed disabled:opacity-45 disabled:hover:bg-transparent";
pub const BRICK_SETTINGS_ROOT: &str =
    "flex min-h-0 min-w-0 flex-1 flex-col gap-app-gap overflow-hidden";
pub const BRICK_SETTINGS_ROW: &str =
    "flex min-h-0 flex-1 gap-app-gap overflow-hidden max-[900px]:flex-col";
pub const BRICK_SETTINGS_COLUMN: &str = "flex min-h-0 min-w-0 flex-1 overflow-hidden";

pub const COLOR_VIEW_GRID: &str = "flex flex-wrap gap-1.5 max-[900px]:h-auto max-[900px]:items-center max-[900px]:gap-2 max-[900px]:overflow-visible";
pub const BRICK_TYPE_VIEW_GRID: &str = "flex flex-wrap gap-1.5 max-[900px]:grid max-[900px]:h-auto max-[900px]:grid-cols-2 max-[900px]:auto-rows-[minmax(64px,_auto)] max-[900px]:content-stretch max-[900px]:overflow-visible";

pub const TUTORIAL_PREVIEW_WRAP: &str = "min-h-0 flex-1 overflow-y-auto";
pub const TUTORIAL_PREVIEW_IMAGE: &str = "block h-auto w-full rounded-[var(--app-radius)]";

pub const TUTORIAL_EDITOR_CONTENT: &str = "flex min-h-0 min-w-0 flex-1 flex-col gap-app-gap";
pub const TUTORIAL_EDITOR_EXPORT_BAR: &str = "flex flex-wrap items-center gap-1.5 rounded-[var(--app-radius)] border border-app-border bg-app-surface-raised p-2";
pub const TUTORIAL_EDITOR_EXPORT_STATUS: &str = "mr-1.5 text-[12px] text-app-text-muted";
pub const TUTORIAL_EDITOR_EXPORT_BUTTON: &str = "rounded-[var(--app-radius)] border border-app-border bg-app-surface px-2 py-1 text-app-text transition-colors hover:bg-app-border disabled:cursor-not-allowed disabled:opacity-50";

pub const APP_ROOT: &str = "flex h-screen flex-col overflow-hidden max-[900px]:relative";
pub const APP_TOOLBAR: &str = "relative z-50 flex w-full items-center justify-between gap-2.5 overflow-visible border-b border-app-border bg-app-surface px-app-gap py-2.5 max-[900px]:order-2 max-[900px]:sticky max-[900px]:bottom-0 max-[900px]:z-[60] max-[900px]:justify-center max-[900px]:border-t max-[900px]:border-b-0 max-[900px]:bg-app-surface-raised max-[900px]:pb-[calc(10px+env(safe-area-inset-bottom))]";
pub const APP_TOOLBAR_GROUP: &str = "flex items-center gap-2.5 overflow-visible";
pub const TOOLBAR_MENU: &str = "toolbar-menu";
pub const TOOLBAR_MENU_DROPDOWN: &str = "toolbar-menu__dropdown";
pub const TOOLBAR_MENU_ITEM: &str = "toolbar-menu__item";
pub const TOOLBAR_MENU_ICON: &str = "toolbar-menu__icon";
pub const APP_MAIN: &str = "flex min-h-0 flex-1 overflow-hidden max-[900px]:order-1 max-[900px]:h-[calc(100dvh-56px)] max-[900px]:pb-[56px]";
pub const APP_EDITOR_WRAP: &str = "flex min-w-0 flex-1 flex-col overflow-auto p-app-gap max-[900px]:h-full max-[900px]:flex-auto max-[900px]:pr-[calc(var(--app-gap)+var(--app-toggle-width))]";
