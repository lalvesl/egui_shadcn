//! Demo-only i18n catalogs. Component-level strings live in `egui_shadcn::i18n`;
//! this module covers every string the demo app itself renders.

use std::borrow::Cow;

pub use ::i18n::{set_language, current_language, Languages, Translate};

// ── Section navigation list (sidebar + toolbar headings) ─────────────────────

::i18n::traductions! {
    pub enum SectionName {
        Overview([EnUs("Overview"),                     PtBr("Visão geral")]),
        Accordion([EnUs("Accordion"),                   PtBr("Acordeão")]),
        Alert([EnUs("Alert"),                           PtBr("Alerta")]),
        AlertDialog([EnUs("Alert Dialog"),              PtBr("Caixa de alerta")]),
        Avatar([EnUs("Avatar"),                         PtBr("Avatar")]),
        Badge([EnUs("Badge"),                           PtBr("Distintivo")]),
        Boxed([EnUs("Boxed"),                           PtBr("Caixa")]),
        Breadcrumb([EnUs("Breadcrumb"),                 PtBr("Trilha")]),
        Button([EnUs("Button"),                         PtBr("Botão")]),
        ButtonGroup([EnUs("Button Group"),              PtBr("Grupo de botões")]),
        Calendar([EnUs("Calendar"),                     PtBr("Calendário")]),
        Card([EnUs("Card"),                             PtBr("Cartão")]),
        Carousel([EnUs("Carousel"),                     PtBr("Carrossel")]),
        Chart([EnUs("Chart"),                           PtBr("Gráfico")]),
        Checkbox([EnUs("Checkbox"),                     PtBr("Caixa de seleção")]),
        Collapsible([EnUs("Collapsible"),               PtBr("Recolhível")]),
        Combobox([EnUs("Combobox"),                     PtBr("Combobox")]),
        Command([EnUs("Command"),                       PtBr("Comando")]),
        ContextMenu([EnUs("Context Menu"),              PtBr("Menu de contexto")]),
        DataTable([EnUs("Data Table"),                  PtBr("Tabela de dados")]),
        DatePicker([EnUs("Date Picker"),                PtBr("Seletor de data")]),
        Dialog([EnUs("Dialog"),                         PtBr("Diálogo")]),
        Drawer([EnUs("Drawer"),                         PtBr("Gaveta")]),
        DropdownMenu([EnUs("Dropdown Menu"),            PtBr("Menu suspenso")]),
        HoverCard([EnUs("Hover Card"),                  PtBr("Cartão flutuante")]),
        Input([EnUs("Input"),                           PtBr("Entrada")]),
        InputOtp([EnUs("Input OTP"),                    PtBr("Entrada OTP")]),
        Label([EnUs("Label"),                           PtBr("Rótulo")]),
        Menubar([EnUs("Menubar"),                       PtBr("Barra de menu")]),
        NavigationMenu([EnUs("Navigation Menu"),        PtBr("Menu de navegação")]),
        Pagination([EnUs("Pagination"),                 PtBr("Paginação")]),
        Popover([EnUs("Popover"),                       PtBr("Popover")]),
        Progress([EnUs("Progress"),                     PtBr("Progresso")]),
        Radio([EnUs("Radio"),                           PtBr("Rádio")]),
        Resizable([EnUs("Resizable"),                   PtBr("Redimensionável")]),
        Select([EnUs("Select"),                         PtBr("Seleção")]),
        Separator([EnUs("Separator"),                   PtBr("Separador")]),
        Sheet([EnUs("Sheet"),                           PtBr("Painel")]),
        Skeleton([EnUs("Skeleton"),                     PtBr("Esqueleto")]),
        Slider([EnUs("Slider"),                         PtBr("Controle deslizante")]),
        Spacing([EnUs("Spacing"),                       PtBr("Espaçamento")]),
        Spinner([EnUs("Spinner"),                       PtBr("Indicador")]),
        Switch([EnUs("Switch"),                         PtBr("Interruptor")]),
        Table([EnUs("Table"),                           PtBr("Tabela")]),
        Tabs([EnUs("Tabs"),                             PtBr("Abas")]),
        Textarea([EnUs("Textarea"),                     PtBr("Área de texto")]),
        Toast([EnUs("Toast"),                           PtBr("Notificação")]),
        Toggle([EnUs("Toggle"),                         PtBr("Alternador")]),
        ToggleGroup([EnUs("Toggle Group"),              PtBr("Grupo de alternadores")]),
        Tooltip([EnUs("Tooltip"),                       PtBr("Dica")]),
        Typography([EnUs("Typography"),                 PtBr("Tipografia")]),
    }
}

/// Map a sidebar index to the localized section name.
pub fn section_name(idx: usize) -> Cow<'static, str> {
    use SectionName::*;
    let v = match idx {
        0 => Overview,        1 => Accordion,     2 => Alert,        3 => AlertDialog,
        4 => Avatar,          5 => Badge,         6 => Boxed,        7 => Breadcrumb,
        8 => Button,          9 => ButtonGroup,   10 => Calendar,    11 => Card,
        12 => Carousel,       13 => Chart,        14 => Checkbox,    15 => Collapsible,
        16 => Combobox,       17 => Command,      18 => ContextMenu, 19 => DataTable,
        20 => DatePicker,     21 => Dialog,       22 => Drawer,      23 => DropdownMenu,
        24 => HoverCard,      25 => Input,        26 => InputOtp,    27 => Label,
        28 => Menubar,        29 => NavigationMenu, 30 => Pagination, 31 => Popover,
        32 => Progress,       33 => Radio,        34 => Resizable,   35 => Select,
        36 => Separator,      37 => Sheet,        38 => Skeleton,    39 => Slider,
        40 => Spacing,        41 => Spinner,      42 => Switch,      43 => Table,
        44 => Tabs,           45 => Textarea,     46 => Toast,       47 => Toggle,
        48 => ToggleGroup,    49 => Tooltip,      50 => Typography,
        _ => return Cow::Borrowed(""),
    };
    ::i18n::translate(SectionName::APP_ID, v as u8, None, &[])
}

pub const SECTION_COUNT: usize = 51;

// ── Toolbar / dialog ────────────────────────────────────────────────────────

