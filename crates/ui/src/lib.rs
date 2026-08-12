//! # UI Layer
//!
//! Manages all user interface elements: menus, HUD, minimap, settings.
//! Uses Bevy's UI system for reactive, composable interface elements.

use bevy::prelude::*;
use bevy::window::ReceivedCharacter;
use galactic_explorer_core::prelude::*;

/// Plugin that registers all UI systems.
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui_system)
           .add_systems(
                Update,
                (
                    menu_flow_input,
                    handle_menu_buttons,
                    handle_settings_buttons,
                    handle_quiz_buttons,
                    handle_challenge_buttons,
                    advance_loading,
                    sync_visibility,
                    update_menu,
                    start_music,
                    handle_search,
                    update_virtual_controls,
                    update_hud,
                    update_minimap,
                    update_progression,
                    pulse_hud_color,
                ),
            );
    }
}

/// Sets up all UI nodes at startup.
fn setup_ui_system(mut commands: Commands) {
    // Note: Add a font file at assets/fonts/FiraSans-Bold.ttf for text rendering
    // For now, UI uses colored panels without text
    commands.insert_resource(Flow {
        screen: ScreenMode::Welcome,
        loading_progress: 0.0,
    });
    commands.insert_resource(Search::default());
    commands.insert_resource(Music::default());
    commands.insert_resource(Settings::default());
    commands.insert_resource(VirtualControls::default());
    commands.insert_resource(QuizState::default());
    commands.insert_resource(ChallengeState::default());

    // HUD
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::FlexStart,
                    padding: UiRect::all(Val::Px(18.0)),
                    ..default()
                },
                background_color: BackgroundColor(Color::NONE),
                visibility: Visibility::Hidden,
                ..default()
            },
            HudRoot,
        ))
        .with_children(|root| {
            // Header bar
            root.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Px(40.0),
                    position_type: PositionType::Absolute,
                    top: Val::Px(0.0),
                    left: Val::Px(0.0),
                    padding: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                background_color: BackgroundColor(Color::rgba(0.05, 0.15, 0.2, 0.8)),
                ..default()
            })
            .insert(HeaderText);

            // Main HUD panel
            root.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Px(350.0),
                        height: Val::Px(300.0),
                        position_type: PositionType::Absolute,
                        top: Val::Px(50.0),
                        left: Val::Px(18.0),
                        padding: UiRect::all(Val::Px(12.0)),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::rgba(0.02, 0.05, 0.1, 0.7)),
                    border_color: BorderColor(Color::rgb(0.2, 0.6, 0.7)),
                    ..default()
                },
                HudText,
            ));

            // Focus info panel
            root.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Px(300.0),
                        height: Val::Px(150.0),
                        position_type: PositionType::Absolute,
                        right: Val::Px(18.0),
                        bottom: Val::Px(18.0),
                        padding: UiRect::all(Val::Px(12.0)),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::rgba(0.1, 0.08, 0.02, 0.8)),
                    border_color: BorderColor(Color::rgb(0.8, 0.7, 0.3)),
                    ..default()
                },
                FocusInfo,
            ));

            // Minimap panel
            root.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Px(200.0),
                        height: Val::Px(200.0),
                        position_type: PositionType::Absolute,
                        right: Val::Px(18.0),
                        top: Val::Px(18.0),
                        padding: UiRect::all(Val::Px(10.0)),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::rgba(0.03, 0.08, 0.15, 0.8)),
                    border_color: BorderColor(Color::rgb(0.3, 0.5, 0.7)),
                    ..default()
                },
                MinimapText,
            ));

            // Progression panel
            root.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Px(280.0),
                        height: Val::Px(200.0),
                        position_type: PositionType::Absolute,
                        left: Val::Px(18.0),
                        top: Val::Px(260.0),
                        padding: UiRect::all(Val::Px(12.0)),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::rgba(0.05, 0.1, 0.15, 0.8)),
                    border_color: BorderColor(Color::rgb(0.4, 0.7, 0.5)),
                    ..default()
                },
                ProgressionText,
            ));
        });

// Menu
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: BackgroundColor(Color::rgba(0.01, 0.02, 0.06, 0.92)),
                visibility: Visibility::Visible,
                ..default()
            },
            MenuRoot,
        ))
        .with_children(|root| {
            root.spawn(NodeBundle {
                style: Style {
                    width: Val::Px(840.0),
                    height: Val::Px(520.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::FlexStart,
                    padding: UiRect::all(Val::Px(24.0)),
                    row_gap: Val::Px(14.0),
                    ..default()
                },
                background_color: BackgroundColor(Color::rgba(0.05, 0.1, 0.18, 0.96)),
                border_color: BorderColor(Color::rgb(0.2, 0.7, 0.8)),
                ..default()
            })
            .with_children(|panel| {
                // Title area
                panel.spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(60.0),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::rgb(0.2, 0.7, 0.8)),
                    ..default()
                })
                .insert(MenuTitle);

                // Body area
                panel.spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(300.0),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::rgba(0.1, 0.2, 0.3, 0.8)),
                    ..default()
                })
                .insert(MenuBody);

// Button area
                panel.spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(50.0),
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Center,
                        column_gap: Val::Px(12.0),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::rgba(0.05, 0.12, 0.2, 0.8)),
                    ..default()
                })
                .with_children(|button_area| {
                    // START button
                    button_area.spawn((
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(120.0),
                                height: Val::Px(40.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: BackgroundColor(Color::rgb(0.2, 0.7, 0.4)),
                            ..default()
                        },
                        MenuButton(MenuAction::Start),
                    ));

                    // SETTINGS button
                    button_area.spawn((
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(120.0),
                                height: Val::Px(40.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: BackgroundColor(Color::rgb(0.5, 0.4, 0.6)),
                            ..default()
                        },
                        MenuButton(MenuAction::Settings),
                    ));

                    // HELP button
                    button_area.spawn((
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(120.0),
                                height: Val::Px(40.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: BackgroundColor(Color::rgb(0.4, 0.6, 0.5)),
                            ..default()
                        },
                        MenuButton(MenuAction::Help),
                    ));

                    // LEARNING button
                    button_area.spawn((
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(120.0),
                                height: Val::Px(40.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: BackgroundColor(Color::rgb(0.6, 0.5, 0.3)),
                            ..default()
                        },
                        MenuButton(MenuAction::Learning),
                    ));

                    // COMBAT button
                    button_area.spawn((
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(120.0),
                                height: Val::Px(40.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: BackgroundColor(Color::rgb(0.7, 0.3, 0.3)),
                            ..default()
                        },
                        MenuButton(MenuAction::Combat),
                    ));
                });

