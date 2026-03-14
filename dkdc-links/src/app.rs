//! Desktop app for dkdc-links

use iced::widget::{
    Column, button, center, checkbox, column, container, row, scrollable, text, text_input,
};
use iced::{Element, Length, Size, Theme};
use std::collections::HashSet;

use crate::config::Config;
use crate::storage::Storage;

// -- Colors ------------------------------------------------------------------

mod colors {
    use iced::Color;

    pub const BG_DARK: Color = Color::from_rgb(0.10, 0.10, 0.16);
    pub const BG_INPUT: Color = Color::from_rgb(0.14, 0.14, 0.22);
    pub const BG_HOVER: Color = Color::from_rgb(0.18, 0.18, 0.28);
    pub const BORDER: Color = Color::from_rgb(0.18, 0.18, 0.28);
    pub const BORDER_FOCUS: Color = Color::from_rgb(0.75, 0.30, 1.0);
    pub const PURPLE: Color = Color::from_rgb(0.75, 0.30, 1.0);
    pub const PURPLE_DIM: Color = Color::from_rgb(0.65, 0.25, 0.95);
    pub const CYAN: Color = Color::from_rgb(0.13, 0.83, 0.93);
    pub const TEXT: Color = Color::from_rgb(0.55, 0.55, 0.65);
    pub const TEXT_BRIGHT: Color = Color::from_rgb(0.93, 0.93, 0.87);
    pub const TEXT_DIM: Color = Color::from_rgb(0.40, 0.40, 0.50);
    pub const RED: Color = Color::from_rgb(1.0, 0.45, 0.45);
    pub const RED_BG: Color = Color::from_rgb(0.23, 0.10, 0.17);
    pub const RED_BORDER: Color = Color::from_rgb(0.36, 0.17, 0.17);
    pub const TAB_ACTIVE_BG: Color = Color::from_rgb(0.22, 0.16, 0.32);
}