::i18n::traductions! {
    pub enum Toolbar {
        AppName([EnUs("egui-shadcn"),                       PtBr("egui-shadcn")]),
        AppSubtitle([EnUs("component demo"),                PtBr("demonstração de componentes")]),
        PrimaryColor([EnUs("Primary Color"),                PtBr("Cor primária")]),
        CustomHue([EnUs("Custom hue"),                      PtBr("Matiz personalizada")]),
        ResetToZinc([EnUs("Reset to Zinc"),                 PtBr("Restaurar para Zinco")]),
        ColorZinc([EnUs("Zinc"),                            PtBr("Zinco")]),
        ColorRed([EnUs("Red"),                              PtBr("Vermelho")]),
        ColorOrange([EnUs("Orange"),                        PtBr("Laranja")]),
        ColorYellow([EnUs("Yellow"),                        PtBr("Amarelo")]),
        ColorGreen([EnUs("Green"),                          PtBr("Verde")]),
        ColorBlue([EnUs("Blue"),                            PtBr("Azul")]),
        ColorViolet([EnUs("Violet"),                        PtBr("Violeta")]),
        ColorPink([EnUs("Pink"),                            PtBr("Rosa")]),
    }
}

::i18n::traductions! {
    pub enum DemoDialog {
        Title([EnUs("Confirm Action"),                      PtBr("Confirmar ação")]),
        Body([EnUs("Are you sure you want to perform this action? This cannot be undone."),
              PtBr("Você tem certeza que deseja executar esta ação? Ela não pode ser desfeita.")]),
        TypePrompt([EnUs("Type CONFIRM to proceed"),        PtBr("Digite CONFIRM para prosseguir")]),
        Placeholder([EnUs("CONFIRM"),                       PtBr("CONFIRM")]),
        Cancel([EnUs("Cancel"),                             PtBr("Cancelar")]),
        Continue([EnUs("Continue"),                         PtBr("Continuar")]),
    }
}

// ── Section: Button ─────────────────────────────────────────────────────────

::i18n::traductions! {
    pub enum BtnSec {
        Subtitle([EnUs("Displays a button or a component that looks like a button."),
                  PtBr("Mostra um botão ou um componente com aparência de botão.")]),
        HVariants([EnUs("Variants"),                        PtBr("Variantes")]),
        HSizes([EnUs("Sizes"),                              PtBr("Tamanhos")]),
        HStates([EnUs("States"),                            PtBr("Estados")]),
        HWithIcon([EnUs("With Icon"),                       PtBr("Com ícone")]),
        Default([EnUs("Default"),                           PtBr("Padrão")]),
        Destructive([EnUs("Destructive"),                   PtBr("Destrutivo")]),
        Outline([EnUs("Outline"),                           PtBr("Contorno")]),
        Secondary([EnUs("Secondary"),                       PtBr("Secundário")]),
        Ghost([EnUs("Ghost"),                               PtBr("Fantasma")]),
        Link([EnUs("Link"),                                 PtBr("Link")]),
        Small([EnUs("Small"),                               PtBr("Pequeno")]),
        Large([EnUs("Large"),                               PtBr("Grande")]),
        Enabled([EnUs("Enabled"),                           PtBr("Habilitado")]),
        Disabled([EnUs("Disabled"),                         PtBr("Desabilitado")]),
        Send([EnUs("Send"),                                 PtBr("Enviar")]),
        Mail([EnUs("Mail"),                                 PtBr("E-mail")]),
        Download([EnUs("Download"),                         PtBr("Baixar")]),
        Search([EnUs("Search"),                             PtBr("Pesquisar")]),
        Add([EnUs("Add"),                                   PtBr("Adicionar")]),
        Clicked([EnUs("Button clicked!"),                   PtBr("Botão clicado!")]),
        Dismiss([EnUs("Dismiss"),                           PtBr("Dispensar")]),
    }
}

::i18n::traductions! {
    pub enum BadgeSec {
        Subtitle([EnUs("Displays a badge or a component that looks like a badge."),
                  PtBr("Mostra um distintivo ou um componente com aparência de distintivo.")]),
        HVariants([EnUs("Variants"),                        PtBr("Variantes")]),
        Default([EnUs("Default"),                           PtBr("Padrão")]),
        Secondary([EnUs("Secondary"),                       PtBr("Secundário")]),
        Destructive([EnUs("Destructive"),                   PtBr("Destrutivo")]),
        Outline([EnUs("Outline"),                           PtBr("Contorno")]),
    }
}

::i18n::traductions! {
    pub enum BtnGrpSec {
        Subtitle([EnUs("Multiple related actions in a connected strip."),
                  PtBr("Várias ações relacionadas em uma faixa conectada.")]),
        HSingle([EnUs("Single select"),                     PtBr("Seleção única")]),
        HOutline([EnUs("Outline variant"),                  PtBr("Variante de contorno")]),
        Months([EnUs("Months"),                             PtBr("Meses")]),
        Weeks([EnUs("Weeks"),                               PtBr("Semanas")]),
        Days([EnUs("Days"),                                 PtBr("Dias")]),
        Left([EnUs("Left"),                                 PtBr("Esquerda")]),
        Center([EnUs("Center"),                             PtBr("Centro")]),
        Right([EnUs("Right"),                               PtBr("Direita")]),
    }
}

::i18n::traductions! {
    pub enum ToggleSec {
        Subtitle([EnUs("A two-state button that can be either on or off."),
                  PtBr("Um botão de dois estados que pode estar ligado ou desligado.")]),
        HBasic([EnUs("Basic"),                              PtBr("Básico")]),
        HWithIcon([EnUs("With Icon"),                       PtBr("Com ícone")]),
        HDisabled([EnUs("Disabled"),                        PtBr("Desabilitado")]),
        Bold([EnUs("Bold"),                                 PtBr("Negrito")]),
        Italic([EnUs("Italic"),                             PtBr("Itálico")]),
        Underline([EnUs("Underline"),                       PtBr("Sublinhado")]),
        Strikethrough([EnUs("Strikethrough"),               PtBr("Tachado")]),
        DisabledOn([EnUs("Disabled On"),                    PtBr("Desabilitado Ligado")]),
        DisabledOff([EnUs("Disabled Off"),                  PtBr("Desabilitado Desligado")]),
    }
}