// Back button (for Settings, Help, Learning, Combat screens)
                panel.spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(40.0),
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    visibility: Visibility::Hidden,
                    ..default()
                })
                .insert(BackButtonRoot)
                .with_children(|back_area| {
                    back_area.spawn((
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(120.0),
                                height: Val::Px(35.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: BackgroundColor(Color::rgb(0.3, 0.3, 0.3)),
                            ..default()
                        },
                        MenuButton(MenuAction::Back),
                    ));
                });

                // Learning content panel - Interactive Planet Quiz
                panel.spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(100.0),
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::FlexStart,
                        align_items: AlignItems::Center,
                        row_gap: Val::Px(6.0),
                        overflow: Overflow::clip(),
                        ..default()
                    },
                    visibility: Visibility::Hidden,
                    ..default()
                })
                .insert(LearningContentRoot)
                .with_children(|content| {
                    // Quiz question text
                    content.spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Px(28.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: BackgroundColor(Color::rgb(0.6, 0.5, 0.3)),
                        ..default()
                    })
                    .insert(QuizQuestionText);

                    // Quiz options container
                    content.spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Px(30.0),
                            display: Display::Flex,
                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::Center,
                            column_gap: Val::Px(8.0),
                            ..default()
                        },
                        ..default()
                    })
                    .insert(QuizOptionsRoot)
                    .with_children(|options| {
                        for i in 0..4 {
                            options.spawn((
                                ButtonBundle {
                                    style: Style {
                                        width: Val::Px(180.0),
                                        height: Val::Px(28.0),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    background_color: BackgroundColor(Color::rgb(0.3, 0.3, 0.4)),
                                    ..default()
                                },
                                QuizOptionButton(i),
                            ));
                        }
                    });

                    // Quiz explanation / result area
                    content.spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Px(20.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: BackgroundColor(Color::rgba(0.1, 0.1, 0.1, 0.8)),
                        ..default()
                    })
                    .insert(QuizExplanationText);

                    // Quiz navigation buttons
                    content.spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Px(22.0),
                            display: Display::Flex,
                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::Center,
                            column_gap: Val::Px(8.0),
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|nav| {
                        // Next question button
                        nav.spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(100.0),
                                    height: Val::Px(20.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                background_color: BackgroundColor(Color::rgb(0.3, 0.5, 0.3)),
                                ..default()
                            },
                            QuizNavButton(QuizNavAction::Next),
                        ));
                        // Restart quiz button
                        nav.spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(100.0),
                                    height: Val::Px(20.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                background_color: BackgroundColor(Color::rgb(0.5, 0.3, 0.3)),
                                ..default()
                            },
                            QuizNavButton(QuizNavAction::Restart),
                        ));
                    });
                });

                // Combat content panel - Challenges
                panel.spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(100.0),
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::FlexStart,
                        align_items: AlignItems::Center,
                        row_gap: Val::Px(6.0),
                        overflow: Overflow::clip(),
                        ..default()
                    },
                    visibility: Visibility::Hidden,
                    ..default()
                })
                .insert(CombatContentRoot)
                .with_children(|content| {
                    // Challenge title
                    content.spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Px(24.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: BackgroundColor(Color::rgb(0.7, 0.3, 0.3)),
                        ..default()
                    })
                    .insert(ChallengeTitleText);

                    // Challenge description
                    content.spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Px(20.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: BackgroundColor(Color::rgba(0.1, 0.1, 0.1, 0.8)),
                        ..default()
                    })
                    .insert(ChallengeDescText);

                    // Challenge objective
                    content.spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Px(18.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: BackgroundColor(Color::rgba(0.15, 0.1, 0.1, 0.8)),
                        ..default()
                    })
                    .insert(ChallengeObjectiveText);

                    // Challenge navigation buttons
                    content.spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Px(22.0),
                            display: Display::Flex,
                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::Center,
                            column_gap: Val::Px(8.0),
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|nav| {
                        // Previous challenge
                        nav.spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(80.0),
                                    height: Val::Px(20.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                background_color: BackgroundColor(Color::rgb(0.3, 0.3, 0.5)),
                                ..default()
                            },
                            ChallengeNavButton(ChallengeNavAction::Previous),
                        ));
                        // Accept challenge
                        nav.spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(100.0),
                                    height: Val::Px(20.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                background_color: BackgroundColor(Color::rgb(0.3, 0.5, 0.3)),
                                ..default()
                            },
                            ChallengeNavButton(ChallengeNavAction::Accept),
                        ));
                        // Next challenge
                        nav.spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(80.0),
                                    height: Val::Px(20.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                background_color: BackgroundColor(Color::rgb(0.3, 0.3, 0.5)),
                                ..default()
                            },
                            ChallengeNavButton(ChallengeNavAction::Next),
                        ));
                    });

                    // Challenge log area
                    content.spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Px(16.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: BackgroundColor(Color::rgba(0.05, 0.05, 0.1, 0.8)),
                        ..default()
                    })
                    .insert(ChallengeLogText);
                });

                panel
                    .spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Px(24.0),
                            border: UiRect::all(Val::Px(2.0)),
                            ..default()
                        },
                        border_color: BorderColor(Color::rgb(0.3, 0.78, 0.92)),
                        background_color: BackgroundColor(Color::rgba(0.01, 0.03, 0.06, 0.96)),
                        ..default()
                    })
                    .with_children(|bar| {
                        bar.spawn((
                            NodeBundle {
                                style: Style {
                                    width: Val::Percent(0.0),
                                    height: Val::Percent(100.0),
                                    ..default()
                                },
                                background_color: BackgroundColor(Color::rgb(0.2, 0.86, 0.76)),
                                ..default()
                            },
                            LoadingBar,
                        ));
                    });
            });
        });

    // Touch controls
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Px(16.0),
                    bottom: Val::Px(16.0),
                    width: Val::Px(360.0),
                    height: Val::Px(150.0),
                    display: Display::Flex,
                    flex_wrap: FlexWrap::Wrap,
                    column_gap: Val::Px(8.0),
                    row_gap: Val::Px(8.0),
                    ..default()
                },
                background_color: BackgroundColor(Color::rgba(0.03, 0.08, 0.14, 0.7)),
                visibility: Visibility::Hidden,
                ..default()
            },
            ControlsRoot,
        ))
        .with_children(|root| {
            for (_label, action) in &[
                ("FWD", TouchAction::Forward),
                ("BACK", TouchAction::Backward),
                ("LEFT", TouchAction::Left),
                ("RIGHT", TouchAction::Right),
                ("UP", TouchAction::Up),
                ("DOWN", TouchAction::Down),
                ("YAW-", TouchAction::YawLeft),
                ("YAW+", TouchAction::YawRight),
                ("BOOST", TouchAction::Boost),
            ] {
                root.spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(108.0),
                            height: Val::Px(38.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: BackgroundColor(Color::rgba(0.22, 0.42, 0.56, 0.9)),
                        ..default()
                    },
                    TouchButton(*action),
                ));
            }
        });
}

