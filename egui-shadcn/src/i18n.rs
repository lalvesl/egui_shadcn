//! Bakes every user-facing string in the component library into compile-time
//! catalogs, one per language. Only `en-US` and `pt-BR` are wired up today.
//!
//! Usage:
//!
//! ```
//! use egui_shadcn::i18n;
//! i18n::set_language(i18n::Languages::PtBr);   // global; reads in t!()
//! ```

pub use ::i18n::{current_language, set_language, set_source, Languages, Source, Translate};

::i18n::traductions! {
    pub enum Calendar {
        MonthLong1([EnUs("January"),   PtBr("Janeiro")]),
        MonthLong2([EnUs("February"),  PtBr("Fevereiro")]),
        MonthLong3([EnUs("March"),     PtBr("Março")]),
        MonthLong4([EnUs("April"),     PtBr("Abril")]),
        MonthLong5([EnUs("May"),       PtBr("Maio")]),
        MonthLong6([EnUs("June"),      PtBr("Junho")]),
        MonthLong7([EnUs("July"),      PtBr("Julho")]),
        MonthLong8([EnUs("August"),    PtBr("Agosto")]),
        MonthLong9([EnUs("September"), PtBr("Setembro")]),
        MonthLong10([EnUs("October"),  PtBr("Outubro")]),
        MonthLong11([EnUs("November"), PtBr("Novembro")]),
        MonthLong12([EnUs("December"), PtBr("Dezembro")]),

        MonthShort1([EnUs("Jan"),  PtBr("Jan")]),
        MonthShort2([EnUs("Feb"),  PtBr("Fev")]),
        MonthShort3([EnUs("Mar"),  PtBr("Mar")]),
        MonthShort4([EnUs("Apr"),  PtBr("Abr")]),
        MonthShort5([EnUs("May"),  PtBr("Mai")]),
        MonthShort6([EnUs("Jun"),  PtBr("Jun")]),
        MonthShort7([EnUs("Jul"),  PtBr("Jul")]),
        MonthShort8([EnUs("Aug"),  PtBr("Ago")]),
        MonthShort9([EnUs("Sep"),  PtBr("Set")]),
        MonthShort10([EnUs("Oct"), PtBr("Out")]),
        MonthShort11([EnUs("Nov"), PtBr("Nov")]),
        MonthShort12([EnUs("Dec"), PtBr("Dez")]),

        Day0([EnUs("Su"), PtBr("Dom")]),
        Day1([EnUs("Mo"), PtBr("Seg")]),
        Day2([EnUs("Tu"), PtBr("Ter")]),
        Day3([EnUs("We"), PtBr("Qua")]),
        Day4([EnUs("Th"), PtBr("Qui")]),
        Day5([EnUs("Fr"), PtBr("Sex")]),
        Day6([EnUs("Sa"), PtBr("Sáb")]),
    }
}

::i18n::traductions! {
    pub enum Combobox {
        Placeholder([EnUs("Select…"),       PtBr("Selecionar…")]),
        SearchPlaceholder([EnUs("Search…"), PtBr("Pesquisar…")]),
    }
}

::i18n::traductions! {
    pub enum Command {
        Placeholder([EnUs("Type a command or search…"), PtBr("Digite um comando ou pesquise…")]),
        NoResults([EnUs("No results found."),           PtBr("Nenhum resultado encontrado.")]),
    }
}

::i18n::traductions! {
    pub enum DatePicker {
        Placeholder([EnUs("Pick a date"), PtBr("Escolha uma data")]),
    }
}

::i18n::traductions! {
    pub enum Select {
        Placeholder([EnUs("Select…"), PtBr("Selecionar…")]),
    }
}

::i18n::traductions! {
    pub enum AlertDialog {
        Cancel([EnUs("Cancel"),    PtBr("Cancelar")]),
        Confirm([EnUs("Continue"), PtBr("Continuar")]),
    }
}

::i18n::traductions! {
    pub enum DataTable {
        FilterPlaceholder([EnUs("Filter…"), PtBr("Filtrar…")]),
        Prev([EnUs("← Prev"),               PtBr("← Ant.")]),
        Next([EnUs("Next →"),               PtBr("Próx. →")]),
        Page([EnUs("Page {current} of {total}"), PtBr("Página {current} de {total}")]),
    }
}

/// Long month name (1..=12) for the active language.
pub fn month_long(month: u8) -> std::borrow::Cow<'static, str> {
    use Calendar::*;
    let v = match month {
        1 => MonthLong1,  2 => MonthLong2,  3 => MonthLong3,  4 => MonthLong4,
        5 => MonthLong5,  6 => MonthLong6,  7 => MonthLong7,  8 => MonthLong8,
        9 => MonthLong9,  10 => MonthLong10, 11 => MonthLong11, 12 => MonthLong12,
        _ => return std::borrow::Cow::Borrowed(""),
    };
    ::i18n::translate(Calendar::APP_ID, v as u8, None, &[])
}

/// Abbreviated month name (1..=12) for the active language.
pub fn month_short(month: u8) -> std::borrow::Cow<'static, str> {
    use Calendar::*;
    let v = match month {
        1 => MonthShort1,  2 => MonthShort2,  3 => MonthShort3,  4 => MonthShort4,
        5 => MonthShort5,  6 => MonthShort6,  7 => MonthShort7,  8 => MonthShort8,
        9 => MonthShort9,  10 => MonthShort10, 11 => MonthShort11, 12 => MonthShort12,
        _ => return std::borrow::Cow::Borrowed(""),
    };
    ::i18n::translate(Calendar::APP_ID, v as u8, None, &[])
}

/// Weekday label (0..=6, Sunday-first) for the active language.
pub fn weekday_short(day: usize) -> std::borrow::Cow<'static, str> {
    use Calendar::*;
    let v = match day {
        0 => Day0, 1 => Day1, 2 => Day2, 3 => Day3,
        4 => Day4, 5 => Day5, 6 => Day6,
        _ => return std::borrow::Cow::Borrowed(""),
    };
    ::i18n::translate(Calendar::APP_ID, v as u8, None, &[])
}