::i18n::traductions! {
    pub enum ToggleGrpSec {
        Subtitle([EnUs("A set of two-state buttons that can be toggled on or off."),
                  PtBr("Um conjunto de botões de dois estados que podem ser alternados.")]),
        HView([EnUs("View"),                                PtBr("Visualização")]),
        Day([EnUs("Day"),                                   PtBr("Dia")]),
        Week([EnUs("Week"),                                 PtBr("Semana")]),
        Month([EnUs("Month"),                               PtBr("Mês")]),
        Selected([EnUs("Selected: {value}"),                PtBr("Selecionado: {value}")]),
    }
}

// ── Section: Avatar / Boxed / Calendar / Card / Collapsible / Data Table / Separator / Spacing / Table ──

::i18n::traductions! {
    pub enum AvatarSec {
        Subtitle([EnUs("An image element with a fallback for representing the user."),
                  PtBr("Um elemento de imagem com substituição para representar o usuário.")]),
        HSizes([EnUs("Sizes"),                              PtBr("Tamanhos")]),
    }
}

::i18n::traductions! {
    pub enum BoxedSec {
        Subtitle([EnUs("Transparent layout container with standard padding and optional margin."),
                  PtBr("Container de layout transparente com preenchimento padrão e margem opcional.")]),
        HPadding([EnUs("Padding variants"),                 PtBr("Variantes de preenchimento")]),
        HPadMargin([EnUs("Padding + margin"),               PtBr("Preenchimento + margem")]),
        HPadMarginDesc([EnUs("Padding inside, margin outside."),
                        PtBr("Preenchimento por dentro, margem por fora.")]),
        LblXs([EnUs("Xs (4px)"),                            PtBr("Xs (4px)")]),
        LblSm([EnUs("Sm (8px)"),                            PtBr("Sm (8px)")]),
        LblMd([EnUs("Md (12px)"),                           PtBr("Md (12px)")]),
        LblLg([EnUs("Lg (16px)"),                           PtBr("Lg (16px)")]),
        LblXl([EnUs("Xl (24px)"),                           PtBr("Xl (24px)")]),
        InnerLabel([EnUs("Lg padding · Sm margin"),         PtBr("Preenchimento Lg · Margem Sm")]),
    }
}

::i18n::traductions! {
    pub enum CalendarSec {
        Subtitle([EnUs("A date field component that allows users to enter and edit date."),
                  PtBr("Um componente de campo de data que permite inserir e editar datas.")]),
        HSingle([EnUs("Single date"),                       PtBr("Data única")]),
        HRange([EnUs("Date range"),                         PtBr("Intervalo de datas")]),
        HRangeDesc([EnUs("Click a start date then an end date."),
                    PtBr("Clique em uma data inicial e em uma data final.")]),
        HRangeCompact([EnUs("Date range (compact)"),        PtBr("Intervalo de datas (compacto)")]),
        HRangeCompactDesc([EnUs("Single-month layout for tight UIs."),
                           PtBr("Layout de mês único para interfaces compactas.")]),
        HPrices([EnUs("Prices"),                            PtBr("Preços")]),
        HPricesDesc([EnUs("Custom cell content — price per day."),
                     PtBr("Conteúdo de célula personalizado — preço por dia.")]),
        SelectedDate([EnUs("Selected: {date}"),             PtBr("Selecionado: {date}")]),
        NoDate([EnUs("No date selected"),                   PtBr("Nenhuma data selecionada")]),
        RangeNone([EnUs("No range selected"),               PtBr("Nenhum intervalo selecionado")]),
        RangePending([EnUs("{start}  →  (pick end)"),       PtBr("{start}  →  (escolha o fim)")]),
        Clear([EnUs("Clear"),                               PtBr("Limpar")]),
    }
}

::i18n::traductions! {
    pub enum CardSec {
        Subtitle([EnUs("Displays a card with header, content, and footer."),
                  PtBr("Mostra um cartão com cabeçalho, conteúdo e rodapé.")]),
        HTitle([EnUs("Card Title"),                         PtBr("Título do cartão")]),
        HTitleDesc([EnUs("A brief description of the card content."),
                    PtBr("Uma breve descrição do conteúdo do cartão.")]),
        Body([EnUs("Card body content goes here."),         PtBr("O conteúdo do cartão vai aqui.")]),
        HUser([EnUs("User Card"),                           PtBr("Cartão de usuário")]),
        UserName([EnUs("John Doe"),                         PtBr("Fulano de Tal")]),
        UserEmail([EnUs("john@example.com"),                PtBr("fulano@exemplo.com")]),
    }
}

::i18n::traductions! {
    pub enum CollapsibleSec {
        Subtitle([EnUs("An interactive component which expands/collapses a panel."),
                  PtBr("Um componente interativo que expande/recolhe um painel.")]),
        Heading([EnUs("Starred repositories"),              PtBr("Repositórios favoritos")]),
    }
}

::i18n::traductions! {
    pub enum DataTableSec {
        Subtitle([EnUs("Powerful table and datagrids with sorting, filtering, and pagination."),
                  PtBr("Tabela poderosa com ordenação, filtragem e paginação.")]),
        HPayments([EnUs("Payments"),                        PtBr("Pagamentos")]),
        ColStatus([EnUs("Status"),                          PtBr("Estado")]),
        ColEmail([EnUs("Email"),                            PtBr("E-mail")]),
        ColAmount([EnUs("Amount"),                          PtBr("Valor")]),
        StatusSuccess([EnUs("Success"),                     PtBr("Sucesso")]),
        StatusProcessing([EnUs("Processing"),               PtBr("Processando")]),
        StatusFailed([EnUs("Failed"),                       PtBr("Falhou")]),
    }
}

::i18n::traductions! {
    pub enum SeparatorSec {
        Subtitle([EnUs("Visually or semantically separates content."),
                  PtBr("Separa conteúdo visualmente ou semanticamente.")]),
        HHoriz([EnUs("Horizontal"),                         PtBr("Horizontal")]),
        HVert([EnUs("Vertical"),                            PtBr("Vertical")]),
        MyAccount([EnUs("My Account"),                      PtBr("Minha conta")]),
        Profile([EnUs("Profile"),                           PtBr("Perfil")]),
        Billing([EnUs("Billing"),                           PtBr("Cobrança")]),
        Settings([EnUs("Settings"),                         PtBr("Configurações")]),
        Blog([EnUs("Blog"),                                 PtBr("Blog")]),
        Docs([EnUs("Docs"),                                 PtBr("Docs")]),
        Source([EnUs("Source"),                             PtBr("Fonte")]),
    }
}