// -- Types -------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tab {
    All,
    Links,
    Aliases,
    Groups,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SortField {
    Name,
    Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum ItemKind {
    Link,
    Alias,
    Group,
}

#[derive(Debug, Clone)]
struct EditState {
    kind: ItemKind,
    name: String,
    field: &'static str,
    value: String,
}

#[derive(Debug, Clone)]
struct ConfirmState {
    title: String,
    message: String,
    action: ConfirmAction,
}

#[derive(Debug, Clone)]
enum ConfirmAction {
    DeleteSingle(ItemKind, String),
    DeleteBulk(Vec<(ItemKind, String)>),
}

// -- Messages ----------------------------------------------------------------

#[derive(Debug, Clone)]
enum Message {
    TabSelected(Tab),
    SearchChanged(String),
    SortBy(SortField),

    AddLinkName(String),
    AddLinkUrl(String),
    SubmitLink,
    AddAliasName(String),
    AddAliasTarget(String),
    SubmitAlias,
    AddGroupName(String),
    AddGroupEntries(String),
    SubmitGroup,

    ToggleSelect(ItemKind, String),
    ToggleSelectAll,
    ClearSelection,
    DeleteSelected,

    RequestDelete(ItemKind, String),
    StartEdit(ItemKind, String, &'static str, String),
    EditChanged(String),
    CommitEdit,
    #[expect(dead_code)]
    CancelEdit,

    OpenUrl(String),

    ConfirmYes,
    ConfirmNo,

    DismissError,
}

// -- App State ---------------------------------------------------------------

struct Links {
    storage: Box<dyn Storage>,
    config: Config,

    tab: Tab,
    search: String,
    sort: SortField,

    add_link_name: String,
    add_link_url: String,
    add_alias_name: String,
    add_alias_target: String,
    add_group_name: String,
    add_group_entries: String,

    selected: HashSet<(ItemKind, String)>,
    editing: Option<EditState>,
    confirm: Option<ConfirmState>,
    error: Option<String>,
}

impl Links {
    fn new(storage: Box<dyn Storage>) -> (Self, iced::Task<Message>) {
        let config = storage.load().unwrap_or_default();
        (
            Self {
                storage,
                config,
                tab: Tab::All,
                search: String::new(),
                sort: SortField::Name,
                add_link_name: String::new(),
                add_link_url: String::new(),
                add_alias_name: String::new(),
                add_alias_target: String::new(),
                add_group_name: String::new(),
                add_group_entries: String::new(),
                selected: HashSet::new(),
                editing: None,
                confirm: None,
                error: None,
            },
            iced::Task::none(),
        )
    }

    fn save(&self) {
        let _ = self.storage.save(&self.config);
    }

    fn resolve_url<'a>(&'a self, name: &str) -> Option<&'a str> {
        if let Some(target) = self.config.aliases.get(name) {
            self.config.links.get(target).map(String::as_str)
        } else {
            self.config.links.get(name).map(String::as_str)
        }
    }

    fn matches_filter(&self, haystack: &str) -> bool {
        if self.search.is_empty() {
            return true;
        }
        let q = self.search.to_lowercase();
        haystack.to_lowercase().contains(&q)
    }

    fn update(&mut self, message: Message) -> iced::Task<Message> {
        match message {
            Message::TabSelected(tab) => self.tab = tab,
            Message::SearchChanged(s) => self.search = s,
            Message::SortBy(field) => {
                self.sort = if self.sort == field {
                    if field == SortField::Name {
                        SortField::Value
                    } else {
                        SortField::Name
                    }
                } else {
                    field
                };
            }

            Message::AddLinkName(s) => self.add_link_name = s,
            Message::AddLinkUrl(s) => self.add_link_url = s,
            Message::SubmitLink => {
                let name = self.add_link_name.trim().to_string();
                let url = self.add_link_url.trim().to_string();
                if !name.is_empty() && !url.is_empty() {
                    self.config.links.insert(name, url);
                    self.save();
                    self.add_link_name.clear();
                    self.add_link_url.clear();
                }
            }
            Message::AddAliasName(s) => self.add_alias_name = s,
            Message::AddAliasTarget(s) => self.add_alias_target = s,
            Message::SubmitAlias => {
                let alias = self.add_alias_name.trim().to_string();
                let target = self.add_alias_target.trim().to_string();
                if !alias.is_empty() && !target.is_empty() {
                    if !self.config.links.contains_key(&target) {
                        self.error =
                            Some(format!("alias target '{target}' does not exist in links"));
                    } else {
                        self.config.aliases.insert(alias, target);
                        self.save();
                        self.add_alias_name.clear();
                        self.add_alias_target.clear();
                        self.error = None;
                    }
                }
            }
            Message::AddGroupName(s) => self.add_group_name = s,
            Message::AddGroupEntries(s) => self.add_group_entries = s,
            Message::SubmitGroup => {
                let name = self.add_group_name.trim().to_string();
                let raw = self.add_group_entries.trim().to_string();
                if !name.is_empty() && !raw.is_empty() {
                    let entries: Vec<String> = raw
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty())
                        .collect();
                    let missing: Vec<&str> = entries
                        .iter()
                        .filter(|e| {
                            !self.config.links.contains_key(e.as_str())
                                && !self.config.aliases.contains_key(e.as_str())
                        })
                        .map(String::as_str)
                        .collect();
                    if !missing.is_empty() {
                        self.error =
                            Some(format!("group entries not found: {}", missing.join(", ")));
                    } else {
                        self.config.groups.insert(name, entries);
                        self.save();
                        self.add_group_name.clear();
                        self.add_group_entries.clear();
                        self.error = None;
                    }
                }
            }

            Message::ToggleSelect(kind, name) => {
                let key = (kind, name);
                if self.selected.contains(&key) {
                    self.selected.remove(&key);
                } else {
                    self.selected.insert(key);
                }
            }
            Message::ToggleSelectAll => {
                let visible = self.visible_items();
                let all_selected = visible.iter().all(|item| self.selected.contains(item));
                if all_selected {
                    self.selected.clear();
                } else {
                    for item in visible {
                        self.selected.insert(item);
                    }
                }
            }
            Message::ClearSelection => {
                self.selected.clear();
            }
            Message::DeleteSelected => {
                let items: Vec<(ItemKind, String)> = self.selected.iter().cloned().collect();
                if !items.is_empty() {
                    let labels: Vec<String> = items
                        .iter()
                        .map(|(k, n)| {
                            let kind_str = match k {
                                ItemKind::Link => "link",
                                ItemKind::Alias => "alias",
                                ItemKind::Group => "group",
                            };
                            format!("{kind_str} \"{n}\"")
                        })
                        .collect();
                    self.confirm = Some(ConfirmState {
                        title: format!(
                            "delete {} item{}",
                            items.len(),
                            if items.len() > 1 { "s" } else { "" }
                        ),
                        message: format!(
                            "are you sure you want to delete: {}? this cannot be undone.",
                            labels.join(", ")
                        ),
                        action: ConfirmAction::DeleteBulk(items),
                    });
                }
            }

            Message::RequestDelete(kind, name) => {
                let kind_str = match kind {
                    ItemKind::Link => "link",
                    ItemKind::Alias => "alias",
                    ItemKind::Group => "group",
                };
                self.confirm = Some(ConfirmState {
                    title: format!("delete {kind_str}"),
                    message: format!(
                        "are you sure you want to delete {kind_str} \"{name}\"? this cannot be undone."
                    ),
                    action: ConfirmAction::DeleteSingle(kind, name),
                });
            }
            Message::StartEdit(kind, name, field, value) => {
                self.editing = Some(EditState {
                    kind,
                    name,
                    field,
                    value,
                });
            }
            Message::EditChanged(s) => {
                if let Some(ref mut edit) = self.editing {
                    edit.value = s;
                }
            }
            Message::CommitEdit => {
                if let Some(edit) = self.editing.take() {
                    let val = edit.value.trim().to_string();
                    if !val.is_empty() {
                        self.apply_edit(edit.kind, &edit.name, edit.field, &val);
                    }
                }
            }
            Message::CancelEdit => {
                self.editing = None;
            }

            Message::OpenUrl(url) => {
                let _ = open::that(&url);
            }

            Message::ConfirmYes => {
                if let Some(confirm) = self.confirm.take() {
                    match confirm.action {
                        ConfirmAction::DeleteSingle(kind, name) => {
                            self.delete_item(kind, &name);
                        }
                        ConfirmAction::DeleteBulk(items) => {
                            for (kind, name) in items {
                                self.delete_item(kind, &name);
                            }
                            self.selected.clear();
                        }
                    }
                    self.save();
                }
            }
            Message::ConfirmNo => {
                self.confirm = None;
            }

            Message::DismissError => {
                self.error = None;
            }
        }
        iced::Task::none()
    }

    fn delete_item(&mut self, kind: ItemKind, name: &str) {
        match kind {
            ItemKind::Link => {
                self.config.links.remove(name);
            }
            ItemKind::Alias => {
                self.config.aliases.remove(name);
            }
            ItemKind::Group => {
                self.config.groups.remove(name);
            }
        }
    }

    fn apply_edit(&mut self, kind: ItemKind, name: &str, field: &str, value: &str) {
        match (kind, field) {
            (ItemKind::Link, "name") => {
                if value != name
                    && let Err(e) = self.config.rename_link(name, value)
                {
                    self.error = Some(e.to_string());
                    return;
                }
            }
            (ItemKind::Link, "url") => {
                if let Some(url) = self.config.links.get_mut(name) {
                    *url = value.to_string();
                }
            }
            (ItemKind::Alias, "name") => {
                if value != name
                    && let Err(e) = self.config.rename_alias(name, value)
                {
                    self.error = Some(e.to_string());
                    return;
                }
            }
            (ItemKind::Alias, "target") => {
                if !self.config.links.contains_key(value) {
                    self.error = Some(format!("alias target '{value}' does not exist in links"));
                    return;
                }
                if let Some(target) = self.config.aliases.get_mut(name) {
                    *target = value.to_string();
                }
            }
            (ItemKind::Group, "name") => {
                if value != name
                    && let Some(entries) = self.config.groups.remove(name)
                {
                    self.config.groups.insert(value.to_string(), entries);
                }
            }
            (ItemKind::Group, "entries") => {
                let entries: Vec<String> = value
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
                let missing: Vec<&str> = entries
                    .iter()
                    .filter(|e| {
                        !self.config.links.contains_key(e.as_str())
                            && !self.config.aliases.contains_key(e.as_str())
                    })
                    .map(String::as_str)
                    .collect();
                if !missing.is_empty() {
                    self.error = Some(format!("group entries not found: {}", missing.join(", ")));
                    return;
                }
                if let Some(existing) = self.config.groups.get_mut(name) {
                    *existing = entries;
                }
            }
            _ => {}
        }
        self.error = None;
        self.save();
    }

    fn visible_items(&self) -> Vec<(ItemKind, String)> {
        let mut items = Vec::new();
        if self.tab == Tab::All || self.tab == Tab::Links {
            for (name, url) in &self.config.links {
                if self.matches_filter(&format!("{name} {url}")) {
                    items.push((ItemKind::Link, name.clone()));
                }
            }
        }
        if self.tab == Tab::All || self.tab == Tab::Aliases {
            for (name, target) in &self.config.aliases {
                if self.matches_filter(&format!("{name} {target}")) {
                    items.push((ItemKind::Alias, name.clone()));
                }
            }
        }
        if self.tab == Tab::All || self.tab == Tab::Groups {
            for (name, entries) in &self.config.groups {
                let filter_str = format!("{name} {}", entries.join(", "));
                if self.matches_filter(&filter_str) {
                    items.push((ItemKind::Group, name.clone()));
                }
            }
        }
        items
    }

    // -- View ----------------------------------------------------------------

    fn view(&self) -> Element<'_, Message> {
        let mut content = column![].spacing(16).width(Length::Fill);

        // Title
        content = content.push(
            column![
                text("Bookmarks").size(24).color(colors::TEXT),
                iced::widget::rich_text::<(), Message, _, _>([
                    iced::widget::span("dkdc-links: bookmarks in your ")
                        .size(13)
                        .color(colors::TEXT_DIM),
                    iced::widget::span("terminal")
                        .size(13)
                        .color(colors::TEXT_DIM)
                        .strikethrough(true),
                    iced::widget::span(" app").size(13).color(colors::TEXT_DIM),
                ]),
            ]
            .spacing(4),
        );

        // Toolbar
        content = content.push(self.view_toolbar());

        // Error banner
        if let Some(ref msg) = self.error {
            content = content.push(self.view_error(msg));
        }

        // Bulk bar
        if !self.selected.is_empty() {
            content = content.push(self.view_bulk_bar());
        }

        // Add forms
        content = content.push(self.view_add_forms());

        // Sections
        let mut sections = column![].spacing(20);
        if self.tab == Tab::All || self.tab == Tab::Links {
            sections = sections.push(self.view_links_section());
        }
        if self.tab == Tab::All || self.tab == Tab::Aliases {
            sections = sections.push(self.view_aliases_section());
        }
        if self.tab == Tab::All || self.tab == Tab::Groups {
            sections = sections.push(self.view_groups_section());
        }
        content = content.push(sections);

        let body = scrollable(
            container(content)
                .width(640)
                .padding(32)
                .center_x(Length::Fill),
        )
        .width(Length::Fill)
        .height(Length::Fill);

        if let Some(ref confirm) = self.confirm {
            let overlay = self.view_confirm_modal(confirm);
            iced::widget::stack![
                container(body)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .style(|_| container::Style {
                        background: Some(iced::Background::Color(colors::BG_DARK)),
                        ..Default::default()
                    }),
                overlay,
            ]
            .into()
        } else {
            container(body)
                .width(Length::Fill)
                .height(Length::Fill)
                .style(|_| container::Style {
                    background: Some(iced::Background::Color(colors::BG_DARK)),
                    ..Default::default()
                })
                .into()
        }
    }

    fn view_toolbar(&self) -> Element<'_, Message> {
        let search = text_input("filter...", &self.search)
            .on_input(Message::SearchChanged)
            .size(13)
            .width(200)
            .style(|_, status| input_style(status));

        let tab_btn = |label: &str, count: usize, tab: Tab| -> Element<'_, Message> {
            let is_active = self.tab == tab;
            let label_str = format!("{label} {count}");
            button(text(label_str).size(12).color(if is_active {
                colors::PURPLE
            } else {
                colors::TEXT
            }))
            .on_press(Message::TabSelected(tab))
            .padding([4, 10])
            .style(move |_, status| tab_button_style(is_active, status))
            .into()
        };

        let total = self.config.links.len() + self.config.aliases.len() + self.config.groups.len();

        let tabs = row![
            tab_btn("all", total, Tab::All),
            tab_btn("links", self.config.links.len(), Tab::Links),
            tab_btn("aliases", self.config.aliases.len(), Tab::Aliases),
            tab_btn("groups", self.config.groups.len(), Tab::Groups),
        ]
        .spacing(4);

        row![search, iced::widget::Space::new().width(Length::Fill), tabs]
            .spacing(8)
            .align_y(iced::Alignment::Center)
            .into()
    }

    fn view_error(&self, msg: &str) -> Element<'_, Message> {
        button(text(format!("{msg}  x")).size(13).color(colors::RED))
            .on_press(Message::DismissError)
            .padding([8, 12])
            .style(|_, _| button::Style {
                background: Some(iced::Background::Color(colors::RED_BG)),
                border: iced::Border {
                    color: colors::RED_BORDER,
                    width: 1.0,
                    radius: 6.0.into(),
                },
                text_color: colors::RED,
                ..Default::default()
            })
            .width(Length::Fill)
            .into()
    }

    fn view_bulk_bar(&self) -> Element<'_, Message> {
        let count_text = text(format!("{} selected", self.selected.len()))
            .size(13)
            .color(colors::PURPLE);

        let delete_btn = button(text("delete selected").size(12).color(colors::RED))
            .on_press(Message::DeleteSelected)
            .padding([4, 8])
            .style(|_, _| danger_button_style());

        let clear_btn = button(text("clear").size(12).color(colors::TEXT))
            .on_press(Message::ClearSelection)
            .padding([4, 8])
            .style(|_, _| default_button_style());

        container(
            row![count_text, delete_btn, clear_btn]
                .spacing(8)
                .align_y(iced::Alignment::Center),
        )
        .padding([8, 12])
        .style(|_| container::Style {
            background: Some(iced::Background::Color(colors::BG_INPUT)),
            border: iced::Border {
                color: colors::BORDER,
                width: 1.0,
                radius: 6.0.into(),
            },
            ..Default::default()
        })
        .width(Length::Fill)
        .into()
    }

    fn view_add_forms(&self) -> Element<'_, Message> {
        let link_form = row![
            text_input("link name", &self.add_link_name)
                .on_input(Message::AddLinkName)
                .on_submit(Message::SubmitLink)
                .size(13)
                .width(Length::FillPortion(2))
                .style(|_, status| input_style(status)),
            text_input("https://...", &self.add_link_url)
                .on_input(Message::AddLinkUrl)
                .on_submit(Message::SubmitLink)
                .size(13)
                .width(Length::FillPortion(3))
                .style(|_, status| input_style(status)),
            button(text("+ link").size(12).color(colors::PURPLE))
                .on_press(Message::SubmitLink)
                .padding([5, 8])
                .width(72)
                .style(|_, _| add_button_style()),
        ]
        .spacing(6)
        .align_y(iced::Alignment::Center);

        let alias_form = row![
            text_input("alias", &self.add_alias_name)
                .on_input(Message::AddAliasName)
                .on_submit(Message::SubmitAlias)
                .size(13)
                .width(Length::FillPortion(2))
                .style(|_, status| input_style(status)),
            text_input("link name", &self.add_alias_target)
                .on_input(Message::AddAliasTarget)
                .on_submit(Message::SubmitAlias)
                .size(13)
                .width(Length::FillPortion(3))
                .style(|_, status| input_style(status)),
            button(text("+ alias").size(12).color(colors::PURPLE))
                .on_press(Message::SubmitAlias)
                .padding([5, 8])
                .width(72)
                .style(|_, _| add_button_style()),
        ]
        .spacing(6)
        .align_y(iced::Alignment::Center);

        let group_form = row![
            text_input("group name", &self.add_group_name)
                .on_input(Message::AddGroupName)
                .on_submit(Message::SubmitGroup)
                .size(13)
                .width(Length::FillPortion(2))
                .style(|_, status| input_style(status)),
            text_input("link1, alias2, ...", &self.add_group_entries)
                .on_input(Message::AddGroupEntries)
                .on_submit(Message::SubmitGroup)
                .size(13)
                .width(Length::FillPortion(3))
                .style(|_, status| input_style(status)),
            button(text("+ group").size(12).color(colors::PURPLE))
                .on_press(Message::SubmitGroup)
                .padding([5, 8])
                .width(72)
                .style(|_, _| add_button_style()),
        ]
        .spacing(6)
        .align_y(iced::Alignment::Center);

        column![link_form, alias_form, group_form].spacing(6).into()
    }

    fn view_links_section(&self) -> Element<'_, Message> {
        let mut links: Vec<_> = self.config.links.iter().collect();
        match self.sort {
            SortField::Name => links.sort_by_key(|(k, _)| k.as_str()),
            SortField::Value => links.sort_by_key(|(_, v)| v.as_str()),
        }

        let header = self.view_table_header("name", "url");

        let mut rows = Column::new().spacing(0);
        let mut visible_count = 0;
        for (name, url) in &links {
            if !self.matches_filter(&format!("{name} {url}")) {
                continue;
            }
            visible_count += 1;
            rows = rows.push(self.view_link_row(name, url));
            rows = rows.push(iced::widget::rule::horizontal(1).style(|_| rule_style()));
        }

        let body: Element<'_, Message> = if visible_count == 0 {
            text("no links yet").size(13).color(colors::TEXT_DIM).into()
        } else {
            column![
                header,
                iced::widget::rule::horizontal(1).style(|_| rule_style()),
                rows
            ]
            .into()
        };

        column![text("links").size(16).color(colors::TEXT), body]
            .spacing(8)
            .into()
    }

    fn view_aliases_section(&self) -> Element<'_, Message> {
        let mut aliases: Vec<_> = self.config.aliases.iter().collect();
        match self.sort {
            SortField::Name => aliases.sort_by_key(|(k, _)| k.as_str()),
            SortField::Value => aliases.sort_by_key(|(_, v)| v.as_str()),
        }

        let header = self.view_table_header("alias", "target");

        let mut rows = Column::new().spacing(0);
        let mut visible_count = 0;
        for (alias, target) in &aliases {
            if !self.matches_filter(&format!("{alias} {target}")) {
                continue;
            }
            visible_count += 1;
            rows = rows.push(self.view_alias_row(alias, target));
            rows = rows.push(iced::widget::rule::horizontal(1).style(|_| rule_style()));
        }

        let body: Element<'_, Message> = if visible_count == 0 {
            text("no aliases yet")
                .size(13)
                .color(colors::TEXT_DIM)
                .into()
        } else {
            column![
                header,
                iced::widget::rule::horizontal(1).style(|_| rule_style()),
                rows
            ]
            .into()
        };

        column![text("aliases").size(16).color(colors::TEXT), body]
            .spacing(8)
            .into()
    }

    fn view_groups_section(&self) -> Element<'_, Message> {
        let mut groups: Vec<_> = self.config.groups.iter().collect();
        groups.sort_by_key(|(k, _)| k.as_str());

        let header = self.view_table_header("group", "entries");

        let mut rows = Column::new().spacing(0);
        let mut visible_count = 0;
        for (name, entries) in &groups {
            let filter_str = format!("{name} {}", entries.join(", "));
            if !self.matches_filter(&filter_str) {
                continue;
            }
            visible_count += 1;
            rows = rows.push(self.view_group_row(name, entries));
            rows = rows.push(iced::widget::rule::horizontal(1).style(|_| rule_style()));
        }

        let body: Element<'_, Message> = if visible_count == 0 {
            text("no groups yet")
                .size(13)
                .color(colors::TEXT_DIM)
                .into()
        } else {
            column![
                header,
                iced::widget::rule::horizontal(1).style(|_| rule_style()),
                rows
            ]
            .into()
        };

        column![text("groups").size(16).color(colors::TEXT), body]
            .spacing(8)
            .into()
    }

    fn view_table_header<'a>(&self, col1: &str, col2: &str) -> Element<'a, Message> {
        let name_active = self.sort == SortField::Name;
        let value_active = self.sort == SortField::Value;

        let select_all = checkbox(false)
            .on_toggle(|_| Message::ToggleSelectAll)
            .size(14)
            .style(|_, _| checkbox_style());

        let name_header = button(text(col1.to_uppercase()).size(11).color(if name_active {
            colors::PURPLE
        } else {
            colors::TEXT_DIM
        }))
        .on_press(Message::SortBy(SortField::Name))
        .padding(0)
        .style(|_, _| button::Style::default());

        let value_header = button(text(col2.to_uppercase()).size(11).color(if value_active {
            colors::PURPLE
        } else {
            colors::TEXT_DIM
        }))
        .on_press(Message::SortBy(SortField::Value))
        .padding(0)
        .style(|_, _| button::Style::default());

        row![
            container(select_all).width(28),
            container(name_header).width(130),
            container(value_header).width(Length::Fill),
            container(text("").size(11)).width(60),
        ]
        .spacing(8)
        .padding([6, 8])
        .align_y(iced::Alignment::Center)
        .into()
    }

    fn view_link_row(&self, name: &str, url: &str) -> Element<'_, Message> {
        let is_selected = self.selected.contains(&(ItemKind::Link, name.to_string()));

        let cb = checkbox(is_selected)
            .on_toggle({
                let name = name.to_string();
                move |_| Message::ToggleSelect(ItemKind::Link, name.clone())
            })
            .size(14)
            .style(|_, _| checkbox_style());

        let name_cell = self.view_editable_cell(
            ItemKind::Link,
            name,
            "name",
            name.to_string(),
            colors::PURPLE,
            Some(url.to_string()),
        );

        let url_cell = self.view_editable_cell(
            ItemKind::Link,
            name,
            "url",
            url.to_string(),
            colors::CYAN,
            None,
        );

        let delete_btn = button(text("delete").size(12).color(colors::RED))
            .on_press(Message::RequestDelete(ItemKind::Link, name.to_string()))
            .padding([2, 8])
            .style(|_, _| danger_button_style());

        row![
            container(cb).width(28),
            container(name_cell).width(130),
            container(url_cell).width(Length::Fill),
            container(delete_btn).width(60),
        ]
        .spacing(8)
        .padding([6, 8])
        .align_y(iced::Alignment::Center)
        .into()
    }

    fn view_alias_row(&self, alias: &str, target: &str) -> Element<'_, Message> {
        let is_selected = self
            .selected
            .contains(&(ItemKind::Alias, alias.to_string()));

        let cb = checkbox(is_selected)
            .on_toggle({
                let alias = alias.to_string();
                move |_| Message::ToggleSelect(ItemKind::Alias, alias.clone())
            })
            .size(14)
            .style(|_, _| checkbox_style());

        let resolved_url = self.resolve_url(alias).map(String::from);

        let name_cell = self.view_editable_cell(
            ItemKind::Alias,
            alias,
            "name",
            alias.to_string(),
            colors::PURPLE,
            resolved_url,
        );

        let target_cell = self.view_editable_cell(
            ItemKind::Alias,
            alias,
            "target",
            target.to_string(),
            colors::PURPLE_DIM,
            None,
        );

        let delete_btn = button(text("delete").size(12).color(colors::RED))
            .on_press(Message::RequestDelete(ItemKind::Alias, alias.to_string()))
            .padding([2, 8])
            .style(|_, _| danger_button_style());

        row![
            container(cb).width(28),
            container(name_cell).width(130),
            container(target_cell).width(Length::Fill),
            container(delete_btn).width(60),
        ]
        .spacing(8)
        .padding([6, 8])
        .align_y(iced::Alignment::Center)
        .into()
    }

    fn view_group_row<'a>(&'a self, name: &'a str, entries: &[String]) -> Element<'a, Message> {
        let is_selected = self.selected.contains(&(ItemKind::Group, name.to_string()));

        let cb = checkbox(is_selected)
            .on_toggle({
                let name = name.to_string();
                move |_| Message::ToggleSelect(ItemKind::Group, name.clone())
            })
            .size(14)
            .style(|_, _| checkbox_style());

        let urls: Vec<String> = entries
            .iter()
            .filter_map(|e| self.resolve_url(e).map(String::from))
            .collect();

        let name_cell: Element<'_, Message> = if !urls.is_empty() {
            button(text(name).size(13).color(colors::PURPLE))
                .on_press(Message::OpenUrl(urls[0].clone()))
                .padding(0)
                .style(|_, _| button::Style::default())
                .into()
        } else {
            text(name).size(13).color(colors::PURPLE).into()
        };

        let entries_str = entries.join(", ");
        let entries_cell = self.view_editable_cell(
            ItemKind::Group,
            name,
            "entries",
            entries_str,
            colors::PURPLE_DIM,
            None,
        );

        let delete_btn = button(text("delete").size(12).color(colors::RED))
            .on_press(Message::RequestDelete(ItemKind::Group, name.to_string()))
            .padding([2, 8])
            .style(|_, _| danger_button_style());

        row![
            container(cb).width(28),
            container(name_cell).width(130),
            container(entries_cell).width(Length::Fill),
            container(delete_btn).width(60),
        ]
        .spacing(8)
        .padding([6, 8])
        .align_y(iced::Alignment::Center)
        .into()
    }

    fn view_editable_cell(
        &self,
        kind: ItemKind,
        item_name: &str,
        field: &'static str,
        display_value: String,
        color: iced::Color,
        open_url: Option<String>,
    ) -> Element<'_, Message> {
        // Check if this cell is being edited
        if let Some(ref edit) = self.editing
            && edit.kind == kind
            && edit.name == item_name
            && edit.field == field
        {
            return text_input("", &edit.value)
                .on_input(Message::EditChanged)
                .on_submit(Message::CommitEdit)
                .size(13)
                .width(Length::Fill)
                .style(|_, status| edit_input_style(status))
                .into();
        }

        // Normal display - clickable to edit
        let display: Element<'_, Message> = if let Some(url) = open_url {
            button(text(display_value.clone()).size(13).color(color))
                .on_press(Message::OpenUrl(url))
                .padding(0)
                .style(|_, _| button::Style::default())
                .into()
        } else {
            text(display_value.clone()).size(13).color(color).into()
        };

        button(display)
            .on_press(Message::StartEdit(
                kind,
                item_name.to_string(),
                field,
                display_value,
            ))
            .padding([2, 4])
            .width(Length::Fill)
            .style(|_, status| editable_cell_style(status))
            .into()
    }

    fn view_confirm_modal<'a>(&self, confirm: &'a ConfirmState) -> Element<'a, Message> {
        let title = text(&confirm.title).size(16).color(colors::TEXT_BRIGHT);
        let message = text(&confirm.message).size(13).color(colors::TEXT);

        let cancel_btn = button(text("cancel").size(13).color(colors::TEXT))
            .on_press(Message::ConfirmNo)
            .padding([6, 16])
            .style(|_, _| default_button_style());

        let confirm_btn = button(text("delete").size(13).color(colors::RED))
            .on_press(Message::ConfirmYes)
            .padding([6, 16])
            .style(|_, _| button::Style {
                background: Some(iced::Background::Color(colors::RED_BG)),
                border: iced::Border {
                    color: colors::RED,
                    width: 1.0,
                    radius: 4.0.into(),
                },
                text_color: colors::RED,
                ..Default::default()
            });

        let modal_content = container(
            column![
                title,
                message,
                row![
                    iced::widget::Space::new().width(Length::Fill),
                    cancel_btn,
                    confirm_btn
                ]
                .spacing(8)
                .align_y(iced::Alignment::Center),
            ]
            .spacing(12),
        )
        .padding(24)
        .max_width(400)
        .style(|_| container::Style {
            background: Some(iced::Background::Color(iced::Color::from_rgb(
                0.08, 0.08, 0.13,
            ))),
            border: iced::Border {
                color: colors::BORDER,
                width: 1.0,
                radius: 8.0.into(),
            },
            ..Default::default()
        });

        center(modal_content)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|_| container::Style {
                background: Some(iced::Background::Color(iced::Color::from_rgba(
                    0.0, 0.0, 0.0, 0.7,
                ))),
                ..Default::default()
            })
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }

    fn title(&self) -> String {
        "dkdc-links".into()
    }
}