/// Settings button actions for clickable settings.
#[derive(Component, Clone, Copy, PartialEq, Eq, Debug)]
pub struct SettingsButton(pub SettingsAction);

/// Settings button action types.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SettingsAction {
    GraphicsLow,
    GraphicsMedium,
    GraphicsHigh,
    GraphicsUltra,
    ToggleMinimap,
    ToggleEngineSound,
    CycleDifficulty,
    ToggleTutorial,
    ToggleCrosshair,
    ToggleLandingAssist,
    ToggleChallengeMode,
}

// === QUIZ UI COMPONENTS ===

/// Marker for the quiz question text panel.
#[derive(Component)]
pub struct QuizQuestionText;

/// Marker for the quiz options container.
#[derive(Component)]
pub struct QuizOptionsRoot;

/// Marker for a quiz option button.
#[derive(Component)]
pub struct QuizOptionButton(pub usize);

/// Marker for the quiz explanation text panel.
#[derive(Component)]
pub struct QuizExplanationText;

/// Quiz navigation button.
#[derive(Component)]
pub struct QuizNavButton(pub QuizNavAction);

/// Quiz navigation actions.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum QuizNavAction {
    Next,
    Restart,
}

// === CHALLENGE UI COMPONENTS ===

/// Marker for the challenge title text panel.
#[derive(Component)]
pub struct ChallengeTitleText;

/// Marker for the challenge description text panel.
#[derive(Component)]
pub struct ChallengeDescText;

/// Marker for the challenge objective text panel.
#[derive(Component)]
pub struct ChallengeObjectiveText;

/// Marker for the challenge log text panel.
#[derive(Component)]
pub struct ChallengeLogText;

/// Challenge navigation button.
#[derive(Component)]
pub struct ChallengeNavButton(pub ChallengeNavAction);

/// Challenge navigation actions.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ChallengeNavAction {
    Previous,
    Next,
    Accept,
}

/// Handles clickable menu button interactions.
fn handle_menu_buttons(
    interaction_query: Query<(&Interaction, &MenuButton), (With<Button>, Changed<Interaction>)>,
    mut flow: ResMut<Flow>,
    mut mission: ResMut<Mission>,
    mut quiz_state: ResMut<QuizState>,
    mut challenge_state: ResMut<ChallengeState>,
) {
    for (interaction, menu_btn) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_btn.0 {
                MenuAction::Start => {
                    flow.screen = ScreenMode::Loading;
                    flow.loading_progress = 0.0;
                    mission.last_event = "Boot sequence started.".to_string();
                }
                MenuAction::Settings => {
                    flow.screen = ScreenMode::Settings;
                }
                MenuAction::Help => {
                    flow.screen = ScreenMode::Help;
                }
                MenuAction::Learning => {
                    flow.screen = ScreenMode::Learning;
                    // Reset quiz state when entering learning
                    *quiz_state = QuizState::default();
                    mission.last_event = "📚 Planet Quiz: 50 questions about our solar system!".to_string();
                }
                MenuAction::Combat => {
                    flow.screen = ScreenMode::Combat;
                    // Reset challenge state when entering combat
                    *challenge_state = ChallengeState::default();
                    let challenges = get_all_challenges();
                    if let Some(first) = challenges.first() {
                        challenge_state.challenge_name = first.name.to_string();
                        challenge_state.challenge_description = first.description.to_string();
                        challenge_state.current_challenge = first.id;
                    }
                    mission.last_event = "⚔️ Challenge Mode: 10 space challenges await!".to_string();
                }
                MenuAction::Back => {
                    flow.screen = ScreenMode::Welcome;
                }
            }
        }
    }
}

/// Handles quiz button interactions.
fn handle_quiz_buttons(
    interaction_query: Query<(&Interaction, &QuizOptionButton), (With<Button>, Changed<Interaction>)>,
    nav_query: Query<(&Interaction, &QuizNavButton), (With<Button>, Changed<Interaction>)>,
    mut quiz_state: ResMut<QuizState>,
    mut mission: ResMut<Mission>,
) {
    // Handle option selection
    for (interaction, option) in &interaction_query {
        if *interaction == Interaction::Pressed && !quiz_state.answered && !quiz_state.quiz_finished {
            let idx = option.0;
            quiz_state.selected_index = Some(idx);
            quiz_state.answered = true;
            quiz_state.showing_explanation = true;

            let is_correct = if let Some(q) = quiz_state.questions.get(quiz_state.current_question) {
                idx == q.correct_index
            } else {
                false
            };

            if is_correct {
                let points = if let Some(q) = quiz_state.questions.get(quiz_state.current_question) {
                    q.difficulty.points()
                } else {
                    10
                };
                quiz_state.score += points;
                quiz_state.correct_count += 1;
                mission.last_event = format!("✅ Correct! +{} XP", points);
            } else {
                quiz_state.incorrect_count += 1;
                mission.last_event = "❌ Incorrect!".to_string();
            }
        }
    }

    // Handle navigation
    for (interaction, nav) in &nav_query {
        if *interaction == Interaction::Pressed {
            match nav.0 {
                QuizNavAction::Next => {
                    if quiz_state.answered {
                        if quiz_state.current_question + 1 < quiz_state.total_questions {
                            quiz_state.current_question += 1;
                            quiz_state.answered = false;
                            quiz_state.selected_index = None;
                            quiz_state.showing_explanation = false;
                        } else {
                            quiz_state.quiz_finished = true;
                            mission.last_event = format!(
                                "🏁 Quiz Complete! Score: {}/{} correct!",
                                quiz_state.correct_count,
                                quiz_state.total_questions
                            );
                        }
                    }
                }
                QuizNavAction::Restart => {
                    *quiz_state = QuizState::default();
                    mission.last_event = "🔄 Quiz restarted!".to_string();
                }
            }
        }
    }
}