::i18n::traductions! {
    pub enum SpacingSec {
        Subtitle([EnUs("Standardized spacing scale based on a 4 px base unit."),
                  PtBr("Escala padronizada de espaçamento baseada em unidade de 4 px.")]),
        HScale([EnUs("Scale"),                              PtBr("Escala")]),
        HScaleDesc([EnUs("Xs=4 · Sm=8 · Md=12 · Lg=16 · Xl=24 · Xl2=32 · Xl3=48"),
                    PtBr("Xs=4 · Sm=8 · Md=12 · Lg=16 · Xl=24 · Xl2=32 · Xl3=48")]),
        HUsage([EnUs("Usage"),                              PtBr("Uso")]),
        HUsageDesc([EnUs("Spacing::Lg.show(ui)  or  f32::from(Spacing::Lg)"),
                    PtBr("Spacing::Lg.show(ui)  ou  f32::from(Spacing::Lg)")]),
        BlockA([EnUs("Block A"),                            PtBr("Bloco A")]),
        BlockB([EnUs("Block B  (Lg = 16px gap above)"),     PtBr("Bloco B  (Lg = 16px de espaço acima)")]),
        BlockC([EnUs("Block C  (Xl = 24px gap above)"),     PtBr("Bloco C  (Xl = 24px de espaço acima)")]),
    }
}

::i18n::traductions! {
    pub enum TableSec {
        Subtitle([EnUs("A responsive table component."),
                  PtBr("Um componente de tabela responsivo.")]),
        HInvoices([EnUs("Recent Invoices"),                 PtBr("Faturas recentes")]),
        ColInvoice([EnUs("Invoice"),                        PtBr("Fatura")]),
        ColStatus([EnUs("Status"),                          PtBr("Estado")]),
        ColMethod([EnUs("Method"),                          PtBr("Método")]),
        ColAmount([EnUs("Amount"),                          PtBr("Valor")]),
        StatusPaid([EnUs("Paid"),                           PtBr("Pago")]),
        StatusPending([EnUs("Pending"),                     PtBr("Pendente")]),
        StatusUnpaid([EnUs("Unpaid"),                       PtBr("Não pago")]),
        MethodCreditCard([EnUs("Credit Card"),              PtBr("Cartão de crédito")]),
        MethodPayPal([EnUs("PayPal"),                       PtBr("PayPal")]),
        MethodBank([EnUs("Bank Transfer"),                  PtBr("Transferência bancária")]),
    }
}

// ── Section: Feedback ──────────────────────────────────────────────────────

::i18n::traductions! {
    pub enum AlertSec {
        Subtitle([EnUs("Displays a callout for user attention."),
                  PtBr("Mostra um aviso destinado a chamar a atenção do usuário.")]),
        InfoTitle([EnUs("Heads up!"),                       PtBr("Atenção!")]),
        InfoDesc([EnUs("This is an informational alert with a description."),
                  PtBr("Este é um alerta informativo com descrição.")]),
        DestructiveTitle([EnUs("Destructive"),              PtBr("Destrutivo")]),
        DestructiveDesc([EnUs("Something went wrong. Please check your inputs."),
                         PtBr("Algo deu errado. Verifique seus dados.")]),
        WarningTitle([EnUs("Warning"),                      PtBr("Aviso")]),
        WarningDesc([EnUs("This action cannot be undone. Proceed with caution."),
                     PtBr("Esta ação não pode ser desfeita. Prossiga com cautela.")]),
    }
}

::i18n::traductions! {
    pub enum AlertDialogSec {
        Subtitle([EnUs("A modal dialog that interrupts the user with important content and expects a response."),
                  PtBr("Uma janela modal que interrompe o usuário com conteúdo importante e espera uma resposta.")]),
        HDefault([EnUs("Default"),                          PtBr("Padrão")]),
        Confirmed([EnUs("Action confirmed!"),               PtBr("Ação confirmada!")]),
        Open([EnUs("Open Alert Dialog"),                    PtBr("Abrir caixa de alerta")]),
        DialogTitle([EnUs("Are you absolutely sure?"),      PtBr("Você tem certeza absoluta?")]),
        DialogBody([EnUs("This action cannot be undone. This will permanently delete your account and remove your data from our servers."),
                    PtBr("Esta ação não pode ser desfeita. Sua conta será excluída permanentemente e seus dados removidos de nossos servidores.")]),
        Delete([EnUs("Delete Account"),                     PtBr("Excluir conta")]),
    }
}

::i18n::traductions! {
    pub enum DialogSec {
        Subtitle([EnUs("A window overlaid on either the primary window or another dialog window, rendering the content underneath inert."),
                  PtBr("Uma janela sobreposta à janela principal ou a outra janela de diálogo, tornando o conteúdo abaixo inativo.")]),
        HConfirm([EnUs("Confirm Action"),                   PtBr("Confirmar ação")]),
        HConfirmDesc([EnUs("Type CONFIRM to proceed with this destructive action."),
                      PtBr("Digite CONFIRM para prosseguir com esta ação destrutiva.")]),
        Body([EnUs("This dialog requires explicit confirmation before proceeding."),
              PtBr("Este diálogo requer confirmação explícita antes de prosseguir.")]),
        Open([EnUs("Open Dialog"),                          PtBr("Abrir diálogo")]),
    }
}

::i18n::traductions! {
    pub enum ProgressSec {
        Subtitle([EnUs("Displays an indicator showing the completion progress of a task, typically displayed as a progress bar."),
                  PtBr("Mostra um indicador que exibe o progresso de uma tarefa, normalmente como uma barra.")]),
        HBar([EnUs("Progress bar"),                         PtBr("Barra de progresso")]),
    }
}

::i18n::traductions! {
    pub enum SkeletonSec {
        Subtitle([EnUs("Use to show a placeholder while content is loading."),
                  PtBr("Use para mostrar um espaço reservado enquanto o conteúdo carrega.")]),
        HLoading([EnUs("Loading state"),                    PtBr("Estado de carregamento")]),
    }
}