// -- Style helpers -----------------------------------------------------------

fn input_style(status: text_input::Status) -> text_input::Style {
    let border_color = match status {
        text_input::Status::Focused { .. } => colors::BORDER_FOCUS,
        _ => colors::BORDER,
    };
    text_input::Style {
        background: iced::Background::Color(colors::BG_INPUT),
        border: iced::Border {
            color: border_color,
            width: 1.0,
            radius: 4.0.into(),
        },
        icon: colors::TEXT_DIM,
        placeholder: colors::TEXT_DIM,
        value: colors::TEXT_BRIGHT,
        selection: colors::PURPLE,
    }
}

fn edit_input_style(_status: text_input::Status) -> text_input::Style {
    text_input::Style {
        background: iced::Background::Color(colors::BG_INPUT),
        border: iced::Border {
            color: colors::PURPLE,
            width: 1.0,
            radius: 3.0.into(),
        },
        icon: colors::TEXT_DIM,
        placeholder: colors::TEXT_DIM,
        value: colors::TEXT_BRIGHT,
        selection: colors::PURPLE,
    }
}

fn tab_button_style(is_active: bool, _status: button::Status) -> button::Style {
    button::Style {
        background: if is_active {
            Some(iced::Background::Color(colors::TAB_ACTIVE_BG))
        } else {
            None
        },
        border: iced::Border {
            color: if is_active {
                colors::PURPLE
            } else {
                colors::BORDER
            },
            width: 1.0,
            radius: 4.0.into(),
        },
        text_color: if is_active {
            colors::PURPLE
        } else {
            colors::TEXT
        },
        ..Default::default()
    }
}