/// Handles challenge button interactions.
fn handle_challenge_buttons(
    interaction_query: Query<(&Interaction, &ChallengeNavButton), (With<Button>, Changed<Interaction>)>,
    mut challenge_state: ResMut<ChallengeState>,
    mut mission: ResMut<Mission>,
) {
    for (interaction, nav) in &interaction_query {
        if *interaction == Interaction::Pressed {
            let challenges = get_all_challenges();
            if challenges.is_empty() {
                continue;
            }

            match nav.0 {
                ChallengeNavAction::Previous => {
                    if challenge_state.challenge_index > 0 {
                        challenge_state.challenge_index -= 1;
                    } else {
                        challenge_state.challenge_index = challenges.len() - 1;
                    }
                    let c = &challenges[challenge_state.challenge_index];
                    challenge_state.challenge_name = c.name.to_string();
                    challenge_state.challenge_description = c.description.to_string();
                    challenge_state.current_challenge = c.id;
                    challenge_state.completed = false;
                    challenge_state.reward_earned = false;
                    challenge_state.progress = 0;
                    challenge_state.goal = 0;
                    mission.last_event = format!("📋 Challenge: {}", c.name);
                }
                ChallengeNavAction::Next => {
                    challenge_state.challenge_index = (challenge_state.challenge_index + 1) % challenges.len();
                    let c = &challenges[challenge_state.challenge_index];
                    challenge_state.challenge_name = c.name.to_string();
                    challenge_state.challenge_description = c.description.to_string();
                    challenge_state.current_challenge = c.id;
                    challenge_state.completed = false;
                    challenge_state.reward_earned = false;
                    challenge_state.progress = 0;
                    challenge_state.goal = 0;
                    mission.last_event = format!("📋 Challenge: {}", c.name);
                }
                ChallengeNavAction::Accept => {
                    if let Some(c) = challenges.get(challenge_state.challenge_index) {
                        challenge_state.active = true;
                        challenge_state.completed = false;
                        challenge_state.reward_earned = false;
                        challenge_state.progress = 0;
                        challenge_state.goal = c.reward_xp;
                        challenge_state.challenge_log.push(
                            format!("✅ Accepted: {} (Reward: {} XP)", c.name, c.reward_xp)
                        );
                        mission.last_event = format!("🎯 Challenge Accepted: {}!", c.name);
                    }
                }
            }
        }
    }
}

/// Handles clickable settings button interactions.
fn handle_settings_buttons(
    interaction_query: Query<(&Interaction, &SettingsButton), (With<Button>, Changed<Interaction>)>,
    mut settings: ResMut<Settings>,
    mut mission: ResMut<Mission>,
) {
    for (interaction, settings_btn) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match settings_btn.0 {
                SettingsAction::GraphicsLow => {
                    settings.graphics_quality = GraphicsQuality::Low;
                }
                SettingsAction::GraphicsMedium => {
                    settings.graphics_quality = GraphicsQuality::Medium;
                }
                SettingsAction::GraphicsHigh => {
                    settings.graphics_quality = GraphicsQuality::High;
                }
                SettingsAction::GraphicsUltra => {
                    settings.graphics_quality = GraphicsQuality::Ultra;
                }
                SettingsAction::ToggleMinimap => {
                    settings.minimap_enabled = !settings.minimap_enabled;
                }
                SettingsAction::ToggleEngineSound => {
                    settings.engine_sound = !settings.engine_sound;
                }
                SettingsAction::CycleDifficulty => {
                    settings.difficulty = settings.difficulty.next();
                    mission.last_event = format!("Difficulty: {}", settings.difficulty.label());
                }
                SettingsAction::ToggleTutorial => {
                    settings.tutorial_hints = !settings.tutorial_hints;
                }
                SettingsAction::ToggleCrosshair => {
                    settings.crosshair_enabled = !settings.crosshair_enabled;
                }
                SettingsAction::ToggleLandingAssist => {
                    settings.auto_landing_assist = !settings.auto_landing_assist;
                    mission.last_event = format!(
                        "Landing assist {}",
                        if settings.auto_landing_assist { "ON" } else { "OFF" }
                    );
                }
                SettingsAction::ToggleChallengeMode => {
                    settings.challenge_mode = !settings.challenge_mode;
                    mission.last_event = format!(
                        "Challenge mode {}",
                        if settings.challenge_mode { "ON" } else { "OFF" }
                    );
                }
            }
        }
    }
}