::i18n::traductions! {
    pub enum SpinnerSec {
        Subtitle([EnUs("Displays an animated spinner to indicate a loading state."),
                  PtBr("Mostra um indicador animado para indicar carregamento.")]),
        HSizes([EnUs("Sizes"),                              PtBr("Tamanhos")]),
    }
}

::i18n::traductions! {
    pub enum ToastSec {
        Subtitle([EnUs("A succinct message that is displayed temporarily."),
                  PtBr("Uma mensagem breve exibida temporariamente.")]),
        HVariants([EnUs("Variants"),                        PtBr("Variantes")]),
        HVariantsDesc([EnUs("Click to show a toast notification."),
                       PtBr("Clique para exibir uma notificação.")]),
        DefaultBtn([EnUs("Default"),                        PtBr("Padrão")]),
        SuccessBtn([EnUs("Success"),                        PtBr("Sucesso")]),
        WarningBtn([EnUs("Warning"),                        PtBr("Aviso")]),
        DestructiveBtn([EnUs("Destructive"),                PtBr("Destrutivo")]),
        DefaultTitle([EnUs("Scheduled"),                    PtBr("Agendado")]),
        DefaultDesc([EnUs("Monday, January 3rd at 6:00pm"), PtBr("Segunda-feira, 3 de janeiro às 18:00")]),
        SuccessTitle([EnUs("Success"),                      PtBr("Sucesso")]),
        SuccessDesc([EnUs("Your changes have been saved."), PtBr("Suas alterações foram salvas.")]),
        WarningTitle([EnUs("Warning"),                      PtBr("Aviso")]),
        WarningDesc([EnUs("This action may have consequences."),
                     PtBr("Esta ação pode ter consequências.")]),
        DestructiveTitle([EnUs("Destructive"),              PtBr("Destrutivo")]),
        DestructiveDesc([EnUs("Your session has expired."), PtBr("Sua sessão expirou.")]),
    }
}

// ── Section: Forms ──────────────────────────────────────────────────────────

::i18n::traductions! {
    pub enum CheckboxSec {
        Subtitle([EnUs("A control that allows the user to toggle between checked and not checked."),
                  PtBr("Um controle que alterna entre marcado e não marcado.")]),
        HOptions([EnUs("Options"),                          PtBr("Opções")]),
        Terms([EnUs("Accept terms and conditions"),         PtBr("Aceito os termos e condições")]),
        Newsletter([EnUs("Subscribe to newsletter"),        PtBr("Inscrever-se na newsletter")]),
        Disabled([EnUs("Disabled option"),                  PtBr("Opção desabilitada")]),
    }
}

::i18n::traductions! {
    pub enum ComboboxSec {
        Subtitle([EnUs("Autocomplete input and command palette with a list of suggestions."),
                  PtBr("Campo com autocompletar e paleta de comandos com sugestões.")]),
        HFramework([EnUs("Framework"),                      PtBr("Framework")]),
        HFrameworkDesc([EnUs("Select a framework to use."), PtBr("Selecione um framework para usar.")]),
        Placeholder([EnUs("Select framework…"),             PtBr("Selecionar framework…")]),
        Selected([EnUs("Selected: {value}"),                PtBr("Selecionado: {value}")]),
    }
}

::i18n::traductions! {
    pub enum DatePickerSec {
        Subtitle([EnUs("A date picker component with calendar popover."),
                  PtBr("Um seletor de data com popover de calendário.")]),
        HPick([EnUs("Pick a date"),                         PtBr("Escolha uma data")]),
        Selected([EnUs("Selected: {date}"),                 PtBr("Selecionado: {date}")]),
    }
}

::i18n::traductions! {
    pub enum InputSec {
        Subtitle([EnUs("Displays a form input field or a component that looks like an input field."),
                  PtBr("Mostra um campo de entrada ou um componente com aparência de entrada.")]),
        HText([EnUs("Text input"),                          PtBr("Entrada de texto")]),
        Username([EnUs("Username"),                         PtBr("Usuário")]),
        UsernamePh([EnUs("Enter your username…"),           PtBr("Digite seu usuário…")]),
        Password([EnUs("Password"),                         PtBr("Senha")]),
    }
}

::i18n::traductions! {
    pub enum InputOtpSec {
        Subtitle([EnUs("Accessible one-time password component with copy paste functionality."),
                  PtBr("Componente acessível de senha temporária com suporte a colar.")]),
        HOtp([EnUs("One-Time Password"),                    PtBr("Senha temporária")]),
        HOtpDesc([EnUs("Enter your one-time password."),    PtBr("Digite sua senha temporária.")]),
        Code([EnUs("Code: {code}"),                         PtBr("Código: {code}")]),
    }
}

::i18n::traductions! {
    pub enum LabelSec {
        Subtitle([EnUs("Renders an accessible label associated with controls."),
                  PtBr("Renderiza um rótulo acessível associado a controles.")]),
        HBasic([EnUs("Basic"),                              PtBr("Básico")]),
        Email([EnUs("Email address"),                       PtBr("Endereço de e-mail")]),
        Required([EnUs("Required field"),                   PtBr("Campo obrigatório")]),
        Password([EnUs("Password"),                         PtBr("Senha")]),
    }
}

::i18n::traductions! {
    pub enum RadioSec {
        Subtitle([EnUs("A set of checkable buttons where no more than one can be checked at a time."),
                  PtBr("Conjunto de botões selecionáveis onde apenas um pode estar marcado por vez.")]),
        HMethod([EnUs("Notification method"),               PtBr("Método de notificação")]),
        OptA([EnUs("Option A"),                             PtBr("Opção A")]),
        OptB([EnUs("Option B"),                             PtBr("Opção B")]),
        OptC([EnUs("Option C"),                             PtBr("Opção C")]),
    }
}

::i18n::traductions! {
    pub enum SelectSec {
        Subtitle([EnUs("Displays a list of options for the user to pick from—triggered by a button."),
                  PtBr("Mostra uma lista de opções para o usuário escolher — acionada por um botão.")]),
        HFramework([EnUs("Framework"),                      PtBr("Framework")]),
        Framework([EnUs("Framework"),                       PtBr("Framework")]),
        Placeholder([EnUs("Pick a framework…"),             PtBr("Escolha um framework…")]),
    }
}

