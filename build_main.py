# Script to rebuild main.rs with all missing functions
import re

with open('c:/Rust/galactic_explorer/src/main.rs', 'r', encoding='utf-8') as f:
    content = f.read()

# Check if the file ends with just ".run();\n}" - meaning it's truncated
if '.run();\n}' in content and 'fn setup_loading_screen' not in content:
    print("File is truncated at main() - adding all functions...")
    
    # Remove the truncated end
    content = content[:content.rindex('.run();\n}')]
    
    # Now add all missing functions
    content += """
        .run();
}

// ── TRANSLATION SYSTEM ──
fn apply_language_translations(
    settings: Res<AppSettings>,
    translations: Res<Translations>,
    mut param_set: ParamSet<(
        Query<(&mut Text, &TranslatableText)>,
        Query<(&mut Text, &ToolbarButtonText)>,
        Query<&mut Text, (With<HudTitleText>, Without<TranslatableText>, Without<ToolbarButtonText>)>,
    )>,
) {
    let lang = &settings.language;
    if settings.is_changed() {
        for (mut text, tt) in param_set.p0().iter_mut() {
            text.sections[0].value = translations.get(lang, &tt.key);
        }
        for (mut text, tbt) in param_set.p1().iter_mut() {
            text.sections[0].value = translations.get(lang, &tbt.key);
        }
        for mut text in param_set.p2().iter_mut() {
            text.sections[0].value = translations.get(lang, "galactic_explorer_title");
        }
    }
}

// ── TOOLBAR ──
fn handle_toolbar_buttons(
    interaction: Query<(&Interaction, &Children), (Changed<Interaction>, With<Button>)>,
    button_texts: Query<&Text>,
    mut ns: ResMut<NextState<AppState>>,
    mut settings: ResMut<AppSettings>,
    mut combat: ResMut<CombatState>,
    selected: Res<SelectedPlanet>,
) {
    for (interact, children) in interaction.iter() {
        if *interact == Interaction::Pressed {
            for &child in children.iter() {
                if let Ok(text) = button_texts.get(child) {
                    if let Some(section) = text.sections.first() {
                        let clicked = &section.value;
                        if clicked.contains("SETTINGS") || clicked.contains("AJUSTES") || clicked.contains("PARAMÈTRES") { ns.set(AppState::Settings); }
                        else if clicked.contains("GEO MAP") || clicked.contains("MAPA GEO") || clicked.contains("CARTE GÉO") || clicked.contains("MAP") { ns.set(AppState::GeoMap); }
                        else if clicked.contains("COMBAT") { 
                            combat.active = true; 
                            combat.wave = 1; 
                            combat.spawn_timer = 0.0; 
                            combat.enemy_count = 0; 
                            combat.has_won = false;
                            combat.has_lost = false;
                            combat.fire_timer = 0.0;
                            ns.set(AppState::Combat); 
                        }
                        else if clicked.contains("CHALLENGES") || clicked.contains("DESAFÍOS") || clicked.contains("DÉFIS") { ns.set(AppState::Challenges); }
                        else if clicked.contains("LEARN") || clicked.contains("APRENDER") || clicked.contains("APPRENDRE") { ns.set(AppState::Educational); }
                        else if clicked.contains("ORBITS") || clicked.contains("ÓRBITAS") || clicked.contains("ORBITES") { settings.show_planet_orbits = !settings.show_planet_orbits; }
                        else if clicked.contains("NIGHT") || clicked.contains("NOCHE") || clicked.contains("NUIT") { settings.night_sky_enabled = !settings.night_sky_enabled; }
                        else if clicked.contains("PLANETS") || clicked.contains("PLANETAS") || clicked.contains("PLANÈTES") { if !selected.name.is_empty() { ns.set(AppState::PlanetInfo); } }
                    }
                }
            }
        }
    }
}

// ── BUTTON CLICK DETECTION ──
fn get_clicked_button<F: QueryFilter>(
    interaction: Query<(&Interaction, &Children), (Changed<Interaction>, With<Button>)>,
    text_query: Query<&Text, F>,
) -> Option<String> {
    for (interaction, children) in interaction.iter() {
        if *interaction == Interaction::Pressed {
            for &child in children.iter() {
                if let Ok(text) = text_query.get(child) {
                    if let Some(section) = text.sections.first() {
                        return Some(section.value.clone());
                    }
                }
            }
        }
    }
    None
}

// ── LOADING SCREEN ──
fn setup_loading_screen(mut commands: Commands) {
    commands.spawn((
        NodeBundle {
            style: Style { width: Val::Percent(100.0), height: Val::Percent(100.0), flex_direction: FlexDirection::Column, justify_content: JustifyContent::Center, align_items: AlignItems::Center, ..default() },
            background_color: BackgroundColor(Color::rgba(0.0, 0.02, 0.08, 1.0)),
            ..default()
        },
        LoadingScreen,
    )).with_children(|p| {
        p.spawn(TextBundle::from_section("GALACTIC EXPLORER", TextStyle { font_size: 48.0, color: Color::CYAN, ..default() }));
        p.spawn(TextBundle::from_section("DISCOVERY PRO", TextStyle { font_size: 24.0, color: Color::rgb(0.3, 0.7, 1.0), ..default() }));
        p.spawn(NodeBundle {
            style: Style { width: Val::Px(400.0), height: Val::Px(20.0), margin: UiRect::top(Val::Px(40.0)), border: UiRect::all(Val::Px(2.0)), ..default() },
            border_color: BorderColor(Color::rgb(0.2, 0.6, 0.8)),
            background_color: BackgroundColor(Color::rgba(0.01, 0.03, 0.06, 0.9)),
            ..default()
        }).with_children(|b| { b.spawn((NodeBundle { style: Style { width: Val::Percent(0.0), height: Val::Percent(100.0), ..default() }, background_color: BackgroundColor(Color::rgb(0.2, 0.8, 0.7)), ..default() }, LoadingBar)); });
        p.spawn((TextBundle::from_section("Initializing systems...", TextStyle { font_size: 16.0, color: Color::rgb(0.5, 0.7, 0.9), ..default() }), LoadingText));
    });
}

fn load_loading(
    time: Res<Time>, mut loading: ResMut<LoadingProgress>, mut ns: ResMut<NextState<AppState>>,
    mut lb: Query<&mut Style, With<LoadingBar>>, mut lt: Query<&mut Text, With<LoadingText>>,
    mut ls: Query<&mut Visibility, With<LoadingScreen>>,
) {
    loading.progress += time.delta_seconds() * 0.4;
    let phase = match loading.progress {
        p if p < 0.25 => "Loading solar system data...",
        p if p < 0.5 => "Generating planetary textures...",
        p if p < 0.75 => "Calibrating navigation systems...",
        p if p < 1.0 => "Preparing exploration mode...",
        _ => "Ready for launch!",
    };
    if let Ok(mut t) = lt.get_single_mut() { t.sections[0].value = phase.into(); }
    if let Ok(mut s) = lb.get_single_mut() { s.width = Val::Percent((loading.progress * 100.0).min(100.0)); }
    if loading.progress >= 1.0 {
        if let Ok(mut v) = ls.get_single_mut() { *v = Visibility::Hidden; }
        ns.set(AppState::Exploration);
    }
}

// ── HUD SETUP ──
fn setup_advanced_hud(mut commands: Commands, _asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle { camera: Camera { order: 1, ..default() }, ..default() });
    commands.spawn((
        NodeBundle {
            style: Style { width: Val::Percent(100.0), height: Val::Percent(100.0), flex_direction: FlexDirection::Column, justify_content: JustifyContent::SpaceBetween, ..default() },
            focus_policy: bevy::ui::FocusPolicy::Pass,
            ..default()
        },
        AdvancedHud,
    )).with_children(|c| {
        // TOP BAR
        c.spawn(NodeBundle {
            style: Style { width: Val::Percent(100.0), height: Val::Px(70.0), padding: UiRect::all(Val::Px(12.0)), justify_content: JustifyContent::SpaceBetween, align_items: AlignItems::Center, ..default() },
            background_color: BackgroundColor(Color::rgba(0.0, 0.03, 0.08, 0.85)),
            ..default()
        }).with_children(|t| {
            t.spawn((TextBundle::from_section("GALACTIC EXPLORER", TextStyle { font_size: 20.0, color: Color::CYAN, ..default() }), HudTitleText));
            t.spawn((TextBundle::from_section("SEARCH: [  ]", TextStyle { font_size: 18.0, color: Color::rgb(0.6, 0.8, 1.0), ..default() }), SearchInput));
            t.spawn((TextBundle::from_section("JUL 13, 2026  12:00 UTC", TextStyle { font_size: 16.0, color: Color::WHITE, ..default() }), HudClock));
        });

        // PLANET INFO PANEL
        c.spawn((NodeBundle {
            style: Style { position_type: PositionType::Absolute, left: Val::Percent(10.0), top: Val::Percent(15.0), width: Val::Px(420.0), padding: UiRect::all(Val::Px(20.0)), display: Display::None, flex_direction: FlexDirection::Column, row_gap: Val::Px(8.0), border: UiRect::all(Val::Px(2.0)), ..default() },
            border_color: BorderColor(Color::rgb(0.2, 0.6, 0.8)),
            background_color: BackgroundColor(Color::rgba(0.02, 0.06, 0.15, 0.92)),
            ..default()
        }, PlanetInfoPanel)).with_children(|i| {
            i.spawn(NodeBundle {
                style: Style { width: Val::Percent(100.0), justify_content: JustifyContent::SpaceBetween, align_items: AlignItems::Center, ..default() },
                background_color: BackgroundColor(Color::NONE),
                ..default()
            }).with_children(|h| {
                h.spawn(TextBundle::from_section("EARTH", TextStyle { font_size: 28.0, color: Color::rgb(0.3, 0.7, 1.0), ..default() }));
                h.spawn(ButtonBundle {
                    style: Style { width: Val::Px(36.0), height: Val::Px(36.0), justify_content: JustifyContent::Center, align_items: AlignItems::Center, ..default() },
                    background_color: BackgroundColor(Color::rgba(0.8, 0.2, 0.2, 0.8)),
                    ..default()
                }).with_children(|b| { b.spawn(TextBundle::from_section("X", TextStyle { font_size: 20.0, color: Color::WHITE, ..default() })); });
            });
            i.spawn(TextBundle::from_section("The Blue Marble - our home.", TextStyle { font_size: 16.0, color: Color::rgb(0.7, 0.8, 0.9), ..default() }));
            i.spawn(TextBundle::from_section("Mass: 1.0 M  |  Radius: 6,371 km  |  Day: 24h", TextStyle { font_size: 14.0, color: Color::rgb(0.5, 0.7, 0.5), ..default() }));
            i.spawn(TextBundle::from_section("Moons: 1  |  Surface: -89 to 57 C", TextStyle { font_size: 14.0, color: Color::rgb(0.7, 0.5, 0.5), ..default() }));
            btn(i, "LEARN MORE", Color::rgb(0.1, 0.2, 0.4));
            btn(i, "TAKE QUIZ", Color::rgb(0.2, 0.15, 0.4));
            btn(i, "VIEW GEO MAP", Color::rgb(0.1, 0.3, 0.2));
            btn(i, "CLOSE", Color::rgb(0.3, 0.1, 0.1));
        });

        // EDUCATIONAL PANEL
        c.spawn((NodeBundle {
            style: Style { position_type: PositionType::Absolute, left: Val::Percent(2.0), top: Val::Percent(3.0), width: Val::Px(950.0), max_height: Val::Px(820.0), padding: UiRect::all(Val::Px(16.0)), display: Display::None, flex_direction: FlexDirection::Column, row_gap: Val::Px(6.0), border: UiRect::all(Val::Px(2.0)), overflow: Overflow::clip(), ..default() },
            border_color: BorderColor(Color::rgb(0.3, 0.8, 0.6)),
            background_color: BackgroundColor(Color::rgba(0.02, 0.04, 0.12, 0.96)),
            ..default()
        }, EducationalPanel)).with_children(|e| {
            e.spawn(NodeBundle {
                style: Style { width: Val::Percent(100.0), justify_content: JustifyContent::SpaceBetween, align_items: AlignItems::Center, ..default() },
                background_color: BackgroundColor(Color::NONE),
                ..default()
            }).with_children(|h| {
                h.spawn(TextBundle::from_section("📚 COMPREHENSIVE PLANET GUIDE", TextStyle { font_size: 22.0, color: Color::rgb(0.3, 0.8, 0.6), ..default() }));
                h.spawn(ButtonBundle {
                    style: Style { width: Val::Px(36.0), height: Val::Px(36.0), justify_content: JustifyContent::Center, align_items: AlignItems::Center, ..default() },
                    background_color: BackgroundColor(Color::rgba(0.8, 0.2, 0.2, 0.8)),
                    ..default()
                }).with_children(|b| { b.spawn(TextBundle::from_section("X", TextStyle { font_size: 20.0, color: Color::WHITE, ..default() })); });
            });

            e.spawn(TextBundle::from_section("━━━ SELECT A PLANET ━━━", TextStyle { font_size: 14.0, color: Color::rgb(0.3, 0.8, 0.6), ..default() }));
            
            let planets_grid = vec![
                ("☿️", "Mercury", Color::rgb(0.6, 0.6, 0.65)),
                ("♀️", "Venus", Color::rgb(0.85, 0.6, 0.3)),
                ("🌍", "Earth", Color::rgb(0.2, 0.5, 0.8)),
                ("♂️", "Mars", Color::rgb(0.7, 0.3, 0.2)),
                ("♃", "Jupiter", Color::rgb(0.75, 0.6, 0.4)),
                ("♄", "Saturn", Color::rgb(0.7, 0.65, 0.45)),
                ("♅", "Uranus", Color::rgb(0.5, 0.7, 0.75)),
                ("♆", "Neptune", Color::rgb(0.2, 0.3, 0.7)),
            ];
            
            e.spawn(NodeBundle {
                style: Style { width: Val::Percent(100.0), flex_direction: FlexDirection::Row, flex_wrap: FlexWrap::Wrap, justify_content: JustifyContent::SpaceEvenly, row_gap: Val::Px(4.0), column_gap: Val::Px(4.0), ..default() },
                background_color: BackgroundColor(Color::NONE),
                ..default()
            }).with_children(|grid| {
                for (emoji, name, accent_color) in &planets_grid {
                    grid.spawn((ButtonBundle {
                        style: Style {
                            width: Val::Px(108.0),
                            height: Val::Px(50.0),
                            justify_content: JustifyContent::Center, align_items: AlignItems::Center,
                            flex_direction: FlexDirection::Column,
                            padding: UiRect::all(Val::Px(4.0)),
                            border: UiRect::all(Val::Px(1.0)),
                            ..default()
                        },
                        border_color: BorderColor(Color::rgba(0.3, 0.8, 0.6, 0.4)),
                        background_color: BackgroundColor(Color::rgba(0.05, 0.08, 0.15, 0.7)),
                        ..default()
                    }, EduPlanetSelectButton { planet_name: name.to_string() })).with_children(|b| {
                        b.spawn(TextBundle::from_section(format!("{} {}", emoji, name), TextStyle { font_size: 13.0, color: *accent_color, ..default() }));
                    });
                }
            });
            
            e.spawn((TextBundle::from_section("🌍 Earth", TextStyle { font_size: 20.0, color: Color::rgb(0.5, 0.7, 1.0), ..default() }), EduPlanetName));
            e.spawn((TextBundle::from_section("The Blue Marble - our home, the only known planet with life.", TextStyle { font_size: 14.0, color: Color::rgb(0.7, 0.8, 0.9), ..default() }), EduDescription));
            
            e.spawn(TextBundle::from_section("━━━ KEY FACTS ━━━", TextStyle { font_size: 14.0, color: Color::rgb(0.3, 0.8, 0.6), ..default() }));
            e.spawn(NodeBundle { style: Style { width: Val::Percent(100.0), flex_direction: FlexDirection::Column, row_gap: Val::Px(3.0), padding: UiRect::all(Val::Px(6.0)), ..default() }, background_color: BackgroundColor(Color::rgba(0.05, 0.08, 0.15, 0.6)), ..default() }).with_children(|f| {
                for i in 0..5 { f.spawn((TextBundle::from_section(format!("• Fact {}: Loading...", i + 1), TextStyle { font_size: 12.0, color: Color::rgb(0.7, 0.8, 0.9), ..default() }), EduFactText(i))); }
            });
            
            e.spawn(TextBundle::from_section("━━━ PHYSICAL CHARACTERISTICS ━━━", TextStyle { font_size: 14.0, color: Color::rgb(0.3, 0.8, 0.6), ..default() }));
            e.spawn(NodeBundle { style: Style { width: Val::Percent(100.0), flex_direction: FlexDirection::Column, row_gap: Val::Px(3.0), padding: UiRect::all(Val::Px(6.0)), ..default() }, background_color: BackgroundColor(Color::rgba(0.05, 0.08, 0.15, 0.6)), ..default() }).with_children(|f| {
                for i in 0..6 { f.spawn((TextBundle::from_section(format!("• Physical {}: Loading...", i + 1), TextStyle { font_size: 12.0, color: Color::rgb(0.7, 0.8, 0.9), ..default() }), EduPhysicalText(i))); }
            });
            
            e.spawn(TextBundle::from_section("━━━ FUN FACTS ━━━", TextStyle { font_size: 14.0, color: Color::rgb(0.3, 0.8, 0.6), ..default() }));
            e.spawn(NodeBundle { style: Style { width: Val::Percent(100.0), flex_direction: FlexDirection::Column, row_gap: Val::Px(3.0), padding: UiRect::all(Val::Px(6.0)), ..default() }, background_color: BackgroundColor(Color::rgba(0.05, 0.08, 0.15, 0.6)), ..default() }).with_children(|f| {
                for i in 0..3 { f.spawn((TextBundle::from_section(format!("• Fun {}: Loading...", i + 1), TextStyle { font_size: 12.0, color: Color::rgb(0.7, 0.8, 0.9), ..default() }), EduFunFactText(i))); }
            });
            
            e.spawn(TextBundle::from_section("━━━ HISTORICAL MISSIONS ━━━", TextStyle { font_size: 14.0, color: Color::rgb(0.3, 0.8, 0.6), ..default() }));
            e.spawn(NodeBundle { style: Style { width: Val::Percent(100.0), flex_direction: FlexDirection::Column, row_gap: Val::Px(3.0), padding: UiRect::all(Val::Px(6.0)), ..default() }, background_color: BackgroundColor(Color::rgba(0.05, 0.08, 0.15, 0.6)), ..default() }).with_children(|f| {
                for i in 0..3 { f.spawn((TextBundle::from_section(format!("🚀 Mission {}: Loading...", i + 1), TextStyle { font_size: 12.0, color: Color::rgb(0.7, 0.8, 0.9), ..default() }), EduMissionText(i))); }
            });
            
            btn(e, "📝 TAKE QUIZ", Color::rgb(0.2, 0.15, 0.4));
            btn(e, "🗺️ VIEW GEO MAP", Color::rgb(0.1, 0.3, 0.2));
            btn(e, "CLOSE", Color::rgb(0.3, 0.1, 0.1));
        });

        // QUIZ PANEL
        c.spawn((NodeBundle {
            style: Style { position_type: PositionType::Absolute, left: Val::Percent(5.0), top: Val::Percent(5.0), width: Val::Px(700.0), max_height: Val::Px(750.0), padding: UiRect::all(Val::Px(20.0)), display: Display::None, flex_direction: FlexDirection::Column, row_gap: Val::Px(8.0), border: UiRect::all(Val::Px(2.0)), overflow: Overflow::clip(), ..default() },
            border_color: BorderColor(Color::rgb(0.8, 0.6, 0.3)),
            background_color: BackgroundColor(Color::rgba(0.02, 0.03, 0.1, 0.97)),
            ..default()
        }, QuizPanel)).with_children(|q| {
            q.spawn(NodeBundle {
                style: Style { width: Val::Percent(100.0), justify_content: JustifyContent::SpaceBetween, align_items: AlignItems::Center, ..default() },
                background_color: BackgroundColor(Color::NONE),
                ..default()
            }).with_children(|h| {
                h.spawn(TextBundle::from_section("PLANET QUIZ", TextStyle { font_size: 24.0, color: Color::rgb(0.8, 0.6, 0.3), ..default() }));
                h.spawn(ButtonBundle {
                    style: Style { width: Val::Px(36.0), height: Val::Px(36.0), justify_content: JustifyContent::Center, align_items: AlignItems::Center, ..default() },
                    background_color: BackgroundColor(Color::rgba(0.8, 0.2, 0.2, 0.8)),
                    ..default()
                }).with_children(|b| { b.spawn(TextBundle::from_section("X", TextStyle { font_size: 20.0, color: Color::WHITE, ..default() })); });
            });
            q.spawn((TextBundle::from_section("Score: 0/0", TextStyle { font_size: 16.0, color: Color::rgb(0.5, 0.8, 0.5), ..default() }), QuizScoreText));
            q.spawn((TextBundle::from_section("Select a planet and press TAKE QUIZ to begin!", TextStyle { font_size: 18.0, color: Color::rgb(0.8, 0.8, 0.9), ..default() }), QuizQuestionText));
            q.spawn(NodeBundle { style: Style { width: Val::Percent(100.0), flex_direction: FlexDirection::Column, row_gap: Val::Px(6.0), ..default() }, background_color: BackgroundColor(Color::NONE), ..default() }).with_children(|o| {
                for i in 0..4 {
                    o.spawn(ButtonBundle {
                        style: Style { width: Val::Percent(100.0), height: Val::Px(40.0), justify_content: JustifyContent::Center, align_items: AlignItems::Center, padding: UiRect::all(Val::Px(8.0)), ..default() },
                        background_color: BackgroundColor(Color::rgba(0.1, 0.15, 0.25, 0.8)),
                       ..default()
                    }).with_children(|b| { b.spawn((TextBundle::from_section("Loading...", TextStyle { font_size: 15.0, color: Color::WHITE, ..default() }), QuizOptionText(i))); });
                }
            });
            q.spawn((TextBundle::from_section("", TextStyle { font_size: 14.0, color: Color::rgb(0.6, 0.8, 0.6), ..default() }), QuizExplanationText));
            q.spawn((TextBundle::from_section("", TextStyle { font_size: 18.0, color: Color::rgb(0.8, 0.8, 0.3), ..default() }), QuizResultText));
            q.spawn(NodeBundle { style: Style { width: Val::Percent(100.0), justify_content: JustifyContent::SpaceBetween, ..default() }, background_color: BackgroundColor(Color::NONE), ..default() }).with_children(|b| { btn(b, "NEXT QUESTION ▶", Color::rgb(0.1, 0.3, 0.2)); btn(b, "✕ CLOSE QUIZ", Color::rgb(0.3, 0.1, 0.1)); });
        });

        // GEO MAP PANEL
        c.spawn((NodeBundle {
            style: Style { position_type: PositionType::Absolute, left: Val::Percent(10.0), top: Val::Percent(10.0), width: Val::Px(600.0), max_height: Val::Px(700.0), padding: UiRect::all(Val::Px(16.0)), display: Display::None, flex_direction: FlexDirection::Column, overflow: Overflow::clip(), row_gap: Val::Px(6.0), border: UiRect::all(Val::Px(2.0)), ..default() },
            border_color: BorderColor(Color::rgb(0.3, 0.8, 0.6)),
            background_color: BackgroundColor(Color::rgba(0.01, 0.04, 0.1, 0.95)),
            ..default()
        }, GeoMapPanel)).with_children(|g| {
            g.spawn(NodeBundle {
                style: Style { width: Val::Percent(100.0), justify_content: JustifyContent::SpaceBetween, align_items: AlignItems::Center, ..default() },
                background_color: BackgroundColor(Color::NONE),
                ..default()
            }).with_children(|h| {
                h.spawn(TextBundle::from_section("EARTH - CONTINENTS & COUNTRIES", TextStyle { font_size: 22.0, color: Color::rgb(0.3, 0.8, 0.6), ..default() }));
                h.spawn(ButtonBundle {
                    style: Style { width: Val::Px(36.0), height: Val::Px(36.0), justify_content: JustifyContent::Center, align_items: AlignItems::Center, ..default() },
                    background_color: BackgroundColor(Color::rgba(0.8, 0.2, 0.2, 0.8)),
                    ..default()
                }).with_children(|b| { b.spawn(TextBundle::from_section("X", TextStyle { font_size: 20.0, color: Color::WHITE, ..default() })); });
            });
            g.spawn(TextBundle::from_section("Total Surface: 510.1 million km2  |  Land: 148.9 million km2", TextStyle { font_size: 14.0, color: Color::rgb(0.6, 0.7, 0.8), ..default() }));
            let conts: Vec<(&str, &str, &str, &str, &str)> = vec![
                ("Africa", "30.37M km2", "1.4B", "54 countries", "Cradle of humankind, Sahara desert."),
                ("Antarctica", "14.2M km2", "~5K", "Research", "Icy desert, 90% of world's ice."),
                ("Asia", "44.58M km2", "4.7B", "49 countries", "Largest continent, Himalayas."),
                ("Europe", "10.18M km2", "745M", "44 countries", "Renaissance birthplace, Alps."),
                ("North America", "24.71M km2", "592M", "23 countries", "Rockies, Great Lakes."),
                ("South America", "17.84M km2", "430M", "12 countries", "Amazon rainforest, Andes."),
                ("Oceania", "8.6M km2", "43M", "14 countries", "Great Barrier Reef."),
            ];
            for (e, a, p, c, d) in conts {
                g.spawn(NodeBundle { style: Style { width: Val::Percent(100.0), padding: UiRect::all(Val::Px(10.0)), flex_direction: FlexDirection::Column, margin: UiRect::bottom(Val::Px(4.0)), ..default() }, background_color: BackgroundColor(Color::rgba(0.05, 0.1, 0.2, 0.6)), ..default() }).with_children(|n| {
                    n.spawn(TextBundle::from_section(format!("{}  |  Area: {}  |  Pop: {}  |  {}", e, a, p, c), TextStyle { font_size: 15.0, color: Color::rgb(0.7, 0.9, 1.0), ..default() }));
                    n.spawn(TextBundle::from_section(d, TextStyle { font_size: 13.0, color: Color::rgb(0.5, 0.7, 0.8), ..default() }));
                });
            }
            btn(g, "✕ CLOSE GEO MAP", Color::rgb(0.3, 0.1, 0.1));
        });

        // SETTINGS PANEL
        c.spawn((NodeBundle {
            style: Style { position_type: PositionType::Absolute, right: Val::Percent(1.0), top: Val::Percent(3.0), width: Val::Px(420.0), max_height: Val::Px(650.0), padding: UiRect::all(Val::Px(16.0)), display: Display::None, flex_direction: FlexDirection::Column, row_gap: Val::Px(6.0), border: UiRect::all(Val::Px(2.0)), ..default() },
            border_color: BorderColor(Color::rgb(0.3, 0.7, 1.0)),
            background_color: BackgroundColor(Color::rgba(0.02, 0.05, 0.12, 0.93)),
            ..default()
        }, SettingsPanel)).with_children(|s| {
            s.spawn(NodeBundle { style: Style { width: Val::Percent(100.0), justify_content: JustifyContent::SpaceBetween, align_items: AlignItems::Center, ..default() }, background_color: BackgroundColor(Color::NONE), ..default() }).with_children(|h| {
                h.spawn((TextBundle::from_section("SETTINGS", TextStyle { font_size: 24.0, color: Color::rgb(0.3, 0.7, 1.0), ..default() }), TranslatableText { key: "settings_title".into() }));
                h.spawn(ButtonBundle {
                    style: Style { width: Val::Px(36.0), height: Val::Px(36.0), justify_content: JustifyContent::Center, align_items: AlignItems::Center, ..default() },
                    background_color: BackgroundColor(Color::rgba(0.8, 0.2, 0.2, 0.8)),
                    ..default()
                }).with_children(|b| { b.spawn(TextBundle::from_section("X", TextStyle { font_size: 20.0, color: Color::WHITE, ..default() })); });
            });
            s.spawn((TextBundle::from_section("--- GRAPHICS ---", TextStyle { font_size: 13.0, color: Color::rgb(0.5, 0.8, 1.0), ..default() }), TranslatableText { key: "graphics_section".into() }));
            btn_with_marker(s, "Night Sky: ON", Color::rgb(0.1, 0.15, 0.3), SettingsNightSkyText);
            btn_with_marker(s, "Quality: High", Color::rgb(0.1, 0.2, 0.25), SettingsQualityText);
            btn_with_marker(s, "Fullscreen: OFF", Color::rgb(0.1, 0.2, 0.2), SettingsFullscreenText);
            s.spawn((TextBundle::from_section("--- DISPLAY ---", TextStyle { font_size: 13.0, color: Color::rgb(0.5, 0.8, 1.0), ..default() }), TranslatableText { key: "display_section".into() }));
            btn_with_marker(s, "Show Orbits: ON", Color::rgb(0.08, 0.2, 0.15), SettingsShowOrbitsText);
            btn_with_marker(s, "Planet Labels: ON", Color::rgb(0.1, 0.15, 0.2), SettingsPlanetLabelsText);
            btn_with_marker(s, "Show FPS: OFF", Color::rgb(0.12, 0.15, 0.2), SettingsShowFpsText);
            btn_with_marker(s, "Auto Rotate: ON", Color::rgb(0.1, 0.18, 0.22), SettingsAutoRotateText);
            s.spawn((TextBundle::from_section("--- AUDIO ---", TextStyle { font_size: 13.0, color: Color::rgb(0.5, 0.8, 1.0), ..default() }), TranslatableText { key: "audio_section".into() }));
            btn_with_marker(s, "Audio: ON", Color::rgb(0.15, 0.2, 0.25), SettingsAudioText);
            btn_with_marker(s, "Music: 70% +", Color::rgb(0.1, 0.18, 0.15), SettingsMusicPlusText);
            btn_with_marker(s, "Music: 70% -", Color::rgb(0.1, 0.15, 0.18), SettingsMusicMinusText);
            btn_with_marker(s, "SFX: 80% +", Color::rgb(0.18, 0.1, 0.15), SettingsSfxPlusText);
            btn_with_marker(s, "SFX: 80% -", Color::rgb(0.15, 0.1, 0.18), SettingsSfxMinusText);
            s.spawn((TextBundle::from_section("--- LANGUAGE ---", TextStyle { font_size: 13.0, color: Color::rgb(0.5, 0.8, 1.0), ..default() }), TranslatableText { key: "language_section".into() }));
            btn_with_marker(s, "Language: English", Color::rgb(0.2, 0.15, 0.3), SettingsLanguageText);
            s.spawn((TextBundle::from_section("--- COMBAT ---", TextStyle { font_size: 13.0, color: Color::rgb(1.0, 0.5, 0.3), ..default() }), TranslatableText { key: "combat_section".into() }));
            btn_with_marker(s, "Difficulty: Normal", Color::rgb(0.2, 0.1, 0.1), SettingsDifficultyText);
            btn_with_marker(s, "Shield: 50/100", Color::rgb(0.1, 0.2, 0.3), SettingsShieldText);
            btn(s, "Close", Color::rgb(0.3, 0.1, 0.1));
        });

        // CHALLENGES PANEL
        c.spawn((NodeBundle {
            style: Style { position_type: PositionType::Absolute, left: Val::Percent(10.0), top: Val::Percent(10.0), width: Val::Px(600.0), height: Val::Px(500.0), padding: UiRect::all(Val::Px(20.0)), display: Display::None, flex_direction: FlexDirection::Column, row_gap: Val::Px(10.0), border: UiRect::all(Val::Px(2.0)), overflow: Overflow::clip(), ..default() },
            border_color: BorderColor(Color::rgb(0.8, 0.5, 0.2)),
            background_color: BackgroundColor(Color::rgba(0.02, 0.03, 0.1, 0.95)),
            ..default()
        }, ChallengesPanel)).with_children(|h| {
            h.spawn(NodeBundle {
                style: Style { width: Val::Percent(100.0), justify_content: JustifyContent::SpaceBetween, align_items: AlignItems::Center, ..default() },
                background_color: BackgroundColor(Color::NONE),
                ..default()
            }).with_children(|t| {
                t.spawn(TextBundle::from_section("CHALLENGES & MISSIONS", TextStyle { font_size: 24.0, color: Color::rgb(0.8, 0.5, 0.2), ..default() }));
                t.spawn(ButtonBundle {
                    style: Style { width: Val::Px(36.0), height: Val::Px(36.0), justify_content: JustifyContent::Center, align_items: AlignItems::Center, ..default() },
                    background_color: BackgroundColor(Color::rgba(0.8, 0.2, 0.2, 0.8)),
                    ..default()
                }).with_children(|b| { b.spawn(TextBundle::from_section("X", TextStyle { font_size: 20.0, color: Color::WHITE, ..default() })); });
            });
            h.spawn((TextBundle::from_section("Select a challenge to begin:", TextStyle { font_size: 16.0, color: Color::rgb(0.7, 0.8, 0.9), ..default() }), ActiveChallengeText));
            h.spawn(NodeBundle { style: Style { width: Val::Percent(100.0), flex_direction: FlexDirection::Column, row_gap: Val::Px(8.0), ..default() }, background_color: BackgroundColor(Color::NONE), ..default() }).with_children(|l| {
                let challenges_list = get_challenges();
                for (i, ch) in challenges_list.iter().enumerate() {
                    l.spawn((ButtonBundle {
                        style: Style { width: Val::Percent(100.0), height: Val::Px(60.0), justify_content: JustifyContent::Center, align_items: AlignItems::Center, padding: UiRect::all(Val::Px(10.0)), border: UiRect::all(Val::Px(1.0)), ..default() },
                        border_color: BorderColor(Color::rgba(0.8, 0.5, 0.2, 0.3)),
                        background_color: BackgroundColor(Color::rgba(0.1, 0.08, 0.05, 0.8)),
                        ..default()
                    }, ChallengeButton { challenge_index: i })).with_children(|b| {
                        let label = format!("{} - {}", ch.title, ch.description);
                        b.spawn(TextBundle::from_section(label, TextStyle { font_size: 14.0, color: Color::rgb(0.9, 0.8, 0.6), ..default() }));
                    });
                }
            });
            h.spawn((TextBundle::from_section("Time Remaining: --:--", TextStyle { font_size: 16.0, color: Color::rgb(0.8, 0.6, 0.3), ..default() }), TimerText));
            btn(h, "CLOSE CHALLENGES", Color::rgb(0.3, 0.1, 0.1));
        });

        // COMBAT PANEL - Added win/lose overlay screens
        c.spawn((NodeBundle {
            style: Style { position_type: PositionType::Absolute, left: Val::Percent(10.0), top: Val::Percent(10.0), width: Val::Px(650.0), height: Val::Px(500.0), padding: UiRect::all(Val::Px(20.0)), display: Display::None, flex_direction: FlexDirection::Column, row_gap: Val::Px(10.0), border: UiRect::all(Val::Px(2.0)), overflow: Overflow::clip(), ..default() },
            border_color: BorderColor(Color::rgb(1.0, 0.3, 0.3)),
            background_color: BackgroundColor(Color::rgba(0.05, 0.01, 0.02, 0.95)),
            ..default()
        }, CombatPanel)).with_children(|m| {
            m.spawn(NodeBundle {
                style: Style { width: Val::Percent(100.0), justify_content: JustifyContent::SpaceBetween, align_items: AlignItems::Center, ..default() },
                background_color: BackgroundColor(Color::NONE),
                ..default()
            }).with_children(|h| {
                h.spawn(TextBundle::from_section("SPACE COMBAT", TextStyle { font_size: 24.0, color: Color::rgb(1.0, 0.3, 0.3), ..default() }));
                h.spawn(ButtonBundle {
                    style: Style { width: Val::Px(36.0), height: Val::Px(36.0), justify_content: JustifyContent::Center, align_items: AlignItems::Center, ..default() },
                    background_color: BackgroundColor(Color::rgba(0.8, 0.2, 0.2, 0.8)),
                    ..default()
                }).with_children(|b| { b.spawn(TextBundle::from_section("X", TextStyle { font_size: 20.0, color: Color::WHITE, ..default() })); });
            });
            m.spawn((TextBundle::from_section("Ship Status: Health 100%  Shield 50%  Wave: 0  Kills: 0", TextStyle { font_size: 16.0, color: Color::rgb(0.8, 0.7, 0.5), ..default() }), CombatStatusText));
            // Health bar
            m.spawn(NodeBundle {
                style: Style { width: Val::Percent(100.0), height: Val::Px(20.0), flex_direction: FlexDirection::Row, ..default() },
                background_color: BackgroundColor(Color::NONE),
                ..default()
            }).with_children(|hb| {
                hb.spawn(NodeBundle {
                    style: Style { width: Val::Percent(100.0), height: Val::Percent(100.0), ..default() },
                    background_color: BackgroundColor(Color::rgb(0.3, 0.05, 0.05)),
                    ..default()
                }).with_children(|bg| {
                    bg.spawn((NodeBundle {
                        style: Style { width: Val::Percent(100.0), height: Val::Percent(100.0), ..default() },
                        background_color: BackgroundColor(Color::rgb(0.1, 0.8, 0.2)),
                        ..default()
                    }, CombatHealthBar));
                });
            });
            // Shield bar
            m.spawn(NodeBundle {
                style: Style { width: Val::Percent(100.0), height: Val::Px(14.0), flex_direction: FlexDirection::Row, ..default() },
                background_color: BackgroundColor(Color::NONE),
                ..default()
            }).with_children(|sb| {
                sb.spawn(NodeBundle {
                    style: Style { width: Val::Percent(100.0), height: Val::Percent(100.0), ..default() },
                    background_color: BackgroundColor(Color::rgba(0.05, 0.05, 0.3, 0.8)),
                    ..default()
                }).with_children(|bg| {
                    bg.spawn((NodeBundle {
                        style: Style { width: Val::Percent(50.0), height: Val::Percent(100.0), ..default() },
                        background_color: BackgroundColor(Color::rgb(0.2, 0.4, 0.9)),
                        ..default()
                    }, CombatShieldBar));
                });
            });
            m.spawn(TextBundle::from_section("Use [SPACE] to shoot rapid-fire lasers | [WASD] to move | Destroy 10 enemies to win!", TextStyle { font_size: 15.0, color: Color::rgb(0.9, 0.6, 0.4), ..default() }));
            m.spawn(TextBundle::from_section("Press [C] to start combat!", TextStyle { font_size: 18.0, color: Color::rgb(0.3, 1.0, 0.3), ..default() }));
            btn(m, "CLOSE COMBAT", Color::rgb(0.3, 0.1, 0.1));

            // WIN OVERLAY (hidden by default)
            m.spawn((NodeBundle {
                style: Style { position_type: PositionType::Absolute, left: Val::Px(0.0), top: Val::Px(0.0), width: Val::Percent(100.0), height: Val::Percent(100.0), flex_direction: FlexDirection::Column, justify_content: JustifyContent::Center, align_items: AlignItems::Center, display: Display::None, row_gap: Val::Px(20.0), ..default() },
                background_color: BackgroundColor(Color::rgba(0.0, 0.3, 0.0, 0.85)),
                ..default()
            }, CombatWinScreen)).with_children(|w| {
                w.spawn(TextBundle::from_section("🏆 YOU WIN! 🏆", TextStyle { font_size: 48.0, color: Color::rgb(0.3, 1.0, 0.3), ..default() }));
                w.spawn(TextBundle::from_section("Congratulations! You defeated all enemies!", TextStyle { font_size: 20.0, color: Color::WHITE, ..default() }));
                w.spawn((TextBundle::from_section("", TextStyle { font_size: 18.0, color: Color::rgb(0.8, 0.8, 0.3), ..default() }), CombatResultStatsText));
                w.spawn(ButtonBundle {
                    style: Style { width: Val::Px(250.0), height: Val::Px(50.0), justify_content: JustifyContent::Center, align_items: AlignItems::Center, border: UiRect::all(Val::Px(2.0)), ..default() },
                    border_color: BorderColor(Color::rgb(0.3, 0.8, 0.3)),
                    background_color: BackgroundColor(Color::rgb(0.1, 0.3, 0.1)),
                    ..default()
                }).with_children(|b| { b.spawn(TextBundle::from_section("RETURN TO EXPLORATION", TextStyle { font_size: 18.0, color: Color::WHITE, ..default() })); });
            });

            // LOSE OVERLAY (hidden by default)
            m.spawn((NodeBundle {
                style: Style { position_type: PositionType::Absolute, left: Val::Px(0.0), top: Val::Px(0.0), width: Val::Percent(100.0), height: Val::Percent(100.0), flex_direction: FlexDirection::Column, justify_content: JustifyContent::Center, align_items: AlignItems::Center, display: Display::None, row_gap: Val::Px(20.0), ..default() },
                background_color: BackgroundColor(Color::rgba(0.3, 0.0, 0.0, 0.85)),
                ..default()
            }, CombatLoseScreen)).with_children(|l| {
                l.spawn(TextBundle::from_section("💀 YOU LOSE! 💀", TextStyle { font_size: 48.0, color: Color::rgb(1.0, 0.2, 0.2), ..default() }));
                l.spawn(TextBundle::from_section("Your ship was destroyed in battle!", TextStyle { font_size: 20.0, color: Color::WHITE, ..default() }));
                l.spawn((TextBundle::from_section("", TextStyle { font_size: 18.0, color: Color::rgb(0.8, 0.3, 0.3), ..default() }), CombatResultStatsText));
                l.spawn(ButtonBundle {
                    style: Style { width: Val::Px(250.0), height: Val::Px(50.0), justify_content: JustifyContent::Center, align_items: AlignItems::Center, border: UiRect::all(Val::Px(2.0)), ..default() },
                    border_color: BorderColor(Color::rgb(0.8, 0.3, 0.3)),
                    background_color: BackgroundColor(Color::rgb(0.3, 0.1, 0.1)),
                    ..default()
                }).with_children(|b| { b.spawn(TextBundle::from_section("TRY AGAIN", TextStyle { font_size: 18.0, color: Color::WHITE, ..default() })); });
                l.spawn(ButtonBundle {
                    style: Style { width: Val::Px(250.0), height: Val::Px(50.0), justify_content: JustifyContent::Center, align_items: AlignItems::Center, border: UiRect::all(Val::Px(2.0)), ..default() },
                    border_color: BorderColor(Color::rgb(0.4, 0.4, 0.4)),
                    background_color: BackgroundColor(Color::rgb(0.15, 0.15, 0.2)),
                    ..default()
                }).with_children(|b| { b.spawn(TextBundle::from_section("RETURN TO EXPLORATION", TextStyle { font_size: 18.0, color: Color::WHITE, ..default() })); });
            });
        });

        // LANDING PANEL
        c.spawn((NodeBundle {
            style: Style { position_type: PositionType::Absolute, left: Val::Percent(15.0), top: Val::Percent(12.0), width: Val::Px(650.0), padding: UiRect::all(Val::Px(20.0)), display: Display::None, flex_direction: FlexDirection::Column, row_gap: Val::Px(10.0), border: UiRect::all(Val::Px(2.0)), ..default() },
            border_color: BorderColor(Color::rgb(0.5, 0.8, 0.3)),
            background_color: BackgroundColor(Color::rgba(0.02, 0.05, 0.03, 0.95)),
            ..default()
        }, LandingPanel)).with_children(|l| {
            l.spawn(NodeBundle {
                style: Style { width: Val::Percent(100.0), justify_content: JustifyContent::SpaceBetween, align_items: AlignItems::Center, ..default() },
                background_color: BackgroundColor(Color::NONE),
                ..default()
            }).with_children(|h| {
                h.spawn(TextBundle::from_section("PLANET LANDING", TextStyle { font_size: 24.0, color: Color::rgb(0.5, 0.8, 0.3), ..default() }));
                h.spawn(ButtonBundle {
                    style: Style { width: Val::Px(36.0), height: Val::Px(36.0), justify_content: JustifyContent::Center, align_items: AlignItems::Center, ..default() },
                    background_color: BackgroundColor(Color::rgba(0.8, 0.2, 0.2, 0.8)),
                    ..default()
                }).with_children(|b| { b.spawn(TextBundle::from_section("X", TextStyle { font_size: 20.0, color: Color::WHITE, ..default() })); });
            });
            l.spawn(TextBundle::from_section("You have landed on Earth!", TextStyle { font_size: 18.0, color: Color::rgb(0.7, 0.9, 0.6), ..default() }));
            l.spawn(TextBundle::from_section("Explore the planet's geography, learn facts, or take a quiz.", TextStyle { font_size: 16.0, color: Color::rgb(0.6, 0.8, 0.7), ..default() }));
            l.spawn(NodeBundle { style: Style { width: Val::Percent(100.0), flex_direction: FlexDirection::Column, row_gap: Val::Px(6.0), padding: UiRect::all(Val::Px(10.0)), ..default() }, background_color: BackgroundColor(Color::rgba(0.05, 0.1, 0.08, 0.6)), ..default() }).with_children(|n| {
                n.spawn(TextBundle::from_section("Welcome to Earth!", TextStyle { font_size: 18.0, color: Color::rgb(0.5, 0.8, 0.5), ..default() }));
                n.spawn(TextBundle::from_section("71% water - 8.7M species - 7 continents - 195 countries", TextStyle { font_size: 14.0, color: Color::rgb(0.6, 0.7, 0.8), ..default() }));
                n.spawn(TextBundle::from_section("Atmosphere: 78% N2, 21% O2 - Surface temp: -89 to 57C", TextStyle { font_size: 14.0, color: Color::rgb(0.6, 0.7, 0.8), ..default() }));
            });
            btn(l, "VIEW EDUCATIONAL FACTS", Color::rgb(0.1, 0.25, 0.15));
            btn(l, "TAKE PLANET QUIZ", Color::rgb(0.2, 0.15, 0.3));
            btn(l, "VIEW GEO MAP", Color::rgb(0.1, 0.2, 0.25));
            btn(l, "LAUNCH BACK TO SPACE", Color::rgb(0.3, 0.1, 0.1));
        });

        // BOTTOM TOOLBAR
        c.spawn(NodeBundle {
            style: Style { width: Val::Percent(100.0), height: Val::Px(80.0), padding: UiRect::all(Val::Px(8.0)), justify_content: JustifyContent::SpaceEvenly, align_items: AlignItems::Center, ..default() },
            background_color: BackgroundColor(Color::rgba(0.0, 0.03, 0.08, 0.9)),
            ..default()
        }).with_children(|b| {
            hbtn_translatable(b, "SETTINGS", Color::rgb(0.15, 0.2, 0.35), "settings_toolbar");
            hbtn_translatable(b, "GEO MAP", Color::rgb(0.1, 0.25, 0.15), "geo_map_toolbar");
            hbtn_translatable(b, "PLANETS", Color::rgb(0.15, 0.15, 0.3), "planets_toolbar");
            hbtn_translatable(b, "ORBITS", Color::rgb(0.1, 0.2, 0.2), "orbits_toolbar");
            hbtn_translatable(b, "NIGHT", Color::rgb(0.1, 0.1, 0.3), "night_toolbar");
            hbtn_translatable(b, "LEARN", Color::rgb(0.15, 0.25, 0.1), "learn_toolbar");
            hbtn_translatable(b, "CHALLENGES", Color::rgb(0.3, 0.2, 0.1), "challenges_toolbar");
            hbtn_translatable(b, "COMBAT", Color::rgb(0.3, 0.1, 0.1), "combat_toolbar");
        });
    });
}

fn btn(parent: &mut ChildBuilder, text: &str, color: Color) {
    parent.spawn(ButtonBundle {
        style: Style { width: Val::Px(220.0), height: Val::Px(40.0), justify_content: JustifyContent::Center, align_items: AlignItems::Center, margin: UiRect::top(Val::Px(4.0)), border: UiRect::all(Val::Px(1.0)), ..default() },
        border_color: BorderColor(Color::rgba(0.5, 0.7, 1.0, 0.3)),
        background_color: BackgroundColor(color),
        ..default()
    }).with_children(|b| { b.spawn(TextBundle::from_section(text, TextStyle { font_size: 15.0, color: Color::WHITE, ..default() })); });
}

fn btn_with_marker<M: Component + Clone + Copy>(parent: &mut ChildBuilder, text: &str, color: Color, marker: M) {
    parent.spawn((ButtonBundle {
        style: Style { width: Val::Px(220.0), height: Val::Px(40.0), justify_content: JustifyContent::Center, align_items: AlignItems::Center, margin: UiRect::top(Val::Px(4.0)), border: UiRect::all(Val::Px(1.0)), ..default() },
        border_color: BorderColor(Color::rgba(0.5, 0.7, 1.0, 0.3)),
        background_color: BackgroundColor(color),
        ..default()
    }, marker)).with_children(|b| { b.spawn((TextBundle::from_section(text, TextStyle { font_size: 15.0, color: Color::WHITE, ..default() }), marker)); });
}

fn hbtn(parent: &mut ChildBuilder, text: &str, color: Color) {
    parent.spawn(ButtonBundle {
        style: Style { width: Val::Px(160.0), height: Val::Px(50.0), justify_content: JustifyContent::Center, align_items: AlignItems::Center, border: UiRect::all(Val::Px(1.0)), ..default() },
        border_color: BorderColor(Color::rgba(0.5, 0.7, 1.0, 0.2)),
        background_color: BackgroundColor(color),
        ..default()
    }).with_children(|b| { b.spawn(TextBundle::from_section(text, TextStyle { font_size: 15.0, color: Color::WHITE, ..default() })); });
}

fn hbtn_translatable(parent: &mut ChildBuilder, text: &str, color: Color, key: &str) {
    parent.spawn((ButtonBundle {
        style: Style { width: Val::Px(160.0), height: Val::Px(50.0), justify_content: JustifyContent::Center, align_items: AlignItems::Center, border: UiRect::all(Val::Px(1.0)), ..default() },
        border_color: BorderColor(Color::rgba(0.5, 0.7, 1.0, 0.2)),
        background_color: BackgroundColor(color),
        ..default()
    }, ToolbarButtonText { key: key.to_string() })).with_children(|b| { b.spawn((TextBundle::from_section(text, TextStyle { font_size: 15.0, color: Color::WHITE, ..default() }), ToolbarButtonText { key: key.to_string() })); });
}

// ── PLANET INFO ──
fn fly_info(
    interaction: Query<(&Interaction, &Children), (Changed<Interaction>, With<Button>)>,
    selected: Res<SelectedPlanet>,
    enc: Res<Encyclopedia>,
    mut panel: Query<(&mut Style, &mut Visibility), With<PlanetInfoPanel>>,
    mut text_queries: ParamSet<(
        Query<&Text>,
        Query<&mut Text, (With<PlanetInfoPanel>, Without<HudClock>, Without<SearchInput>)>,
    )>,
    mut ns: ResMut<NextState<AppState>>,
    current_state: Res<State<AppState>>,
) {
    if *current_state.get() != AppState::PlanetInfo {
        if let Ok((mut style, mut vis)) = panel.get_single_mut() {
            style.display = Display::None;
            *vis = Visibility::Hidden;
        }
        return;
    }
    if let Ok((mut style, mut vis)) = panel.get_single_mut() {
        style.display = Display::Flex;
        *vis = Visibility::Visible;
    }
    if let Some(d) = enc.data.get(&selected.name) {
        let mut query = text_queries.p1();
        let mut iter = query.iter_mut();
        if let Some(mut title) = iter.next() { title.sections[0].value = selected.name.clone().to_uppercase(); }
        if let Some(mut desc) = iter.next() { desc.sections[0].value = d.description.clone(); }
        if let Some(mut s) = iter.next() { s.sections[0].value = format!(\"Mass: {} M  |  Radius: {} km  |  Day: {}h\", d.mass, d.radius, d.day_length); }
        if let Some(mut s) = iter.next() { s.sections[0].value = format!(\"Moons: {}  |  Surface: {}\", d.moons, d.surface_temp); }
    }
    for (interact, children) in interaction.iter() {
        if *interact == Interaction::Pressed {
            for &child in children.iter() {
                if let Ok(text) = text_queries.p0().get(child) {
                    if let Some(section) = text.sections.first() {
                        let clicked = &section.value;
                        if clicked.contains(\"X\") || clicked.contains(\"CLOSE\") { ns.set(AppState::Exploration); }
                        else if clicked.contains(\"LEARN\") { ns.set(AppState::Educational); }
                        else if clicked.contains(\"QUIZ\") { ns.set(AppState::Quiz); }
                        else if clicked.contains(\"GEO MAP\") { ns.set(AppState::GeoMap); }
                    }
                }
            }
        }
    }
}

fn rand() -> f32 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let seed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    (seed % 1000) as f32 / 1000.0
}
"""
    
    with open('c:/Rust/galactic_explorer/src/main.rs', 'w', encoding='utf-8') as f:
        f.write(content)
    print("Done! File written successfully")
else:
    print("Setup already contains other functions, checking...")
    
    import os
    print(f"File size: {os.path.getsize('c:/Rust/galactic_explorer/src/main.rs')} bytes")
    print(f"Has setup_loading_screen: {'fn setup_loading_screen' in content}")
    print(f"Has handle_toolbar_buttons: {'fn handle_toolbar_buttons' in content}")
    print(f"Has fly_landing: {'fn fly_landing' in content}")
    print(f"Has update_combat: {'fn update_combat' in content}")
    print(f"Has destroy_asteroids: {'fn destroy_asteroids' in content}")
    print(f"Has fly_combat: {'fn fly_combat' in content}")
    print(f"Has rand(): {'fn rand()' in content}")