/// Keyboard-based menu navigation.
fn menu_flow_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut flow: ResMut<Flow>,
    mut mission: ResMut<Mission>,
    mut settings: ResMut<Settings>,
    mut quiz_state: ResMut<QuizState>,
    mut challenge_state: ResMut<ChallengeState>,
) {
    match flow.screen {
        ScreenMode::Welcome => {
            if keyboard.just_pressed(KeyCode::Enter) {
                flow.screen = ScreenMode::Loading;
                flow.loading_progress = 0.0;
                mission.last_event = "Boot sequence started.".to_string();
            }
            if keyboard.just_pressed(KeyCode::KeyS) {
                flow.screen = ScreenMode::Settings;
            }
            if keyboard.just_pressed(KeyCode::KeyH) {
                flow.screen = ScreenMode::Help;
            }
            if keyboard.just_pressed(KeyCode::KeyL) {
                flow.screen = ScreenMode::Learning;
                *quiz_state = QuizState::default();
                mission.last_event = "📚 Planet Quiz: 50 questions about our solar system!".to_string();
            }
            if keyboard.just_pressed(KeyCode::KeyC) {
                flow.screen = ScreenMode::Combat;
                *challenge_state = ChallengeState::default();
                let challenges = get_all_challenges();
                if let Some(first) = challenges.first() {
                    challenge_state.challenge_name = first.name.to_string();
                    challenge_state.challenge_description = first.description.to_string();
                    challenge_state.current_challenge = first.id;
                }
                mission.last_event = "⚔️ Challenge Mode: 10 space challenges await!".to_string();
            }
        }
        ScreenMode::Settings | ScreenMode::Help => {
            if flow.screen == ScreenMode::Settings {
                if keyboard.just_pressed(KeyCode::Digit1) {
                    settings.graphics_quality = GraphicsQuality::Low;
                }
                if keyboard.just_pressed(KeyCode::Digit2) {
                    settings.graphics_quality = GraphicsQuality::Medium;
                }
                if keyboard.just_pressed(KeyCode::Digit3) {
                    settings.graphics_quality = GraphicsQuality::High;
                }
                if keyboard.just_pressed(KeyCode::Digit4) {
                    settings.graphics_quality = GraphicsQuality::Ultra;
                }
                if keyboard.just_pressed(KeyCode::KeyQ) {
                    settings.star_density = (settings.star_density - 0.1).clamp(0.5, 2.0);
                }
                if keyboard.just_pressed(KeyCode::KeyW) {
                    settings.star_density = (settings.star_density + 0.1).clamp(0.5, 2.0);
                }
                if keyboard.just_pressed(KeyCode::KeyA) {
                    settings.planet_detail = settings.planet_detail.next();
                }
                if keyboard.just_pressed(KeyCode::KeyS) {
                    settings.planet_detail = settings.planet_detail.next();
                }
                if keyboard.just_pressed(KeyCode::KeyZ) {
                    settings.camera_smoothing = (settings.camera_smoothing - 0.1).clamp(0.0, 1.0);
                }
                if keyboard.just_pressed(KeyCode::KeyX) {
                    settings.camera_smoothing = (settings.camera_smoothing + 0.1).clamp(0.0, 1.0);
                }
                if keyboard.just_pressed(KeyCode::KeyM) {
                    settings.minimap_enabled = !settings.minimap_enabled;
                }
                if keyboard.any_pressed([KeyCode::AltLeft, KeyCode::AltRight]) {
                    if keyboard.just_pressed(KeyCode::ArrowUp) {
                        settings.master_volume = (settings.master_volume + 0.1).clamp(0.0, 1.0);
                    }
                    if keyboard.just_pressed(KeyCode::ArrowDown) {
                        settings.master_volume = (settings.master_volume - 0.1).clamp(0.0, 1.0);
                    }
                    if keyboard.just_pressed(KeyCode::ArrowLeft) {
                        settings.music_volume = (settings.music_volume - 0.1).clamp(0.0, 1.0);
                    }
                    if keyboard.just_pressed(KeyCode::ArrowRight) {
                        settings.music_volume = (settings.music_volume + 0.1).clamp(0.0, 1.0);
                    }
                }
                if keyboard.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]) {
                    if keyboard.just_pressed(KeyCode::ArrowUp) {
                        settings.sfx_volume = (settings.sfx_volume + 0.1).clamp(0.0, 1.0);
                    }
                    if keyboard.just_pressed(KeyCode::ArrowDown) {
                        settings.sfx_volume = (settings.sfx_volume - 0.1).clamp(0.0, 1.0);
                    }
                }
                if keyboard.just_pressed(KeyCode::KeyE) {
                    settings.engine_sound = !settings.engine_sound;
                }
                if keyboard.just_pressed(KeyCode::KeyD) {
                    settings.difficulty = settings.difficulty.next();
                    mission.last_event = format!("Difficulty: {}", settings.difficulty.label());
                }
                if keyboard.just_pressed(KeyCode::KeyT) {
                    settings.tutorial_hints = !settings.tutorial_hints;
                }
                if keyboard.just_pressed(KeyCode::KeyC) {
                    settings.crosshair_enabled = !settings.crosshair_enabled;
                }
                if keyboard.just_pressed(KeyCode::KeyL) {
                    settings.auto_landing_assist = !settings.auto_landing_assist;
                    mission.last_event = format!(
                        "Landing assist {}",
                        if settings.auto_landing_assist {
                            "ON"
                        } else {
                            "OFF"
                        }
                    );
                }
                if keyboard.just_pressed(KeyCode::KeyG) {
                    settings.challenge_mode = !settings.challenge_mode;
                    mission.last_event = format!(
                        "Challenge mode {}",
                        if settings.challenge_mode { "ON" } else { "OFF" }
                    );
                }
                if keyboard.just_pressed(KeyCode::KeyU) {
                    settings.auto_save_interval = match settings.auto_save_interval as u32 {
                        0 => 30,
                        30 => 60,
                        60 => 120,
                        120 => 0,
                        _ => 60,
                    } as f32;
                }
                if keyboard.just_pressed(KeyCode::KeyF) {
                    settings.input_sensitivity = (settings.input_sensitivity - 0.1).clamp(0.6, 2.0);
                }
                if keyboard.just_pressed(KeyCode::KeyH) {
                    settings.input_sensitivity = (settings.input_sensitivity + 0.1).clamp(0.6, 2.0);
                }
                if keyboard.just_pressed(KeyCode::ArrowUp) {
                    settings.simulation_speed = (settings.simulation_speed + 0.1).clamp(0.4, 3.0);
                }
                if keyboard.just_pressed(KeyCode::ArrowDown) {
                    settings.simulation_speed = (settings.simulation_speed - 0.1).clamp(0.4, 3.0);
                }
                if keyboard.just_pressed(KeyCode::ArrowRight) {
                    settings.ship_scale = (settings.ship_scale + 0.02).clamp(0.1, 0.6);
                }
                if keyboard.just_pressed(KeyCode::ArrowLeft) {
                    settings.ship_scale = (settings.ship_scale - 0.02).clamp(0.1, 0.6);
                }
            }
            if keyboard.just_pressed(KeyCode::Escape) {
                flow.screen = ScreenMode::Welcome;
            }
            if keyboard.just_pressed(KeyCode::Enter) {
                flow.screen = ScreenMode::Loading;
                flow.loading_progress = 0.0;
            }
        }
        ScreenMode::Playing => {
            if keyboard.just_pressed(KeyCode::F1) {
                flow.screen = ScreenMode::Settings;
                mission.last_event = "Settings opened.".to_string();
            }
            if keyboard.just_pressed(KeyCode::F2) {
                flow.screen = ScreenMode::Help;
            }
            if keyboard.just_pressed(KeyCode::Escape) {
                flow.screen = ScreenMode::Welcome;
            }
        }
        ScreenMode::Learning | ScreenMode::Combat => {
            if keyboard.just_pressed(KeyCode::Escape) {
                flow.screen = ScreenMode::Welcome;
            }
            // Quiz keyboard shortcuts
            if flow.screen == ScreenMode::Learning && !quiz_state.quiz_finished {
                if keyboard.just_pressed(KeyCode::Digit1) { try_answer_quiz(0, &mut quiz_state, &mut mission); }
                if keyboard.just_pressed(KeyCode::Digit2) { try_answer_quiz(1, &mut quiz_state, &mut mission); }
                if keyboard.just_pressed(KeyCode::Digit3) { try_answer_quiz(2, &mut quiz_state, &mut mission); }
                if keyboard.just_pressed(KeyCode::Digit4) { try_answer_quiz(3, &mut quiz_state, &mut mission); }
                if keyboard.just_pressed(KeyCode::Enter) && quiz_state.answered {
                    if quiz_state.current_question + 1 < quiz_state.total_questions {
                        quiz_state.current_question += 1;
                        quiz_state.answered = false;
                        quiz_state.selected_index = None;
                        quiz_state.showing_explanation = false;
                    } else {
                        quiz_state.quiz_finished = true;
                        mission.last_event = format!(
                            "🏁 Quiz Complete! Score: {}/{} correct!",
                            quiz_state.correct_count,
                            quiz_state.total_questions
                        );
                    }
                }
                if keyboard.just_pressed(KeyCode::KeyR) {
                    *quiz_state = QuizState::default();
                    mission.last_event = "🔄 Quiz restarted!".to_string();
                }
            }
            // Challenge keyboard shortcuts
            if flow.screen == ScreenMode::Combat {
                let challenges = get_all_challenges();
                if !challenges.is_empty() {
                    if keyboard.just_pressed(KeyCode::ArrowLeft) {
                        if challenge_state.challenge_index > 0 {
                            challenge_state.challenge_index -= 1;
                        } else {
                            challenge_state.challenge_index = challenges.len() - 1;
                        }
                        let c = &challenges[challenge_state.challenge_index];
                        challenge_state.challenge_name = c.name.to_string();
                        challenge_state.challenge_description = c.description.to_string();
                        challenge_state.current_challenge = c.id;
                        challenge_state.completed = false;
                        challenge_state.reward_earned = false;
                        challenge_state.progress = 0;
                        challenge_state.goal = 0;
                        mission.last_event = format!("📋 Challenge: {}", c.name);
                    }
                    if keyboard.just_pressed(KeyCode::ArrowRight) {
                        challenge_state.challenge_index = (challenge_state.challenge_index + 1) % challenges.len();
                        let c = &challenges[challenge_state.challenge_index];
                        challenge_state.challenge_name = c.name.to_string();
                        challenge_state.challenge_description = c.description.to_string();
                        challenge_state.current_challenge = c.id;
                        challenge_state.completed = false;
                        challenge_state.reward_earned = false;
                        challenge_state.progress = 0;
                        challenge_state.goal = 0;
                        mission.last_event = format!("📋 Challenge: {}", c.name);
                    }
                    if keyboard.just_pressed(KeyCode::Enter) {
                        if let Some(c) = challenges.get(challenge_state.challenge_index) {
                            challenge_state.active = true;
                            challenge_state.completed = false;
                            challenge_state.reward_earned = false;
                            challenge_state.progress = 0;
                            challenge_state.goal = c.reward_xp;
                            challenge_state.challenge_log.push(
                                format!("✅ Accepted: {} (Reward: {} XP)", c.name, c.reward_xp)
                            );
                            mission.last_event = format!("🎯 Challenge Accepted: {}!", c.name);
                        }
                    }
                }
            }
        }
        ScreenMode::Loading => {}
    }
}