::i18n::traductions! {
    pub enum SliderSec {
        Subtitle([EnUs("An input where the user selects a value from within a given range."),
                  PtBr("Uma entrada onde o usuário escolhe um valor dentro de um intervalo.")]),
        HVolume([EnUs("Volume"),                            PtBr("Volume")]),
        Value([EnUs("Value: {value}"),                      PtBr("Valor: {value}")]),
    }
}

::i18n::traductions! {
    pub enum SwitchSec {
        Subtitle([EnUs("A control that allows the user to toggle between on and off states."),
                  PtBr("Um controle que alterna entre ligado e desligado.")]),
        HSettings([EnUs("Settings"),                        PtBr("Configurações")]),
        Enable([EnUs("Enable notifications"),               PtBr("Habilitar notificações")]),
        Disabled([EnUs("Disabled switch"),                  PtBr("Interruptor desabilitado")]),
    }
}

::i18n::traductions! {
    pub enum TextareaSec {
        Subtitle([EnUs("Displays a form textarea or a component that looks like a textarea."),
                  PtBr("Mostra uma área de texto ou um componente com aparência de área de texto.")]),
        HVariants([EnUs("Variants"),                        PtBr("Variantes")]),
        LblDefault([EnUs("Default (fixed 4 rows)"),         PtBr("Padrão (4 linhas fixas)")]),
        PhDefault([EnUs("Tell us about yourself…"),         PtBr("Conte sobre você…")]),
        LblScroll([EnUs("Scroll (fixed 3 rows)"),           PtBr("Rolagem (3 linhas fixas)")]),
        PhScroll([EnUs("Keep typing — content scrolls inside…"),
                  PtBr("Continue digitando — o conteúdo rola por dentro…")]),
        LblGrow([EnUs("Grow Y (2 → 8 rows, then scrolls)"),
                 PtBr("Cresce Y (2 → 8 linhas, depois rola)")]),
        PhGrow([EnUs("Start typing — textarea grows…"),     PtBr("Comece a digitar — a área cresce…")]),
    }
}

// ── Section: Media / Navigation ────────────────────────────────────────────

::i18n::traductions! {
    pub enum CarouselSec {
        Subtitle([EnUs("A carousel with motion and swipe capabilities."),
                  PtBr("Um carrossel com movimento e deslizamento.")]),
        HItems([EnUs("Items"),                              PtBr("Itens")]),
        Slide([EnUs("Slide {n}"),                           PtBr("Slide {n}")]),
    }
}

::i18n::traductions! {
    pub enum ChartSec {
        Subtitle([EnUs("Bar and line charts for data visualization."),
                  PtBr("Gráficos de barra e linha para visualização de dados.")]),
        HBar([EnUs("Bar chart – Desktop vs Mobile"),        PtBr("Gráfico de barras – Desktop vs Móvel")]),
        HLine([EnUs("Line chart"),                          PtBr("Gráfico de linha")]),
        Desktop([EnUs("Desktop"),                           PtBr("Desktop")]),
        Mobile([EnUs("Mobile"),                             PtBr("Móvel")]),
    }
}

::i18n::traductions! {
    pub enum CommandSec {
        Subtitle([EnUs("Fast, composable, keyboard-first command menu."),
                  PtBr("Menu de comandos rápido, composto e priorizando teclado.")]),
        HPalette([EnUs("Command palette"),                  PtBr("Paleta de comandos")]),
        HPaletteDesc([EnUs("Press the button or ⌘K to open."),
                      PtBr("Pressione o botão ou ⌘K para abrir.")]),
        Open([EnUs("Open Command Palette"),                 PtBr("Abrir paleta de comandos")]),
        GrpSuggestions([EnUs("Suggestions"),                PtBr("Sugestões")]),
        GrpSettings([EnUs("Settings"),                      PtBr("Configurações")]),
        ItemCalendar([EnUs("Calendar"),                     PtBr("Calendário")]),
        ItemEmoji([EnUs("Search Emoji"),                    PtBr("Buscar emoji")]),
        ItemCalculator([EnUs("Calculator"),                 PtBr("Calculadora")]),
        ItemProfile([EnUs("Profile"),                       PtBr("Perfil")]),
        ItemBilling([EnUs("Billing"),                       PtBr("Cobrança")]),
        ItemSettings([EnUs("Settings"),                     PtBr("Configurações")]),
        Placeholder([EnUs("Type a command or search…"),     PtBr("Digite um comando ou pesquise…")]),
    }
}

::i18n::traductions! {
    pub enum ResizableSec {
        Subtitle([EnUs("Accessible resizable panel groups and layouts."),
                  PtBr("Grupos de painéis redimensionáveis e layouts acessíveis.")]),
        One([EnUs("One"),                                   PtBr("Um")]),
        Two([EnUs("Two"),                                   PtBr("Dois")]),
        Three([EnUs("Three"),                               PtBr("Três")]),
    }
}

::i18n::traductions! {
    pub enum AccordionSec {
        Subtitle([EnUs("A vertically stacked set of interactive headings that each reveal a section of content."),
                  PtBr("Cabeçalhos interativos empilhados verticalmente que revelam uma seção de conteúdo.")]),
        HFaq([EnUs("FAQ"),                                  PtBr("FAQ")]),
        Q1([EnUs("What is egui-shadcn?"),                   PtBr("O que é o egui-shadcn?")]),
        A1([EnUs("A Shadcn/ui implementation for the egui Rust GUI library."),
            PtBr("Uma implementação do Shadcn/ui para a biblioteca de GUI egui em Rust.")]),
        Q2([EnUs("Does it work on WASM?"),                  PtBr("Funciona em WASM?")]),
        A2([EnUs("Yes! Both native and web targets are supported via eframe."),
            PtBr("Sim! Tanto nativo quanto web são suportados via eframe.")]),
        Q3([EnUs("Can I customise the theme?"),             PtBr("Posso personalizar o tema?")]),
        A3([EnUs("Use the palette icon in the toolbar to pick a primary color."),
            PtBr("Use o ícone de paleta na barra para escolher a cor primária.")]),
    }
}

