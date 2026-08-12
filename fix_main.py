# Script to add all missing functions to main.rs
with open('c:/Rust/galactic_explorer/src/main.rs', 'r', encoding='utf-8') as f:
    content = f.read()

# List of missing function signatures to check
missing_fns = [
    "fn setup_fps_counter", "fn setup_audio", "fn fly_settings", "fn fly_geo", 
    "fn fly_edu", "fn fly_quiz", "fn fly_challenges", "fn fly_combat",
    "fn fly_landing", "fn hide_panels_on_state_exit", "fn update_hud_clock",
    "fn search_planets", "fn update_planet_labels", "fn sync_health_from_physics",
    "fn update_combat", "fn update_combat_ui", "fn update_enemy_bullets",
    "fn update_night_sky_visibility", "fn update_orbit_visibility",
    "fn handle_quiz_buttons", "fn update_settings_ui", "fn apply_all_settings",
    "fn destroy_asteroids_with_player_bullets"
]

missing = [fn for fn in missing_fns if fn not in content]
print(f"Missing {len(missing)} functions: {missing}")

if missing:
    # Find the position of the last function (rand) and append after it
    # Or find the setup_loading_screen and add before it
    pos = content.find("fn rand() -> f32")
    if pos > 0:
        # Find the closing brace of rand
        end_pos = content.find("fn setup_loading_screen")
        if end_pos > 0:
            insert_pos = content[:end_pos].rfind("}")
            if insert_pos > 0:
                insert_pos = end_pos
            else:
                insert_pos = end_pos
        else:
            insert_pos = len(content)
        
        # Add all missing functions before setup_loading_screen
        add = """
fn setup_fps_counter(mut commands: Commands) {
    let id = commands
        .spawn((
            TextBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(5.0),
                    right: Val::Px(5.0),
                    ..default()
                },
                text: Text::from_section("FPS: --", TextStyle {
                    font_size: 16.0,
                    color: Color::rgb(0.0, 1.0, 0.0),
                    ..default()
                }),
                visibility: Visibility::Hidden,
                ..default()
            },
            FpsCounter,
        ))
        .id();
    commands.insert_resource(FpsText { entity: Some(id) });
}

fn setup_audio(_commands: Commands, asset_server: Res<AssetServer>) {
    let _handle: Handle<AudioSource> = asset_server.load("audio/space_ambient.ogg");
}

fn fly_settings(
    interaction: Query<(&Interaction, &Children), (Changed<Interaction>, With<Button>)>,
    button_texts: Query<&Text>,
    mut settings: ResMut<AppSettings>,
    mut ship: ResMut<PlayerShip>,
    mut panel: Query<&mut Style, With<SettingsPanel>>,
    mut ns: ResMut<NextState<AppState>>,
) {
    if let Ok(mut s) = panel.get_single_mut() { s.display = Display::Flex; }
    for (interact, children) in interaction.iter() {
        if *interact != Interaction::Pressed { continue; }
        for &child in children.iter() {
            if let Ok(text) = button_texts.get(child) {
                if let Some(section) = text.sections.first() {
                    let clicked = &section.value;
                    if clicked == "X" || clicked.contains("CLOSE") || clicked.contains("Close") || clicked.contains("Cerrar") || clicked.contains("Fermer") { ns.set(AppState::Exploration); continue; }
                    if clicked.contains("Night Sky") || clicked.contains("Cielo Nocturno") || clicked.contains("Ciel Nocturne") { settings.night_sky_enabled = !settings.night_sky_enabled; }
                    else if clicked.contains("Quality") || clicked.contains("Calidad") || clicked.contains("Qualité") {
                        settings.graphics_quality = match settings.graphics_quality.as_str() { "High" => "Medium".into(), "Medium" => "Low".into(), _ => "High".into() };
                    }
                    else if clicked.contains("Fullscreen") || clicked.contains("Pantalla Completa") || clicked.contains("Plein Écran") { settings.fullscreen = !settings.fullscreen; }
                    else if clicked.contains("Show Orbits") || clicked.contains("Mostrar Órbitas") || clicked.contains("Afficher Orbites") { settings.show_planet_orbits = !settings.show_planet_orbits; }
                    else if clicked.contains("Planet Labels") || clicked.contains("Etiquetas") || clicked.contains("Étiquettes") { settings.show_planet_labels = !settings.show_planet_labels; }
                    else if clicked.contains("Show FPS") || clicked.contains("Mostrar FPS") || clicked.contains("Afficher FPS") { settings.show_fps = !settings.show_fps; }
                    else if clicked.contains("Auto Rotate") || clicked.contains("Rotación Auto") || clicked.contains("Rotation Auto") { settings.auto_rotate = !settings.auto_rotate; }
                    else if clicked.contains("Audio") { settings.audio_enabled = !settings.audio_enabled; }
                    else if clicked.contains("Music") || clicked.contains("Música") || clicked.contains("Musique") {
                        if clicked.contains("+") { settings.music_volume = (settings.music_volume + 0.1).min(1.0); }
                        else if clicked.contains("-") { settings.music_volume = (settings.music_volume - 0.1).max(0.0); }
                    }
                    else if clicked.contains("SFX") || clicked.contains("EFX") || clicked.contains("Effets") {
                        if clicked.contains("+") { settings.sfx_volume = (settings.sfx_volume + 0.1).min(1.0); }
                        else if clicked.contains("-") { settings.sfx_volume = (settings.sfx_volume - 0.1).max(0.0); }
                    }
                    else if clicked.contains("Language") || clicked.contains("Idioma") || clicked.contains("Langue") {
                        settings.language = match settings.language.as_str() { "English" => "Spanish".into(), "Spanish" => "French".into(), "French" => "English".into(), _ => "English".into() };
                    }
                    else if clicked.contains("Difficulty") || clicked.contains("Dificultad") || clicked.contains("Difficulté") {
                        settings.combat_difficulty = match settings.combat_difficulty.as_str() { "Easy" => "Normal".into(), "Normal" => "Hard".into(), _ => "Easy".into() };
                    }
                    else if clicked.contains("Shield") || clicked.contains("Escudo") || clicked.contains("Bouclier") { ship.shield = (ship.shield + 25.0).min(ship.max_shield); }
                }
            }
        }
    }
}

fn fly_geo(
    interaction: Query<(&Interaction, &Children), (Changed<Interaction>, With<Button>)>,
    button_texts: Query<&Text>,
    mut panel: Query<&mut Style, With<GeoMapPanel>>,
    mut ns: ResMut<NextState<AppState>>,
) {
    if let Ok(mut g) = panel.get_single_mut() { g.display = Display::Flex; }
    for (interact, children) in interaction.iter() {
        if *interact == Interaction::Pressed {
            for &child in children.iter() {
                if let Ok(text) = button_texts.get(child) {
                    if let Some(section) = text.sections.first() {
                        let v = &section.value;
                        if v.contains("X") || v.contains("CLOSE") || v.contains("GEOMAP") {
                            if let Ok(mut g) = panel.get_single_mut() { g.display = Display::None; }
                            ns.set(AppState::Exploration);
                        }
                    }
                }
            }
        }
    }
}

fn fly_edu(
    interaction: Query<(&Interaction, &Children), (Changed<Interaction>, With<Button>)>,
    enc: Res<Encyclopedia>, selected: Res<SelectedPlanet>,
    mut panel: Query<&mut Style, With<EducationalPanel>>,
    mut ns: ResMut<NextState<AppState>>,
    mut planet_selector: Local<String>,
    _current_state: Res<State<AppState>>,
    mut texts: ParamSet<(
        Query<&mut Text, (With<EduPlanetName>, Without<EducationalPanel>)>,
        Query<&mut Text, (With<EduDescription>, Without<EduPlanetName>)>,
        Query<(&mut Text, &EduFactText)>,
        Query<(&mut Text, &EduPhysicalText)>,
        Query<(&mut Text, &EduFunFactText)>,
        Query<(&mut Text, &EduMissionText)>,
        Query<&Text>,
    )>,
) {
    if *_current_state.get() != AppState::Educational { if let Ok(mut e) = panel.get_single_mut() { e.display = Display::None; } return; }
    if let Ok(mut e) = panel.get_single_mut() { e.display = Display::Flex; }
    let pn = if !planet_selector.is_empty() { planet_selector.clone() } else { selected.name.clone() };
    if let Some(d) = enc.data.get(&pn) {
        let emoji = match pn.as_str() { "Mercury" => "☿️", "Venus" => "♀️", "Earth" => "🌍", "Mars" => "♂️", "Jupiter" => "♃", "Saturn" => "♄", "Uranus" => "♅", "Neptune" => "♆", _ => "🪐" };
        if let Ok(mut name) = texts.p0().get_single_mut() { name.sections[0].value = format!(\"{} {} - {}\", emoji, pn, d.description); }
        if let Ok(mut desc) = texts.p1().get_single_mut() { desc.sections[0].value = d.description.clone(); }
        for (mut text, fm) in texts.p2().iter_mut() { text.sections[0].value = if fm.0 < d.educational_facts.len() { format!(\"• {}\", d.educational_facts[fm.0]) } else { String::new() }; }
        let phys = vec![format!(\"Mass: {} Earth masses\", d.mass), format!(\"Radius: {} km\", d.radius), format!(\"Day Length: {} hours\", d.day_length), format!(\"Surface Temperature: {}\", d.surface_temp), format!(\"Number of Moons: {}\", d.moons)];
        for (mut text, pm) in texts.p3().iter_mut() { text.sections[0].value = if pm.0 < phys.len() { format!(\"• {}\", phys[pm.0]) } else { String::new() }; }
        for (mut text, fm) in texts.p4().iter_mut() { text.sections[0].value = if fm.0 < d.fun_facts.len() { format!(\"• {}\", d.fun_facts[fm.0]) } else { String::new() }; }
        let mnames: Vec<String> = d.missions.iter().map(|m| format!(\"🚀 {} ({}) - {}: {}\", m.name, m.year, m.agency, m.description)).collect();
        for (mut text, mm) in texts.p5().iter_mut() { text.sections[0].value = if mm.0 < mnames.len() { mnames[mm.0].clone() } else { String::new() }; }
    }
    for (interact, children) in interaction.iter() {
        if *interact == Interaction::Pressed { for &child in children.iter() {
            if let Ok(text) = texts.p6().get(child) { if let Some(section) = text.sections.first() { let v = &section.value;
                if v.contains(\"CLOSE\") { ns.set(AppState::Exploration); return; }
                if v.contains(\"TAKE QUIZ\") { ns.set(AppState::Quiz); return; }
                if v.contains(\"GEO MAP\") { ns.set(AppState::GeoMap); return; }
            } }
            if let Ok(sel) = texts.p6().get(child) { if let Some(section) = sel.sections.first() { let v = &section.value;
                for p in [\"Mercury\",\"Venus\",\"Earth\",\"Mars\",\"Jupiter\",\"Saturn\",\"Uranus\",\"Neptune\"] { if v.contains(p) { *planet_selector = p.to_string(); return; } }
            } }
        }}
    }
}

fn fly_quiz(
    enc: Res<Encyclopedia>, mut qs: ResMut<QuizState>, selected: Res<SelectedPlanet>,
    mut panel: Query<&mut Style, With<QuizPanel>>,
    mut texts: ParamSet<(
        Query<&mut Text, (With<QuizScoreText>, Without<QuizQuestionText>)>,
        Query<&mut Text, (With<QuizQuestionText>, Without<QuizScoreText>)>,
        Query<(&mut Text, &QuizOptionText)>,
        Query<&mut Text, (With<QuizExplanationText>, Without<QuizQuestionText>, Without<QuizScoreText>)>,
        Query<&mut Text, (With<QuizResultText>, Without<QuizExplanationText>, Without<QuizQuestionText>, Without<QuizScoreText>)>,
    )>,
    mut option_bg: Query<(&QuizOptionText, &mut BackgroundColor)>,
) {
    if let Ok(mut q) = panel.get_single_mut() { q.display = Display::Flex; }
    if !qs.active && !qs.finished { qs.active = true; qs.current_planet = selected.name.clone(); qs.current_question = 0; qs.score = 0; qs.correct_count = 0; qs.wrong_count = 0; qs.answered = false; qs.selected_answer = 0; qs.finished = false; qs.used_50_global_questions = false; if let Some(d) = enc.data.get(&selected.name) { qs.total_questions = d.quiz_questions.len() as u32; } }
    if qs.active && !qs.finished { if let Some(d) = enc.data.get(&qs.current_planet) { let total = d.quiz_questions.len(); let current = qs.current_question;
        if let Ok(mut st) = texts.p0().get_single_mut() { st.sections[0].value = format!(\"Score: {}/{}  |  Correct: {}  |  Wrong: {}  |  Question {}/{}\", qs.score, total, qs.correct_count, qs.wrong_count, current + 1, total); }
        if current < total { let q = &d.quiz_questions[current]; if let Ok(mut qt) = texts.p1().get_single_mut() { qt.sections[0].value = q.question.clone(); }
            for (mut text, om) in texts.p2().iter_mut() { text.sections[0].value = if om.0 < q.options.len() { q.options[om.0].clone() } else { String::new() }; }
            for (om, mut bg) in option_bg.iter_mut() { if qs.answered { if om.0 == q.correct_index { *bg = BackgroundColor(Color::rgb(0.1, 0.5, 0.1)); } else if Some(om.0) == Some(qs.selected_answer) { *bg = BackgroundColor(Color::rgb(0.5, 0.1, 0.1)); } else { *bg = BackgroundColor(Color::rgba(0.1, 0.15, 0.25, 0.5)); } } else { *bg = BackgroundColor(Color::rgba(0.1, 0.15, 0.25, 0.8)); } }
            if let Ok(mut et) = texts.p3().get_single_mut() { et.sections[0].value = if qs.answered { q.explanation.clone() } else { String::new() }; }
            if let Ok(mut rt) = texts.p4().get_single_mut() { rt.sections[0].value = String::new(); }
        } else { qs.active = false; qs.finished = true; }
    } }
    if qs.finished {
        if let Ok(mut qt) = texts.p1().get_single_mut() { qt.sections[0].value = \"🎉 QUIZ COMPLETE! 🎉\".to_string(); }
        if let Ok(mut st) = texts.p0().get_single_mut() { st.sections[0].value = format!(\"Final Score: {}/{}\", qs.score, qs.total_questions); }
        if let Ok(mut rt) = texts.p4().get_single_mut() { let pct = if qs.total_questions > 0 { (qs.correct_count as f32 / qs.total_questions as f32) * 100.0 } else { 0.0 }; rt.sections[0].value = format!(\"✅ Correct: {} | ❌ Wrong: {} | 📊 {:.0}%\", qs.correct_count, qs.wrong_count, pct); }
        for (mut text, _) in texts.p2().iter_mut() { text.sections[0].value = String::new(); }
        if let Ok(mut et) = texts.p3().get_single_mut() { et.sections[0].value = String::new(); }
        for (_, mut bg) in option_bg.iter_mut() { *bg = BackgroundColor(Color::rgba(0.1, 0.15, 0.25, 0.8)); }
    }
}

fn handle_quiz_buttons(
    interaction: Query<(&Interaction, &Children), (With<Button>, Without<SettingsPanel>)>,
    button_texts: Query<&Text>, enc: Res<Encyclopedia>, mut qs: ResMut<QuizState>,
    mut panel: Query<&mut Style, With<QuizPanel>>, mut ns: ResMut<NextState<AppState>>,
) {
    for (interact, children) in interaction.iter() { if *interact == Interaction::Pressed { for &child in children.iter() { if let Ok(text) = button_texts.get(child) { if let Some(section) = text.sections.first() { let v = &section.value;
        if v == \"X\" || v.contains(\"✕\") || v.contains(\"CLOSE QUIZ\") { if let Ok(mut q) = panel.get_single_mut() { q.display = Display::None; } qs.active = false; qs.finished = false; ns.set(AppState::Exploration); return; }
        if v.contains(\"NEXT\") { if qs.active && !qs.finished && qs.answered { if let Some(d) = enc.data.get(&qs.current_planet) { qs.current_question += 1; qs.answered = false; qs.selected_answer = 0; if qs.current_question >= d.quiz_questions.len() { qs.active = false; qs.finished = true; } } } return; }
        if qs.active && !qs.finished && !qs.answered { if let Some(d) = enc.data.get(&qs.current_planet) { if qs.current_question < d.quiz_questions.len() { let q = &d.quiz_questions[qs.current_question]; for (i, opt) in q.options.iter().enumerate() { if v == opt { qs.selected_answer = i; qs.answered = true; if i == q.correct_index { qs.correct_count += 1; qs.score += 1; } else { qs.wrong_count += 1; } break; } } } } }
    } } } } }
}

fn fly_challenges(
    interaction: Query<(&Interaction, &Children), (Changed<Interaction>, With<Button>)>,
    text_query: Query<&Text, Without<TimerText>>, mut ns: ResMut<NextState<AppState>>,
    challenges: Res<ChallengesResource>, mut cs: ResMut<ChallengeState>,
    mut panel: Query<&mut Style, With<ChallengesPanel>>,
    mut text_queries: ParamSet<(Query<&Text>, Query<&mut Text, (With<TimerText>, Without<ChallengesPanel>)>)>,
    time: Res<Time>,
) {
    if let Ok(mut c) = panel.get_single_mut() { c.display = Display::Flex; }
    if cs.active { cs.time_remaining -= time.delta_seconds(); if cs.time_remaining <= 0.0 { cs.active = false; cs.time_remaining = 0.0; } }
    if let Ok(mut t) = text_queries.p1().get_single_mut() { t.sections[0].value = if cs.active { format!(\"Time: {:.0}s\", cs.time_remaining) } else { \"Time Remaining: --:--\".into() }; }
    if let Some(clicked) = get_clicked_button(interaction, text_query) {
        if clicked.contains(\"CLOSE\") { ns.set(AppState::Exploration); return; }
        for (i, ch) in challenges.challenges.iter().enumerate() { if clicked.contains(&ch.title.chars().take(5).collect::<String>()) { cs.active = true; cs.current_challenge = i; cs.time_remaining = ch.time_limit; cs.total_time = ch.time_limit; if ch.title.contains(\"Warrior\") { ns.set(AppState::Combat); } break; } }
    }
}

fn fly_combat(
    interaction: Query<(&Interaction, &Children), (Changed<Interaction>, With<Button>)>,
    button_texts: Query<&Text>, mut panel: Query<&mut Style, With<CombatPanel>>,
    mut ns: ResMut<NextState<AppState>>, mut combat: ResMut<CombatState>, mut ship: ResMut<PlayerShip>,
    mut win_screen: Query<&mut Style, (With<CombatWinScreen>, Without<CombatLoseScreen>)>,
    mut lose_screen: Query<&mut Style, (With<CombatLoseScreen>, Without<CombatWinScreen>)>,
) {
    if let Ok(mut c) = panel.get_single_mut() { c.display = Display::Flex; }
    if let Ok(mut ws) = win_screen.get_single_mut() { ws.display = if combat.has_won { Display::Flex } else { Display::None }; }
    if let Ok(mut ls) = lose_screen.get_single_mut() { ls.display = if combat.has_lost { Display::Flex } else { Display::None }; }
    for (interact, children) in interaction.iter() { if *interact == Interaction::Pressed { for &child in children.iter() { if let Ok(text) = button_texts.get(child) { if let Some(section) = text.sections.first() { let v = &section.value;
        if v.contains(\"CLOSE\") { ns.set(AppState::Exploration); combat.active = false; }
        else if v.contains(\"TRY AGAIN\") { combat.has_lost = false; combat.has_won = false; combat.wave = 1; combat.spawn_timer = 0.0; combat.enemy_count = 0; combat.active = true; ship.health = 100.0; ship.kills = 0; }
        else if v.contains(\"RETURN TO EXPLORATION\") { ns.set(AppState::Exploration); combat.active = false; }
    } } } } }
}

fn fly_landing(
    interaction: Query<(&Interaction, &Children), (Changed<Interaction>, With<Button>)>,
    selected: Res<SelectedPlanet>, mut panel: Query<&mut Style, With<LandingPanel>>,
    mut text_queries: ParamSet<(Query<&Text>, Query<&mut Text, (With<LandingPanel>, Without<HudClock>, Without<SearchInput>)>)>,
    enc: Res<Encyclopedia>, mut ns: ResMut<NextState<AppState>>,
) {
    if let Ok(mut p) = panel.get_single_mut() { p.display = Display::Flex; }
    for (interact, children) in interaction.iter() { if *interact == Interaction::Pressed { for &child in children.iter() { if let Ok(text) = text_queries.p0().get(child) { if let Some(section) = text.sections.first() { let v = &section.value;
        if v.contains(\"LAUNCH\") || v.contains(\"X\") { ns.set(AppState::Exploration); }
        else if v.contains(\"EDUCATIONAL\") { ns.set(AppState::Educational); }
        else if v.contains(\"QUIZ\") { ns.set(AppState::Quiz); }
        else if v.contains(\"GEO MAP\") { ns.set(AppState::GeoMap); }
    } } } } }
}

fn hide_panels_on_state_exit(
    mut panels: ParamSet<(
        Query<&mut Style, With<SettingsPanel>>, Query<&mut Style, With<GeoMapPanel>>,
        Query<&mut Style, With<EducationalPanel>>, Query<&mut Style, With<QuizPanel>>,
        Query<&mut Style, With<ChallengesPanel>>, Query<&mut Style, With<CombatPanel>>,
        Query<&mut Style, With<LandingPanel>>, Query<&mut Style, With<PlanetInfoPanel>>,
    )>, current_state: Res<State<AppState>>,
) {
    if let Ok(mut s) = panels.p0().get_single_mut() { if *current_state.get() != AppState::Settings { s.display = Display::None; } }
    if let Ok(mut g) = panels.p1().get_single_mut() { if *current_state.get() != AppState::GeoMap { g.display = Display::None; } }
    if let Ok(mut e) = panels.p2().get_single_mut() { if *current_state.get() != AppState::Educational { e.display = Display::None; } }
    if let Ok(mut q) = panels.p3().get_single_mut() { if *current_state.get() != AppState::Quiz { q.display = Display::None; } }
    if let Ok(mut c) = panels.p4().get_single_mut() { if *current_state.get() != AppState::Challenges { c.display = Display::None; } }
    if let Ok(mut c) = panels.p5().get_single_mut() { if *current_state.get() != AppState::Combat { c.display = Display::None; } }
    if let Ok(mut l) = panels.p6().get_single_mut() { if *current_state.get() != AppState::Landing { l.display = Display::None; } }
    if let Ok(mut p) = panels.p7().get_single_mut() { if *current_state.get() != AppState::PlanetInfo { p.display = Display::None; } }
}

fn update_hud_clock(time: Res<Time>, mut sim_time: ResMut<SimulationTime>, mut clock: Query<&mut Text, With<HudClock>>) {
    let es = time.elapsed_seconds() as u64;
    let hours = (es / 3600) % 24; let minutes = (es / 60) % 60; let seconds = es % 60;
    sim_time.current_time = format!(\"{:02}:{:02}:{:02} UTC\", hours, minutes, seconds);
    sim_time.current_date = format!(\"JUL {}\", es % 31 + 1);
    if let Ok(mut t) = clock.get_single_mut() { t.sections[0].value = format!(\"{}  {}\", sim_time.current_date, sim_time.current_time); }
}

fn search_planets(keys: Res<ButtonInput<KeyCode>>, mut search: ResMut<SearchQuery>, mut st: Query<&mut Text, With<SearchInput>>) {
    if keys.just_pressed(KeyCode::Slash) { search.active = !search.active; }
    if search.active {
        for k in keys.get_just_pressed() { match k { KeyCode::Backspace => { search.text.pop(); } KeyCode::Enter | KeyCode::Escape => { search.text.clear(); search.active = false; } _ => {} } }
        if let Ok(mut t) = st.get_single_mut() { t.sections[0].value = format!(\" SEARCH: [ {} ]\", search.text); }
    }
}

fn update_planet_labels(settings: Res<AppSettings>, planets: Query<(&Planet, &Transform)>) { let _ = settings; let _ = planets; }

fn sync_health_from_physics(health: Res<Health>, mut ship: ResMut<PlayerShip>) {
    if health.is_changed() { ship.health = (health.hearts as f32 / health.max_hearts as f32) * 100.0; }
}

fn update_combat_ui(
    ship: Res<PlayerShip>, combat: Res<CombatState>,
    mut bars: ParamSet<(Query<&mut Style, With<CombatHealthBar>>, Query<&mut Style, With<CombatShieldBar>>)>,
    mut status_text: Query<&mut Text, (With<CombatPanel>, Without<CombatHealthBar>, Without<CombatShieldBar>)>,
    mut result_stats: Query<&mut Text, With<CombatResultStatsText>>,
) {
    if !combat.active && !combat.has_won && !combat.has_lost { return; }
    if let Ok(mut style) = bars.p0().get_single_mut() { let hp = (ship.health / ship.max_health * 100.0).max(0.0); style.width = Val::Percent(hp); }
    if let Ok(mut style) = bars.p1().get_single_mut() { let sp = (ship.shield / ship.max_shield * 100.0).max(0.0); style.width = Val::Percent(sp); }
    for mut text in status_text.iter_mut() { if text.sections[0].value.contains(\"Ship Status\") { text.sections[0].value = format!(\"Ship Status: Health {:.0}%  Shield {:.0}%  Wave: {}  Kills: {}/{}\", (ship.health / ship.max_health * 100.0).max(0.0), (ship.shield / ship.max_shield * 100.0).max(0.0), combat.wave, ship.kills, combat.total_to_kill); } }
    for mut text in result_stats.iter_mut() { if combat.has_won { text.sections[0].value = format!(\"Kills: {} | Score: {} | Waves: {}\", combat.enemy_count, ship.score, combat.wave); } else if combat.has_lost { text.sections[0].value = format!(\"Enemies killed: {} | Score: {} | Wave: {}\", combat.enemy_count, ship.score, combat.wave); } }
}

fn update_enemy_bullets(mut commands: Commands, bullets: Query<(Entity, &mut Bullet, &mut Transform)>, combat: Res<CombatState>) {
    if !combat.active { return; }
    for (entity, _bullet, transform) in bullets.iter() { if transform.translation.length() > 50.0 { commands.entity(entity).despawn(); } }
}

fn update_night_sky_visibility(settings: Res<AppSettings>, mut query: Query<&mut Visibility, With<StarDome>>) {
    if settings.is_changed() { if let Ok(mut vis) = query.get_single_mut() { *vis = if settings.night_sky_enabled { Visibility::Visible } else { Visibility::Hidden }; } }
}

fn update_orbit_visibility(settings: Res<AppSettings>, mut gizmos: Gizmos, planets: Query<(&Transform, &Orbiting)>) {
    if !settings.show_planet_orbits { return; }
    for (_transform, orbiting) in &planets { let segs = 48; for i in 0..segs { let a1 = (i as f32 / segs as f32) * std::f32::consts::TAU; let a2 = ((i + 1) as f32 / segs as f32) * std::f32::consts::TAU; gizmos.line(Vec3::new(a1.cos() * orbiting.radius, 0.0, a1.sin() * orbiting.radius), Vec3::new(a2.cos() * orbiting.radius, 0.0, a2.sin() * orbiting.radius), Color::rgba(0.3, 0.6, 0.8, 0.25)); } }
}

fn update_settings_ui(
    settings: Res<AppSettings>, ship: Res<PlayerShip>, translations: Res<Translations>,
    mut all_settings_text: Query<&mut Text, Or<(With<SettingsNightSkyText>, With<SettingsQualityText>, With<SettingsFullscreenText>, With<SettingsShowOrbitsText>, With<SettingsPlanetLabelsText>, With<SettingsShowFpsText>, With<SettingsAutoRotateText>, With<SettingsAudioText>, With<SettingsMusicPlusText>, With<SettingsMusicMinusText>, With<SettingsSfxPlusText>, With<SettingsSfxMinusText>, With<SettingsLanguageText>, With<SettingsDifficultyText>, With<SettingsShieldText>)>>,
) {
    let lang = &settings.language;
    for mut text in all_settings_text.iter_mut() { let full = &text.sections[0].value;
        if full.starts_with(\"Night Sky\") || full.starts_with(\"Cielo\") || full.starts_with(\"Ciel\") { text.sections[0].value = format!(\"{}: {}\", translations.get(lang, \"night_sky\"), if settings.night_sky_enabled { translations.get(lang, \"on\") } else { translations.get(lang, \"off\") }); }
        else if full.starts_with(\"Quality\") || full.starts_with(\"Calidad\") || full.starts_with(\"Qualité\") { text.sections[0].value = format!(\"{}: {}\", translations.get(lang, \"quality\"), match settings.graphics_quality.as_str() { \"High\" => translations.get(lang, \"high\"), \"Medium\" => translations.get(lang, \"medium_q\"), _ => translations.get(lang, \"low\") }); }
        else if full.starts_with(\"Fullscreen\") || full.starts_with(\"Pantalla\") || full.starts_with(\"Plein\") { text.sections[0].value = format!(\"{}: {}\", translations.get(lang, \"fullscreen\"), if settings.fullscreen { translations.get(lang, \"on\") } else { translations.get(lang, \"off\") }); }
        else if full.starts_with(\"Show Orbits\") || full.starts_with(\"Mostrar\") || full.starts_with(\"Afficher\") { text.sections[0].value = format!(\"{}: {}\", translations.get(lang, \"show_orbits\"), if settings.show_planet_orbits { translations.get(lang, \"on\") } else { translations.get(lang, \"off\") }); }
        else if full.starts_with(\"Planet Labels\") || full.starts_with(\"Etiquetas\") || full.starts_with(\"Étiquettes\") { text.sections[0].value = format!(\"{}: {}\", translations.get(lang, \"planet_labels\"), if settings.show_planet_labels { translations.get(lang, \"on\") } else { translations.get(lang, \"off\") }); }
        else if full.starts_with(\"Show FPS\") || full.starts_with(\"Mostrar FPS\") || full.starts_with(\"Afficher FPS\") { text.sections[0].value = format!(\"{}: {}\", translations.get(lang, \"show_fps\"), if settings.show_fps { translations.get(lang, \"on\") } else { translations.get(lang, \"off\") }); }
        else if full.starts_with(\"Auto Rotate\") || full.starts_with(\"Rotación\") || full.starts_with(\"Rotation\") { text.sections[0].value = format!(\"{}: {}\", translations.get(lang, \"auto_rotate\"), if settings.auto_rotate { translations.get(lang, \"on\") } else { translations.get(lang, \"off\") }); }
        else if full.contains(\"Audio\") { text.sections[0].value = format!(\"{}: {}\", translations.get(lang, \"audio\"), if settings.audio_enabled { translations.get(lang, \"on\") } else { translations.get(lang, \"off\") }); }
        else if full.starts_with(\"Music\") || full.starts_with(\"Música\") || full.starts_with(\"Musique\") { if full.ends_with('+') { text.sections[0].value = format!(\"{}: {:.0}% +\", translations.get(lang, \"music\"), settings.music_volume * 100.0); } else { text.sections[0].value = format!(\"{}: {:.0}% -\", translations.get(lang, \"music\"), settings.music_volume * 100.0); } }
        else if full.starts_with(\"SFX\") || full.starts_with(\"EFX\") || full.starts_with(\"Effets\") { if full.ends_with('+') { text.sections[0].value = format!(\"{}: {:.0}% +\", translations.get(lang, \"sfx\"), settings.sfx_volume * 100.0); } else { text.sections[0].value = format!(\"{}: {:.0}% -\", translations.get(lang, \"sfx\"), settings.sfx_volume * 100.0); } }
        else if full.starts_with(\"Language\") || full.starts_with(\"Idioma\") || full.starts_with(\"Langue\") { text.sections[0].value = format!(\"{}: {}\", translations.get(lang, \"language_label\"), settings.language); }
        else if full.starts_with(\"Difficulty\") || full.starts_with(\"Dificultad\") || full.starts_with(\"Difficulté\") { let dt = match settings.combat_difficulty.as_str() { \"Easy\" => translations.get(lang, \"easy\"), \"Normal\" => translations.get(lang, \"normal\"), _ => translations.get(lang, \"hard\") }; text.sections[0].value = format!(\"{}: {}\", translations.get(lang, \"difficulty\"), dt); }
        else if full.starts_with(\"Shield\") || full.starts_with(\"Escudo\") || full.starts_with(\"Bouclier\") { text.sections[0].value = format!(\"{}: {:.0}/{}\", translations.get(lang, \"shield\"), ship.shield, ship.max_shield); }
    }
}

fn apply_all_settings(
    settings: Res<AppSettings>, mut window: Query<&mut Window>, time: Res<Time>,
    mut last_fps_update: Local<f32>, mut rot_query: Query<(&mut Transform, &Rotating)>,
    mut quality_border: Query<&mut BorderColor, (With<SettingsPanel>, Without<QualityGlowEffect>)>,
    mut main_bg: Query<&mut BackgroundColor, With<AdvancedHud>>,
    mut param_set: ParamSet<(Query<(&mut Visibility, &mut Text), With<FpsCounter>>, Query<&mut Text, With<CombatPanel>>)>,
) {
    if let Ok(mut window) = window.get_single_mut() { if settings.fullscreen != (window.mode == bevy::window::WindowMode::Fullscreen) { window.mode = if settings.fullscreen { bevy::window::WindowMode::Fullscreen } else { bevy::window::WindowMode::Windowed }; } }
    for (mut vis, mut text) in param_set.p0().iter_mut() { *vis = if settings.show_fps { Visibility::Visible } else { Visibility::Hidden }; if settings.show_fps { *last_fps_update += time.delta_seconds(); if *last_fps_update >= 0.5 { text.sections[0].value = format!(\"FPS: {:.0}\", (1.0 / time.delta_seconds().max(0.001)).round()); *last_fps_update = 0.0; } } }
    for mut border in quality_border.iter_mut() { *border = match settings.graphics_quality.as_str() { \"Low\" => BorderColor(Color::rgb(0.8, 0.2, 0.2)), \"Medium\" => BorderColor(Color::rgb(0.8, 0.6, 0.2)), _ => BorderColor(Color::rgb(0.2, 0.8, 0.4)) }; }
    for mut bg in main_bg.iter_mut() { *bg = match settings.graphics_quality.as_str() { \"Low\" => BackgroundColor(Color::rgba(0.08, 0.02, 0.02, 0.95)), \"Medium\" => BackgroundColor(Color::rgba(0.08, 0.08, 0.02, 0.95)), _ => BackgroundColor(Color::rgba(0.0, 0.03, 0.08, 0.85)) }; }
    if settings.auto_rotate { let er = 0.3 * time.delta_seconds(); for (mut transform, _) in rot_query.iter_mut() { transform.rotate_y(er); } }
    for mut text in param_set.p1().iter_mut() { if text.sections[0].value.contains(\"Press [C]\") { text.sections[0].value = format!(\"Press [C] to start combat! (Difficulty: {})\", settings.combat_difficulty); } }
}

fn update_combat(
    mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>,
    mut combat: ResMut<CombatState>, keys: Res<ButtonInput<KeyCode>>, time: Res<Time>,
    mut transforms: ParamSet<(Query<&mut Transform, With<MainCamera>>, Query<(Entity, &mut Enemy, &mut Transform)>, Query<(Entity, &mut Bullet, &mut Transform)>)>,
    mut ship: ResMut<PlayerShip>,
) {
    if !combat.active || combat.has_won || combat.has_lost { return; }
    let cam_translation;
    { let mut cam_query = transforms.p0(); let Ok(mut cam) = cam_query.get_single_mut() else { return };
        if keys.pressed(KeyCode::KeyW) { cam.translation.z += 0.2; }
        if keys.pressed(KeyCode::KeyS) { cam.translation.z -= 0.2; }
        if keys.pressed(KeyCode::KeyA) { cam.translation.x -= 0.2; }
        if keys.pressed(KeyCode::KeyD) { cam.translation.x += 0.2; }
        combat.fire_timer -= time.delta_seconds();
        if keys.pressed(KeyCode::Space) && combat.fire_timer <= 0.0 {
            let laser_mesh = meshes.add(Sphere { radius: 0.08 });
            let laser_mat = materials.add(StandardMaterial { base_color: Color::rgb(0.0, 1.0, 0.3), emissive: Color::rgb(0.0, 1.0, 0.3), unlit: true, ..default() });
            commands.spawn((PbrBundle { mesh: laser_mesh, material: laser_mat, transform: Transform::from_xyz(cam.translation.x, cam.translation.y, cam.translation.z - 2.0), ..default() }, Bullet { direction: Vec3::new(0.0, 0.0, -1.0), speed: 35.0, damage: 15.0 }, PlayerBullet));
            combat.fire_timer = combat.fire_cooldown;
        }
        cam_translation = cam.translation;
    }
    let mut enemy_data = Vec::new();
    { let mut eq = transforms.p1(); for (entity, enemy, mut transform) in eq.iter_mut() { let dir = Vec3::new(0.0 - transform.translation.x, 0.0, 0.0 - transform.translation.z); if dir.length() > 0.0 { let dir = dir.normalize(); transform.translation += dir * enemy.speed * time.delta_seconds(); } enemy_data.push((entity, enemy.health, enemy.reward_score, transform.translation)); } }
    let mut bullet_data = Vec::new();
    { let mut bq = transforms.p2(); for (entity, bullet, mut transform) in bq.iter_mut() { transform.translation += bullet.direction * bullet.speed * time.delta_seconds(); bullet_data.push((entity, bullet.damage, transform.translation)); } }
    let mut enemies_to_kill = Vec::new(); let mut bullets_to_despawn = Vec::new();
    for (be, bd, bp) in bullet_data.iter() { for (ee, eh, _, ep) in enemy_data.iter() { if bp.distance(*ep) < 1.5 { bullets_to_despawn.push(*be); if *eh - bd <= 0.0 { enemies_to_kill.push(*ee); } break; } } }
    for entity in bullets_to_despawn { if let Some(e) = commands.get_entity(entity) { e.despawn(); } }
    for entity in enemies_to_kill { if let Some((_, _, rs, _)) = enemy_data.iter().find(|(e, _, _, _)| *e == entity) { ship.kills += 1; ship.score += *rs; combat.enemy_count += 1; combat.wave = ((combat.enemy_count / 3) + 1).min(10); } if let Some(e) = commands.get_entity(entity) { e.despawn(); } }
    if combat.enemy_count >= combat.total_to_kill { combat.has_won = true; combat.active = false; return; }
    for (ee, _, ep) in enemy_data.iter() { if ep.distance(Vec3::new(cam_translation.x, cam_translation.y, cam_translation.z)) < 2.0 { ship.health = (ship.health - 5.0).max(0.0); if let Some(e) = commands.get_entity(*ee) { e.despawn(); } if ship.health <= 0.0 { combat.has_lost = true; combat.active = false; return; } break; } }
    for (entity, _, transform) in bullet_data { if transform.length() > 50.0 { if let Some(e) = commands.get_entity(entity) { e.despawn(); } } }
    combat.spawn_timer -= time.delta_seconds();
    if combat.spawn_timer <= 0.0 { combat.spawn_timer = 3.0; let x = (rand() * 15.0) - 7.5; let z = (rand() * 15.0) - 7.5 - 20.0; let health = 20.0 + combat.wave as f32 * 5.0; let speed = 0.8 + combat.wave as f32 * 0.08; commands.spawn((PbrBundle { mesh: meshes.add(Sphere { radius: 0.5 }), material: materials.add(StandardMaterial { base_color: Color::rgb(0.8, 0.2, 0.2), emissive: Color::rgb(0.4, 0.1, 0.1), unlit: true, ..default() }), transform: Transform::from_xyz(x, cam_translation.y, cam_translation.z + z), ..default() }, Enemy { health, speed, damage: 5.0, reward_score: 10 + combat.wave as u32 * 5 })); }
}

fn destroy_asteroids_with_player_bullets(
    mut commands: Commands,
    bullets: Query<(Entity, &Transform), (With<PlayerBullet>, Without<Asteroid>)>,
    asteroids: Query<(Entity, &Transform), (With<Asteroid>, Without<PlayerBullet>)>,
) {
    let ap: Vec<(Entity, Vec3)> = asteroids.iter().map(|(e, t)| (e, t.translation)).collect();
    let bp: Vec<(Entity, Vec3)> = bullets.iter().map(|(e, t)| (e, t.translation)).collect();
    for (be, bpos) in &bp { for (ae, apos) in &ap { if bpos.distance(*apos) < 2.0 { if let Some(e) = commands.get_entity(*be) { e.despawn(); } if let Some(e) = commands.get_entity(*ae) { e.despawn_recursive(); } break; } } }
}
"""
    
    content += add
    
    with open('c:/Rust/galactic_explorer/src/main.rs', 'w', encoding='utf-8') as f:
        f.write(content)
    print(f"Added all {len(missing)} missing functions!")
else:
    print("All functions already present!")