/// Helper to process a quiz answer attempt.
fn try_answer_quiz(idx: usize, quiz_state: &mut QuizState, mission: &mut Mission) {
    if quiz_state.answered || quiz_state.quiz_finished {
        return;
    }
    quiz_state.selected_index = Some(idx);
    quiz_state.answered = true;
    quiz_state.showing_explanation = true;

    let is_correct = if let Some(q) = quiz_state.questions.get(quiz_state.current_question) {
        idx == q.correct_index
    } else {
        false
    };

    if is_correct {
        let points = if let Some(q) = quiz_state.questions.get(quiz_state.current_question) {
            q.difficulty.points()
        } else {
            10
        };
        quiz_state.score += points;
        quiz_state.correct_count += 1;
        mission.last_event = format!("✅ Correct! +{} XP", points);
    } else {
        quiz_state.incorrect_count += 1;
        mission.last_event = "❌ Incorrect!".to_string();
    }
}

/// Advances the loading bar.
fn advance_loading(time: Res<Time>, mut flow: ResMut<Flow>, mut mission: ResMut<Mission>) {
    if flow.screen != ScreenMode::Loading {
        return;
    }
    flow.loading_progress =
        (flow.loading_progress + time.delta_seconds() / LOADING_DURATION).clamp(0.0, 1.0);
    if flow.loading_progress >= 1.0 {
        flow.screen = ScreenMode::Playing;
        mission.last_event = "Mission live!".to_string();
    }
}