::i18n::traductions! {
    pub enum BreadcrumbSec {
        Subtitle([EnUs("Displays the path to the current resource using a hierarchy of links."),
                  PtBr("Mostra o caminho até o recurso atual usando uma hierarquia de links.")]),
        HDefault([EnUs("Default"),                          PtBr("Padrão")]),
        HCustom([EnUs("Custom separator"),                  PtBr("Separador personalizado")]),
        Home([EnUs("Home"),                                 PtBr("Início")]),
        Components([EnUs("Components"),                     PtBr("Componentes")]),
        Breadcrumb([EnUs("Breadcrumb"),                     PtBr("Trilha")]),
        Docs([EnUs("Docs"),                                 PtBr("Docs")]),
        Nav([EnUs("Navigated to: {item}"),                  PtBr("Navegou para: {item}")]),
    }
}

::i18n::traductions! {
    pub enum MenubarSec {
        Subtitle([EnUs("A visually persistent menu common in desktop applications that provides quick access to a consistent set of commands."),
                  PtBr("Um menu visualmente persistente comum em aplicações desktop que oferece acesso rápido a um conjunto consistente de comandos.")]),
        HApp([EnUs("Application Menu"),                     PtBr("Menu do aplicativo")]),
        File([EnUs("File"),                                 PtBr("Arquivo")]),
        Edit([EnUs("Edit"),                                 PtBr("Editar")]),
        View([EnUs("View"),                                 PtBr("Visualizar")]),
        NewTab([EnUs("New Tab"),                            PtBr("Nova aba")]),
        NewWindow([EnUs("New Window"),                      PtBr("Nova janela")]),
        Share([EnUs("Share"),                               PtBr("Compartilhar")]),
        Print([EnUs("Print…"),                              PtBr("Imprimir…")]),
        Undo([EnUs("Undo"),                                 PtBr("Desfazer")]),
        Redo([EnUs("Redo"),                                 PtBr("Refazer")]),
        Cut([EnUs("Cut"),                                   PtBr("Recortar")]),
        Copy([EnUs("Copy"),                                 PtBr("Copiar")]),
        Paste([EnUs("Paste"),                               PtBr("Colar")]),
        ZoomIn([EnUs("Zoom In"),                            PtBr("Aumentar zoom")]),
        ZoomOut([EnUs("Zoom Out"),                          PtBr("Diminuir zoom")]),
    }
}

::i18n::traductions! {
    pub enum NavMenuSec {
        Subtitle([EnUs("A collection of links for navigating websites."),
                  PtBr("Conjunto de links para navegar em sites.")]),
        HHoriz([EnUs("Horizontal"),                         PtBr("Horizontal")]),
        HVert([EnUs("Vertical (sidebar)"),                  PtBr("Vertical (barra lateral)")]),
        Dashboard([EnUs("Dashboard"),                       PtBr("Painel")]),
        Projects([EnUs("Projects"),                         PtBr("Projetos")]),
        Team([EnUs("Team"),                                 PtBr("Equipe")]),
        Settings([EnUs("Settings"),                         PtBr("Configurações")]),
    }
}

::i18n::traductions! {
    pub enum PaginationSec {
        Subtitle([EnUs("Pagination with page navigation, next and previous links."),
                  PtBr("Paginação com navegação entre páginas, próximas e anteriores.")]),
        HBasic([EnUs("Basic"),                              PtBr("Básico")]),
        HMany([EnUs("Many pages"),                          PtBr("Muitas páginas")]),
        Page10([EnUs("Page {n} of 10"),                     PtBr("Página {n} de 10")]),
        Page20([EnUs("Page {n} of 20"),                     PtBr("Página {n} de 20")]),
    }
}

::i18n::traductions! {
    pub enum TabsSec {
        Subtitle([EnUs("A set of layered sections of content — known as tab panels — that are displayed one at a time."),
                  PtBr("Conjunto de seções de conteúdo em camadas — chamadas de painéis de abas — exibidas uma de cada vez.")]),
        HAccount([EnUs("Account"),                          PtBr("Conta")]),
        Account([EnUs("Account"),                           PtBr("Conta")]),
        Password([EnUs("Password"),                         PtBr("Senha")]),
        Notifications([EnUs("Notifications"),               PtBr("Notificações")]),
        BodyAccount([EnUs("Account settings go here."),     PtBr("As configurações da conta vão aqui.")]),
        BodyPassword([EnUs("Change your password."),        PtBr("Altere sua senha.")]),
        BodyNotifications([EnUs("Notification preferences."), PtBr("Preferências de notificação.")]),
    }
}

// ── Section: Overlays ──────────────────────────────────────────────────────

::i18n::traductions! {
    pub enum ContextMenuSec {
        Subtitle([EnUs("Displays a menu to the user triggered by a right-click."),
                  PtBr("Mostra um menu ativado por clique com o botão direito.")]),
        HArea([EnUs("Right-click area"),                    PtBr("Área de clique direito")]),
        HAreaDesc([EnUs("Right-click anywhere in the box below."),
                   PtBr("Clique com o botão direito em qualquer ponto da caixa abaixo.")]),
        AreaHint([EnUs("Right-click here"),                 PtBr("Clique direito aqui")]),
        Back([EnUs("Back"),                                 PtBr("Voltar")]),
        Forward([EnUs("Forward"),                           PtBr("Avançar")]),
        Reload([EnUs("Reload"),                             PtBr("Recarregar")]),
        SaveAs([EnUs("Save as…"),                           PtBr("Salvar como…")]),
        Clicked([EnUs("Clicked: {item}"),                   PtBr("Clicado: {item}")]),
    }
}

::i18n::traductions! {
    pub enum DrawerSec {
        Subtitle([EnUs("A panel that slides in from the bottom edge of the screen."),
                  PtBr("Um painel que desliza a partir da borda inferior da tela.")]),
        HBottom([EnUs("Bottom drawer"),                     PtBr("Gaveta inferior")]),
        Open([EnUs("Open Drawer"),                          PtBr("Abrir gaveta")]),
        Title([EnUs("Move Goal"),                           PtBr("Mover meta")]),
        Body([EnUs("Set your daily activity goal."),        PtBr("Defina sua meta diária de atividade.")]),
        Submit([EnUs("Submit"),                             PtBr("Enviar")]),
    }
}