fn default_button_style() -> button::Style {
    button::Style {
        background: None,
        border: iced::Border {
            color: colors::BORDER,
            width: 1.0,
            radius: 4.0.into(),
        },
        text_color: colors::TEXT,
        ..Default::default()
    }
}

fn danger_button_style() -> button::Style {
    button::Style {
        background: None,
        border: iced::Border {
            color: colors::RED_BORDER,
            width: 1.0,
            radius: 4.0.into(),
        },
        text_color: colors::RED,
        ..Default::default()
    }
}

fn add_button_style() -> button::Style {
    button::Style {
        background: Some(iced::Background::Color(colors::BG_INPUT)),
        border: iced::Border {
            color: colors::BORDER,
            width: 1.0,
            radius: 4.0.into(),
        },
        text_color: colors::PURPLE,
        ..Default::default()
    }
}

fn checkbox_style() -> checkbox::Style {
    checkbox::Style {
        background: iced::Background::Color(colors::BG_INPUT),
        icon_color: colors::PURPLE,
        border: iced::Border {
            color: colors::BORDER,
            width: 1.0,
            radius: 3.0.into(),
        },
        text_color: Some(colors::TEXT),
    }
}

fn editable_cell_style(status: button::Status) -> button::Style {
    button::Style {
        background: match status {
            button::Status::Hovered => Some(iced::Background::Color(colors::BG_HOVER)),
            _ => None,
        },
        border: iced::Border {
            radius: 3.0.into(),
            ..Default::default()
        },
        text_color: colors::TEXT,
        ..Default::default()
    }
}

fn rule_style() -> iced::widget::rule::Style {
    iced::widget::rule::Style {
        color: iced::Color::from_rgb(0.14, 0.14, 0.22),
        radius: 0.0.into(),
        fill_mode: iced::widget::rule::FillMode::Full,
        snap: false,
    }
}

// -- Entry point -------------------------------------------------------------

pub fn run(storage: Box<dyn Storage>) -> iced::Result {
    use std::cell::RefCell;
    let storage = RefCell::new(Some(storage));
    iced::application(
        move || {
            let s = storage
                .borrow_mut()
                .take()
                .expect("boot called more than once");
            Links::new(s)
        },
        Links::update,
        Links::view,
    )
    .title(Links::title)
    .theme(Links::theme)
    .antialiasing(true)
    .window_size(Size::new(720.0, 800.0))
    .run()
}