/// Syncs visibility of UI elements based on current screen.
fn sync_visibility(
    flow: Res<Flow>,
    settings: Res<Settings>,
    mut vis_sets: ParamSet<(
        Query<&mut Visibility, With<HudRoot>>,
        Query<&mut Visibility, With<MenuRoot>>,
        Query<&mut Visibility, With<ControlsRoot>>,
        Query<&mut Visibility, With<SettingsButtonRoot>>,
        Query<&mut Visibility, With<BackButtonRoot>>,
        Query<&mut Visibility, With<LearningContentRoot>>,
        Query<&mut Visibility, With<CombatContentRoot>>,
    )>,
) {
    let show_hud = flow.is_playing();
    let show_settings_buttons = flow.screen == ScreenMode::Settings;
    let show_back_button = matches!(
        flow.screen,
        ScreenMode::Settings | ScreenMode::Help | ScreenMode::Learning | ScreenMode::Combat
    );
    let show_learning = flow.screen == ScreenMode::Learning;
    let show_combat = flow.screen == ScreenMode::Combat;
    for mut v in &mut vis_sets.p0() {
        *v = if show_hud {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
    for mut v in &mut vis_sets.p1() {
        *v = if show_hud {
            Visibility::Hidden
        } else {
            Visibility::Visible
        };
    }
    for mut v in &mut vis_sets.p2() {
        *v = if show_hud && settings.show_touch_controls {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
    for mut v in &mut vis_sets.p3() {
        *v = if show_settings_buttons {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
    for mut v in &mut vis_sets.p4() {
        *v = if show_back_button {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
    for mut v in &mut vis_sets.p5() {
        *v = if show_learning {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
    for mut v in &mut vis_sets.p6() {
        *v = if show_combat {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

/// Updates menu colors based on current screen.
fn update_menu(
    flow: Res<Flow>,
    _settings: Res<Settings>,
    quiz_state: Res<QuizState>,
    challenge_state: Res<ChallengeState>,
    mut param_set: ParamSet<(
        Query<(&mut BackgroundColor, &mut Style), (With<MenuTitle>, Without<MenuBody>)>,
        Query<(&mut BackgroundColor, &mut Style), (With<MenuBody>, Without<MenuTitle>)>,
        Query<&mut Style, With<LoadingBar>>,
    )>,
    mut quiz_question_q: Query<&mut BackgroundColor, With<QuizQuestionText>>,
    mut quiz_explanation_q: Query<&mut BackgroundColor, With<QuizExplanationText>>,
    mut challenge_title_q: Query<&mut BackgroundColor, With<ChallengeTitleText>>,
    mut challenge_desc_q: Query<&mut BackgroundColor, With<ChallengeDescText>>,
    mut challenge_obj_q: Query<&mut BackgroundColor, With<ChallengeObjectiveText>>,
    mut challenge_log_q: Query<&mut BackgroundColor, With<ChallengeLogText>>,
) {
    for (mut bg, mut style) in &mut param_set.p0() {
        match flow.screen {
            ScreenMode::Welcome => {
                *bg = BackgroundColor(Color::rgb(0.2, 0.7, 0.8));
                style.height = Val::Px(60.0);
            }
            ScreenMode::Loading => {
                *bg = BackgroundColor(Color::rgb(0.3, 0.5, 0.7));
                style.height = Val::Px(50.0);
            }
            ScreenMode::Settings => {
                *bg = BackgroundColor(Color::rgb(0.5, 0.4, 0.6));
                style.height = Val::Px(50.0);
            }
            ScreenMode::Help => {
                *bg = BackgroundColor(Color::rgb(0.4, 0.6, 0.5));
                style.height = Val::Px(50.0);
            }
            ScreenMode::Learning => {
                *bg = BackgroundColor(Color::rgb(0.6, 0.5, 0.3));
                style.height = Val::Px(50.0);
            }
            ScreenMode::Combat => {
                *bg = BackgroundColor(Color::rgb(0.7, 0.3, 0.3));
                style.height = Val::Px(50.0);
            }
            ScreenMode::Playing => {
                *bg = BackgroundColor(Color::rgb(0.3, 0.3, 0.3));
                style.height = Val::Px(30.0);
            }
        }
    }

    for (mut bg, _) in &mut param_set.p1() {
        *bg = BackgroundColor(Color::rgba(0.1, 0.2, 0.3, 0.8));
    }

    for mut s in &mut param_set.p2() {
        s.width = if flow.screen == ScreenMode::Loading {
            Val::Percent(flow.loading_progress * 100.0)
        } else {
            Val::Percent(0.0)
        };
    }

    // Update quiz panel colors based on state
    if flow.screen == ScreenMode::Learning {
        for mut bg in &mut quiz_question_q {
            if quiz_state.quiz_finished {
                *bg = BackgroundColor(Color::rgb(0.2, 0.6, 0.3));
            } else if quiz_state.answered {
                *bg = BackgroundColor(Color::rgb(0.5, 0.4, 0.2));
            } else {
                *bg = BackgroundColor(Color::rgb(0.6, 0.5, 0.3));
            }
        }
        for mut bg in &mut quiz_explanation_q {
            if quiz_state.showing_explanation {
                let is_correct = quiz_state.selected_index.map_or(false, |idx| {
                    quiz_state.questions.get(quiz_state.current_question).map_or(false, |q| idx == q.correct_index)
                });
                *bg = if is_correct {
                    BackgroundColor(Color::rgba(0.1, 0.3, 0.1, 0.8))
                } else {
                    BackgroundColor(Color::rgba(0.3, 0.1, 0.1, 0.8))
                };
            } else {
                *bg = BackgroundColor(Color::rgba(0.1, 0.1, 0.1, 0.8));
            }
        }
    }

    // Update challenge panel colors
    if flow.screen == ScreenMode::Combat {
        for mut bg in &mut challenge_title_q {
            if challenge_state.active && !challenge_state.completed {
                *bg = BackgroundColor(Color::rgb(0.3, 0.6, 0.3));
            } else if challenge_state.completed {
                *bg = BackgroundColor(Color::rgb(0.2, 0.5, 0.2));
            } else {
                *bg = BackgroundColor(Color::rgb(0.7, 0.3, 0.3));
            }
        }
        for mut bg in &mut challenge_desc_q {
            *bg = BackgroundColor(Color::rgba(0.1, 0.1, 0.1, 0.8));
        }
        for mut bg in &mut challenge_obj_q {
            if challenge_state.active {
                *bg = BackgroundColor(Color::rgba(0.15, 0.2, 0.1, 0.8));
            } else {
                *bg = BackgroundColor(Color::rgba(0.15, 0.1, 0.1, 0.8));
            }
        }
        for mut bg in &mut challenge_log_q {
            *bg = BackgroundColor(Color::rgba(0.05, 0.05, 0.1, 0.8));
        }
    }
}

/// Starts background music if available.
fn start_music(
    flow: Res<Flow>,
    mut music: ResMut<Music>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    if !flow.is_playing() || music.started {
        return;
    }
    if std::path::Path::new("assets/audio/space_theme.ogg").exists() {
        commands.spawn(AudioBundle {
            source: asset_server.load("audio/space_theme.ogg"),
            settings: PlaybackSettings::LOOP,
        });
        music.started = true;
    }
}

/// Search input handler.
fn handle_search(
    flow: Res<Flow>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut char_events: EventReader<ReceivedCharacter>,
    mut search: ResMut<Search>,
    mut target: ResMut<Target>,
    mut mission: ResMut<Mission>,
) {
    if !flow.is_playing() {
        search.active = false;
        return;
    }
    if keyboard.just_pressed(KeyCode::Slash) {
        search.active = true;
        search.query.clear();
    }
    if !search.active {
        return;
    }

    for event in char_events.read() {
        for ch in event.char.chars() {
            if ch.is_ascii_alphanumeric() || ch == ' ' {
                search.query.push(ch);
            }
        }
    }
    if keyboard.just_pressed(KeyCode::Backspace) {
        search.query.pop();
    }
    if keyboard.just_pressed(KeyCode::Enter) {
        if let Some(kind) = PlanetKind::from_query(&search.query) {
            target.target = kind;
            mission.last_event = format!("Search: {}", kind.display_name());
        } else {
            mission.last_event = format!("No match: '{}'", search.query);
        }
        search.active = false;
    }
    if keyboard.just_pressed(KeyCode::Escape) {
        search.active = false;
    }
}

/// Updates virtual touch controls state.
fn update_virtual_controls(
    mut controls: ResMut<VirtualControls>,
    buttons: Query<(&Interaction, &TouchButton), With<Button>>,
) {
    *controls = VirtualControls::default();
    for (interaction, touch) in &buttons {
        if *interaction != Interaction::Pressed {
            continue;
        }
        match touch.0 {
            TouchAction::Forward => controls.forward = true,
            TouchAction::Backward => controls.backward = true,
            TouchAction::Left => controls.left = true,
            TouchAction::Right => controls.right = true,
            TouchAction::Up => controls.up = true,
            TouchAction::Down => controls.down = true,
            TouchAction::YawLeft => controls.yaw_left = true,
            TouchAction::YawRight => controls.yaw_right = true,
            TouchAction::Boost => controls.boost = true,
        }
    }
}

/// Updates the main HUD panel colors.
fn update_hud(
    target: Res<Target>,
    scanner: Res<Scanner>,
    _flight: Res<Flight>,
    _health: Res<Health>,
    _fuel: Res<Fuel>,
    _mission: Res<Mission>,
    camera_state: Res<CameraState>,
    search: Res<Search>,
    _settings: Res<Settings>,
    ship_query: Query<&Transform, With<PlayerShip>>,
    planets: Query<(&Transform, &CelestialBody)>,
    mut params: ParamSet<(
        Query<(&mut BackgroundColor, &mut Style), With<HudText>>,
        Query<(&mut BackgroundColor, &mut Style), With<FocusInfo>>,
    )>,
    focus: Res<FocusedInfo>,
) {
    let Ok(ship) = ship_query.get_single() else {
        return;
    };
    let _target_name = target.target.display_name();
    let mut _target_distance = 0.0;
    for (pt, body) in &planets {
        if body.kind == target.target {
            _target_distance = ship.translation.distance(pt.translation);
            break;
        }
    }
    let scan_pct = (scanner.progress * 100.0).round();
    let _scan_state = if scanner.active { "ACTIVE" } else { "STANDBY" };
    let _cam_label = if camera_state.cockpit {
        "COCKPIT"
    } else if camera_state.overview {
        "OVERVIEW"
    } else {
        "CHASE"
    };
    let _search_lbl = if search.active {
        format!("SEARCH: {}", search.query)
    } else {
        "SEARCH: /".into()
    };

    // Update HUD panel color based on status
    for (mut bg, _) in &mut params.p0() {
        let intensity = (scan_pct / 100.0).min(1.0);
        *bg = BackgroundColor(Color::rgb(
            0.02 + intensity * 0.1,
            0.05 + intensity * 0.1,
            0.1 + intensity * 0.15,
        ));
    }

    // Update focus panel
    for (mut bg, _) in &mut params.p1() {
        if focus.message.is_empty() {
            *bg = BackgroundColor(Color::rgba(0.1, 0.08, 0.02, 0.8));
        } else {
            *bg = BackgroundColor(Color::rgb(0.15, 0.12, 0.03));
        }
    }
}

/// Updates the minimap panel colors.
fn update_minimap(
    _target: Res<Target>,
    _ship_query: Query<&Transform, With<PlayerShip>>,
    _planets: Query<(&Transform, &CelestialBody)>,
    mut minimap_q: Query<(&mut BackgroundColor, &mut Style), With<MinimapText>>,
) {
    // Update minimap panel appearance
    for (mut bg, _) in &mut minimap_q {
        *bg = BackgroundColor(Color::rgba(0.03, 0.08, 0.15, 0.8));
    }
}

/// Updates the progression panel colors.
fn update_progression(mission: Res<Mission>, mut prog_q: Query<(&mut BackgroundColor, &mut Style), With<ProgressionText>>) {
    for (mut bg, _) in &mut prog_q {
        let tier_color = match mission.tier {
            0..=1 => Color::rgb(0.05, 0.1, 0.15),
            2..=3 => Color::rgb(0.08, 0.15, 0.1),
            4..=5 => Color::rgb(0.15, 0.12, 0.05),
            _ => Color::rgb(0.15, 0.08, 0.15),
        };
        *bg = BackgroundColor(tier_color);
    }
}

/// Pulsing color effect on HUD header.
fn pulse_hud_color(time: Res<Time>, mut header_q: Query<&mut BackgroundColor, With<HeaderText>>) {
    let pulse = (time.elapsed_seconds() * 0.5).sin() * 0.15 + 0.85;
    for mut bg in &mut header_q {
        *bg = BackgroundColor(Color::rgb(0.05 * pulse, 0.15 * pulse, 0.2 * pulse));
    }
}