::i18n::traductions! {
    pub enum DropdownSec {
        Subtitle([EnUs("Displays a menu to the user — such as a set of actions or functions — triggered by a button."),
                  PtBr("Mostra um menu ao usuário — como um conjunto de ações ou funções — ativado por um botão.")]),
        HAccount([EnUs("Account Menu"),                     PtBr("Menu da conta")]),
        MyAccount([EnUs("My Account"),                      PtBr("Minha conta")]),
        Profile([EnUs("Profile"),                           PtBr("Perfil")]),
        Billing([EnUs("Billing"),                           PtBr("Cobrança")]),
        Settings([EnUs("Settings"),                         PtBr("Configurações")]),
        Logout([EnUs("Log out"),                            PtBr("Sair")]),
        Clicked([EnUs("Clicked: {item}"),                   PtBr("Clicado: {item}")]),
    }
}

::i18n::traductions! {
    pub enum HoverCardSec {
        Subtitle([EnUs("For sighted users to preview content available behind a link."),
                  PtBr("Para usuários enxergantes pré-visualizarem o conteúdo de um link.")]),
        HProfile([EnUs("Profile link"),                     PtBr("Link de perfil")]),
        HProfileDesc([EnUs("Hover over the username below."),
                      PtBr("Passe o cursor sobre o nome de usuário abaixo.")]),
        Body([EnUs("Shadcn/ui components for Rust egui. Dark/light, 50+ components."),
              PtBr("Componentes Shadcn/ui para egui em Rust. Claro/escuro, 50+ componentes.")]),
    }
}

::i18n::traductions! {
    pub enum PopoverSec {
        Subtitle([EnUs("Displays rich content in a portal, triggered by a button."),
                  PtBr("Mostra conteúdo rico em um portal, ativado por um botão.")]),
        HDimensions([EnUs("Dimensions"),                    PtBr("Dimensões")]),
        Open([EnUs("Open"),                                 PtBr("Abrir")]),
        Title([EnUs("Dimensions"),                          PtBr("Dimensões")]),
        Desc([EnUs("Set the dimensions for the layer."),    PtBr("Defina as dimensões da camada.")]),
        Width([EnUs("Width"),                               PtBr("Largura")]),
        MaxWidth([EnUs("Max. width"),                       PtBr("Largura máxima")]),
    }
}

::i18n::traductions! {
    pub enum SheetSec {
        Subtitle([EnUs("Extends the Dialog component to display content that complements the main content of the screen."),
                  PtBr("Estende o componente de diálogo para exibir conteúdo que complementa o conteúdo principal da tela.")]),
        HRight([EnUs("Right sheet"),                        PtBr("Painel à direita")]),
        Open([EnUs("Open Sheet"),                           PtBr("Abrir painel")]),
        Title([EnUs("Edit Profile"),                        PtBr("Editar perfil")]),
        Body([EnUs("Make changes to your profile here."),   PtBr("Faça alterações no seu perfil aqui.")]),
        Save([EnUs("Save changes"),                         PtBr("Salvar alterações")]),
    }
}

::i18n::traductions! {
    pub enum TooltipSec {
        Subtitle([EnUs("A popup that displays information related to an element when the element receives keyboard focus or the mouse hovers over it."),
                  PtBr("Janela que exibe informações sobre um elemento quando ele recebe foco ou o cursor passa sobre ele.")]),
        HBasic([EnUs("Basic"),                              PtBr("Básico")]),
        HDescriptive([EnUs("Descriptive"),                  PtBr("Descritivo")]),
        Helpful([EnUs("This is a helpful tooltip"),         PtBr("Esta é uma dica útil")]),
        HoverMe([EnUs("Hover me"),                          PtBr("Passe aqui")]),
        OpensPalette([EnUs("Opens the command palette (⌘K)"),
                      PtBr("Abre a paleta de comandos (⌘K)")]),
        CmdPalette([EnUs("Command Palette"),                PtBr("Paleta de comandos")]),
    }
}

// ── Section: Overview / Typography ─────────────────────────────────────────

::i18n::traductions! {
    pub enum OverviewSec {
        Subtitle([EnUs("Shadcn/ui components for Rust egui."),
                  PtBr("Componentes Shadcn/ui para egui em Rust.")]),
        HWelcome([EnUs("Welcome"),                          PtBr("Bem-vindo")]),
        HWelcomeDesc([EnUs("A beautiful, accessible component library."),
                      PtBr("Uma biblioteca de componentes bonita e acessível.")]),
        Body([EnUs("Use the sidebar to explore all components. Toggle dark mode or pick a primary color with the toolbar icons above."),
              PtBr("Use a barra lateral para explorar todos os componentes. Alterne o modo escuro ou escolha uma cor primária pelos ícones acima.")]),
        Components([EnUs("Components"),                     PtBr("Componentes")]),
        Themes([EnUs("Themes"),                             PtBr("Temas")]),
        ColorPresets([EnUs("Color presets"),                PtBr("Predefinições de cor")]),
        Customizable([EnUs("Customizable"),                 PtBr("Personalizável")]),
    }
}

::i18n::traductions! {
    pub enum TypographySec {
        Subtitle([EnUs("Text styles and type scale."),      PtBr("Estilos de texto e escala tipográfica.")]),
        H1([EnUs("Heading 1"),                              PtBr("Título 1")]),
        H2([EnUs("Heading 2"),                              PtBr("Título 2")]),
        H3([EnUs("Heading 3"),                              PtBr("Título 3")]),
        H4([EnUs("Heading 4"),                              PtBr("Título 4")]),
        Lead([EnUs("Lead paragraph — larger, muted text for introductions."),
              PtBr("Parágrafo destaque — texto maior e suave para introduções.")]),
        Body([EnUs("Body text — the default paragraph size."),
              PtBr("Texto do corpo — tamanho padrão de parágrafo.")]),
        Muted([EnUs("Muted text — de-emphasised, helper copy."),
               PtBr("Texto suave — texto de apoio com menos ênfase.")]),
        Small([EnUs("Small text — footnotes and captions."),
               PtBr("Texto pequeno — notas de rodapé e legendas.")]),
        InlinePre([EnUs("Inline "),                         PtBr("Em linha ")]),
        InlinePost([EnUs(" inside a sentence."),            PtBr(" dentro de uma frase.")]),
    }
}
