use bevy::{
    app::AppExit,
    audio::{AudioBundle, AudioSink, AudioSource, PlaybackMode, PlaybackSettings},
    prelude::*,
};
use std::collections::HashMap;

// Import workspace crate plugins - only use non-UI crates
// UiPlugin is excluded to avoid conflicts with main.rs's built-in UI system
use galactic_explorer_assets::AssetPipelinePlugin;
use galactic_explorer_data::DataPlugin;
use galactic_explorer_physics::PhysicsPlugin;
use galactic_explorer_rendering::RenderingPlugin;
// core types for components like StarDome, Orbiting, etc.
use galactic_explorer_core::prelude::*;
use galactic_explorer_rendering::MainCamera;

// ── STATES ─
#[derive(States, Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub enum AppState {
    #[default] Loading,
    Exploration,
    PlanetInfo,
    Settings,
    GeoMap,
    Educational,
    Quiz,
    Challenges,
    Combat,
    Landing,
}

#[derive(Resource, Clone)]
pub struct Encyclopedia {
    pub data: HashMap<String, PlanetData>,
}

#[derive(Clone)]
pub struct PlanetData {
    pub description: String,
    pub mass: f32,
    pub radius: f32,
    pub day_length: f32,
    pub surface_temp: String,
    pub moons: u32,
    pub has_geo_map: bool,
    pub continent_info: Vec<ContinentInfo>,
    pub educational_facts: Vec<String>,
    pub quiz_questions: Vec<QuizQuestion>,
    pub fun_facts: Vec<String>,
    pub missions: Vec<Mission>,
}

#[derive(Clone)]
pub struct Mission {
    pub name: String,
    pub year: String,
    pub agency: String,
    pub description: String,
}

#[derive(Clone)]
pub struct QuizQuestion {
    pub question: String,
    pub options: Vec<String>,
    pub correct_index: usize,
    pub explanation: String,
}

#[derive(Clone)]
pub struct ContinentInfo {
    pub name: String,
    pub area: String,
    pub population: String,
    pub countries: u32,
    pub description: String,
}

#[derive(Resource)]
pub struct AppSettings {
    pub ui_scale: f32,
    pub graphics_quality: String,
    pub night_sky_enabled: bool,
    pub show_planet_orbits: bool,
    pub show_geo_map: bool,
    pub show_educational_tips: bool,
    pub show_planet_labels: bool,
    pub language: String,
    pub audio_enabled: bool,
    pub music_volume: f32,
    pub sfx_volume: f32,
    pub fullscreen: bool,
    pub vsync: bool,
    pub antialiasing: bool,
    pub show_fps: bool,
    pub auto_rotate: bool,
    pub combat_difficulty: String,
    pub shield_level: u32,
}

#[derive(Resource)]
pub struct PlayerShip {
    pub health: f32,
    pub max_health: f32,
    pub shield: f32,
    pub max_shield: f32,
    pub score: u32,
    pub kills: u32,
    pub level: u32,
    pub speed: f32,
}

#[derive(Resource)]
pub struct CombatState {
    pub enemies: Vec<EnemyData>,
    pub wave: u32,
    pub active: bool,
    pub spawn_timer: f32,
    pub enemy_count: u32,
    pub has_won: bool,
    pub has_lost: bool,
    pub total_to_kill: u32,
    pub fire_cooldown: f32,
    pub fire_timer: f32,
}

pub struct EnemyData {
    pub name: String,
    pub health: f32,
    pub speed: f32,
    pub damage: f32,
    pub reward_score: u32,
}

#[derive(Resource, Default)]
pub struct SearchQuery {
    pub text: String,
    pub active: bool,
}

#[derive(Resource, Default)]
pub struct SelectedPlanet {
    pub name: String,
    pub index: usize,
}

#[derive(Resource, Default)]
pub struct LoadingProgress {
    pub progress: f32,
    pub phase: String,
}

#[derive(Resource, Default)]
pub struct QuizState {
    pub active: bool,
    pub current_planet: String,
    pub current_question: usize,
    pub score: u32,
    pub total_questions: u32,
    pub answered: bool,
    pub selected_answer: usize,
    pub correct_count: u32,
    pub wrong_count: u32,
    pub finished: bool,
    pub used_50_global_questions: bool,
    pub next_pressed_frame: bool,
}

#[derive(Resource, Default)]
pub struct ChallengeState {
    pub active: bool,
    pub current_challenge: usize,
    pub completed_challenges: Vec<usize>,
    pub time_remaining: f32,
    pub total_time: f32,
}

#[derive(Resource)]
pub struct MusicHandle {
    pub handle: Handle<AudioSource>,
}

#[derive(Resource)]
pub struct SfxHandles {
    pub button_click: Handle<AudioSource>,
    pub close: Handle<AudioSource>,
    pub quiz_success: Handle<AudioSource>,
    pub quiz_fail: Handle<AudioSource>,
    pub lose: Handle<AudioSource>,
}

#[derive(Resource, Clone)]
pub struct Challenge {
    pub title: String,
    pub description: String,
    pub objective: String,
    pub time_limit: f32,
    pub reward: String,
    pub difficulty: String,
}

#[derive(Resource, Clone, Default)]
pub struct ChallengesResource {
    pub challenges: Vec<Challenge>,
}

#[derive(Resource)]
pub struct SimulationTime {
    pub current_date: String,
    pub current_time: String,
    pub speed_multiplier: f32,
}

// ── TRANSLATIONS RESOURCE ──
#[derive(Resource, Clone)]
pub struct Translations {
    pub map: HashMap<String, HashMap<String, String>>,
}

impl Translations {
    pub fn get(&self, lang: &str, key: &str) -> String {
        if let Some(lang_map) = self.map.get(lang) {
            if let Some(text) = lang_map.get(key) {
                return text.clone();
            }
        }
        if let Some(en_map) = self.map.get("English") {
            if let Some(text) = en_map.get(key) {
                return text.clone();
            }
        }
        key.to_string()
    }
}

fn build_translations() -> Translations {
    let mut map: HashMap<String, HashMap<String, String>> = HashMap::new();
    let t = |k: &str, v: &str| (k.to_string(), v.to_string());
    let en: HashMap<String, String> = vec![
        t("settings_title", "SETTINGS"),
        t("graphics_section", "--- GRAPHICS ---"),
        t("display_section", "--- DISPLAY ---"),
        t("audio_section", "--- AUDIO ---"),
        t("language_section", "--- LANGUAGE ---"),
        t("combat_section", "--- COMBAT ---"),
        t("close", "Close"),
        t("night_sky", "Night Sky"),
        t("quality", "Quality"),
        t("fullscreen", "Fullscreen"),
        t("show_orbits", "Show Orbits"),
        t("planet_labels", "Planet Labels"),
        t("show_fps", "Show FPS"),
        t("auto_rotate", "Auto Rotate"),
        t("audio", "Audio"),
        t("music", "Music"),
        t("sfx", "SFX"),
        t("language_label", "Language"),
        t("difficulty", "Difficulty"),
        t("shield", "Shield"),
        t("on", "ON"),
        t("off", "OFF"),
        t("settings_toolbar", "SETTINGS"),
        t("geo_map_toolbar", "GEO MAP"),
        t("planets_toolbar", "PLANETS"),
        t("orbits_toolbar", "ORBITS"),
        t("night_toolbar", "NIGHT"),
        t("learn_toolbar", "LEARN"),
        t("challenges_toolbar", "CHALLENGES"),
        t("combat_toolbar", "COMBAT"),
        t("galactic_explorer_title", "GALACTIC EXPLORER"),
        t("high", "High"),
        t("medium_q", "Medium"),
        t("low", "Low"),
        t("easy", "Easy"),
        t("normal", "Normal"),
        t("hard", "Hard"),
    ].into_iter().collect();
    map.insert("English".to_string(), en);
    let es: HashMap<String, String> = vec![
        t("settings_title", "AJUSTES"),
        t("graphics_section", "--- GRÁFICOS ---"),
        t("display_section", "--- PANTALLA ---"),
        t("audio_section", "--- AUDIO ---"),
        t("language_section", "--- IDIOMA ---"),
        t("combat_section", "--- COMBATE ---"),
        t("close", "Cerrar"),
        t("night_sky", "Cielo Nocturno"),
        t("quality", "Calidad"),
        t("fullscreen", "Pantalla Completa"),
        t("show_orbits", "Mostrar Órbitas"),
        t("planet_labels", "Etiquetas Planetas"),
        t("show_fps", "Mostrar FPS"),
        t("auto_rotate", "Rotación Auto"),
        t("audio", "Audio"),
        t("music", "Música"),
        t("sfx", "EFX"),
        t("language_label", "Idioma"),
        t("difficulty", "Dificultad"),
        t("shield", "Escudo"),
        t("on", "SÍ"),
        t("off", "NO"),
        t("settings_toolbar", "AJUSTES"),
        t("geo_map_toolbar", "MAPA GEO"),
        t("planets_toolbar", "PLANETAS"),
        t("orbits_toolbar", "ÓRBITAS"),
        t("night_toolbar", "NOCHE"),
        t("learn_toolbar", "APRENDER"),
        t("challenges_toolbar", "DESAFÍOS"),
        t("combat_toolbar", "COMBATE"),
        t("galactic_explorer_title", "EXPLORADOR GALÁCTICO"),
        t("high", "Alta"),
        t("medium_q", "Media"),
        t("low", "Baja"),
        t("easy", "Fácil"),
        t("normal", "Normal"),
        t("hard", "Difícil"),
    ].into_iter().collect();
    map.insert("Spanish".to_string(), es);
    let fr: HashMap<String, String> = vec![
        t("settings_title", "PARAMÈTRES"),
        t("graphics_section", "--- GRAPHIQUES ---"),
        t("display_section", "--- AFFICHAGE ---"),
        t("audio_section", "--- AUDIO ---"),
        t("language_section", "--- LANGUE ---"),
        t("combat_section", "--- COMBAT ---"),
        t("close", "Fermer"),
        t("night_sky", "Ciel Nocturne"),
        t("quality", "Qualité"),
        t("fullscreen", "Plein Écran"),
        t("show_orbits", "Afficher Orbites"),
        t("planet_labels", "Étiquettes Planètes"),
        t("show_fps", "Afficher FPS"),
        t("auto_rotate", "Rotation Auto"),
        t("audio", "Audio"),
        t("music", "Musique"),
        t("sfx", "EFX"),
        t("language_label", "Langue"),
        t("difficulty", "Difficulté"),
        t("shield", "Bouclier"),
        t("on", "OUI"),
        t("off", "NON"),
        t("settings_toolbar", "PARAMÈTRES"),
        t("geo_map_toolbar", "CARTE GÉO"),
        t("planets_toolbar", "PLANÈTES"),
        t("orbits_toolbar", "ORBITES"),
        t("night_toolbar", "NUIT"),
        t("learn_toolbar", "APPRENDRE"),
        t("challenges_toolbar", "DÉFIS"),
        t("combat_toolbar", "COMBAT"),
        t("galactic_explorer_title", "EXPLORATEUR GALACTIQUE"),
        t("high", "Haute"),
        t("medium_q", "Moyenne"),
        t("low", "Basse"),
        t("easy", "Facile"),
        t("normal", "Normal"),
        t("hard", "Difficile"),
    ].into_iter().collect();
    map.insert("French".to_string(), fr);
    Translations { map }
}

// ── COMPONENTS ──
#[derive(Component)]
pub struct Planet { pub name: String, pub index: usize }
#[derive(Component)]
pub struct AdvancedHud;
#[derive(Component)]
pub struct HudClock;
#[derive(Component)]
pub struct LoadingText;
#[derive(Component)]
pub struct SearchInput;
#[derive(Component)]
pub struct PlanetInfoPanel;
#[derive(Component)]
pub struct SettingsPanel;
#[derive(Component)]
pub struct GeoMapPanel;
#[derive(Component)]
pub struct GeoCountryLabel;
#[derive(Component)]
pub struct LoadingScreen;
#[derive(Component)]
pub struct LoadingBar;
#[derive(Component)]
pub struct EducationalPanel;
#[derive(Component)]
pub struct EduPlanetName;
#[derive(Component)]
pub struct EduDescription;
#[derive(Component)]
pub struct EduFactText(usize);
#[derive(Component)]
pub struct EduPhysicalText(usize);
#[derive(Component)]
pub struct EduFunFactText(usize);
#[derive(Component, Clone, Copy)]
pub struct SettingsNightSkyText;
#[derive(Component, Clone, Copy)]
pub struct SettingsQualityText;
#[derive(Component, Clone, Copy)]
pub struct SettingsFullscreenText;
#[derive(Component, Clone, Copy)]
pub struct SettingsShowOrbitsText;
#[derive(Component, Clone, Copy)]
pub struct SettingsPlanetLabelsText;
#[derive(Component, Clone, Copy)]
pub struct SettingsShowFpsText;
#[derive(Component, Clone, Copy)]
pub struct SettingsAutoRotateText;
#[derive(Component, Clone, Copy)]
pub struct SettingsAudioText;
#[derive(Component, Clone, Copy)]
pub struct SettingsMusicText;
#[derive(Component, Clone, Copy)]
pub struct SettingsMusicPlusText;
#[derive(Component, Clone, Copy)]
pub struct SettingsMusicMinusText;
#[derive(Component, Clone, Copy)]
pub struct SettingsSfxText;
#[derive(Component, Clone, Copy)]
pub struct SettingsSfxPlusText;
#[derive(Component, Clone, Copy)]
pub struct SettingsSfxMinusText;
#[derive(Component, Clone, Copy)]
pub struct SettingsLanguageText;
#[derive(Component, Clone, Copy)]
pub struct SettingsDifficultyText;
#[derive(Component, Clone, Copy)]
pub struct SettingsShieldText;
#[derive(Component)]
pub struct TranslatableText { pub key: String }
#[derive(Component)]
pub struct ToolbarButtonText { pub key: String }
#[derive(Component)]
pub struct HudTitleText;
#[derive(Component)]
pub struct QuizPanel;
#[derive(Component)]
pub struct QuizScoreText;
#[derive(Component)]
pub struct QuizQuestionText;
#[derive(Component)]
pub struct QuizOptionText(usize);
#[derive(Component)]
pub struct QuizExplanationText;
#[derive(Component)]
pub struct QuizResultText;
#[derive(Component)]
pub struct ChallengesPanel;
#[derive(Component)]
pub struct CombatPanel;
#[derive(Component)]
pub struct CombatWinScreen;
#[derive(Component)]
pub struct CombatLoseScreen;
#[derive(Component)]
pub struct LandingPanel;
#[derive(Component)]
pub struct ChallengeButton { pub challenge_index: usize }
#[derive(Component)]
pub struct ActiveChallengeText;
#[derive(Component)]
pub struct TimerText;
#[derive(Component)]
pub struct Enemy { pub health: f32, pub speed: f32, pub damage: f32, pub reward_score: u32 }
#[derive(Component)]
pub struct Bullet { pub direction: Vec3, pub speed: f32, pub damage: f32, pub lifetime: f32 }
#[derive(Component)]
pub struct PlayerBullet;
#[derive(Component)]
pub struct CombatHealthBar;
#[derive(Component)]
pub struct CombatShieldBar;
#[derive(Component)]
pub struct HudHealthBar;
#[derive(Component)]
pub struct HudShieldBar;
#[derive(Component)]
pub struct CombatStatusText;
#[derive(Component)]
pub struct CombatResultStatsText;
#[derive(Resource, Default)]
pub struct FpsText { pub entity: Option<Entity> }
#[derive(Component)]
pub struct FpsCounter;
#[derive(Component)]
pub struct QualityGlowEffect;
#[derive(Component)]
pub struct EduPlanetSelectButton { pub planet_name: String }
#[derive(Component)]
pub struct EduMissionText(usize);

// ── PLANET PRESETS ─
fn build_encyclopedia() -> Encyclopedia { let mut data = HashMap::new();
    let general_questions: Vec<QuizQuestion> = vec![
        QuizQuestion { question: "What is the largest planet in our solar system?".into(), options: vec!["Saturn".into(), "Jupiter".into(), "Neptune".into(), "Uranus".into()], correct_index: 1, explanation: "Jupiter is the largest planet, with a diameter of 139,820 km.".into() },
        QuizQuestion { question: "Which planet is known as the Red Planet?".into(), options: vec!["Venus".into(), "Mars".into(), "Jupiter".into(), "Mercury".into()], correct_index: 1, explanation: "Mars is known as the Red Planet due to iron oxide (rust) on its surface.".into() },
        QuizQuestion { question: "What is the closest planet to the Sun?".into(), options: vec!["Venus".into(), "Mercury".into(), "Earth".into(), "Mars".into()], correct_index: 1, explanation: "Mercury is the closest planet to the Sun at 57.9 million km.".into() },
        QuizQuestion { question: "How many planets are in our solar system?".into(), options: vec!["7".into(), "8".into(), "9".into(), "10".into()], correct_index: 1, explanation: "There are 8 recognized planets in our solar system: Mercury to Neptune.".into() },
        QuizQuestion { question: "Which planet has the most moons?".into(), options: vec!["Jupiter".into(), "Saturn".into(), "Uranus".into(), "Neptune".into()], correct_index: 1, explanation: "Saturn has at least 146 known moons, the most of any planet.".into() },
        QuizQuestion { question: "What is the hottest planet in our solar system?".into(), options: vec!["Mercury".into(), "Venus".into(), "Mars".into(), "Jupiter".into()], correct_index: 1, explanation: "Venus is the hottest planet at 462°C due to its runaway greenhouse effect.".into() },
        QuizQuestion { question: "Which planet has the shortest day?".into(), options: vec!["Earth".into(), "Jupiter".into(), "Saturn".into(), "Mars".into()], correct_index: 1, explanation: "Jupiter has the shortest day at just 9.9 hours.".into() },
        QuizQuestion { question: "What are Saturn's rings made of?".into(), options: vec!["Gas and dust".into(), "Ice and rock particles".into(), "Liquid water".into(), "Pure gold".into()], correct_index: 1, explanation: "Saturn's rings are made of billions of ice and rock particles.".into() },
        QuizQuestion { question: "Which planet rotates on its side?".into(), options: vec!["Neptune".into(), "Uranus".into(), "Pluto".into(), "Venus".into()], correct_index: 1, explanation: "Uranus rotates on its side with an extreme axial tilt of 98 degrees.".into() },
        QuizQuestion { question: "What is the largest moon in the solar system?".into(), options: vec!["Titan".into(), "Ganymede".into(), "Europa".into(), "Triton".into()], correct_index: 1, explanation: "Ganymede is the largest moon, bigger than Mercury!".into() },
        QuizQuestion { question: "Which planet has the strongest winds?".into(), options: vec!["Jupiter".into(), "Neptune".into(), "Saturn".into(), "Earth".into()], correct_index: 1, explanation: "Neptune has the fastest winds, reaching 2,100 km/h.".into() },
        QuizQuestion { question: "What causes a solar eclipse?".into(), options: vec!["Earth blocks the Sun".into(), "Moon blocks the Sun".into(), "Venus passes in front".into(), "Clouds cover the sky".into()], correct_index: 1, explanation: "A solar eclipse occurs when the Moon passes between Earth and the Sun.".into() },
        QuizQuestion { question: "What is a light-year?".into(), options: vec!["One year of time".into(), "Distance light travels in one year".into(), "The speed of light".into(), "A measure of brightness".into()], correct_index: 1, explanation: "A light-year is the distance light travels in one year: about 9.46 trillion km.".into() },
        QuizQuestion { question: "Which planet is called Earth's twin?".into(), options: vec!["Mars".into(), "Venus".into(), "Mercury".into(), "Neptune".into()], correct_index: 1, explanation: "Venus is called Earth's twin due to similar size and mass.".into() },
        QuizQuestion { question: "What is a black hole?".into(), options: vec!["A hole in space".into(), "An object with extreme gravity".into(), "A dark star".into(), "An empty region".into()], correct_index: 1, explanation: "A black hole is an object with gravity so strong that nothing, not even light, can escape.".into() },
        QuizQuestion { question: "How long does it take for the Moon to orbit Earth?".into(), options: vec!["7 days".into(), "27.3 days".into(), "30 days".into(), "365 days".into()], correct_index: 1, explanation: "The Moon takes about 27.3 days to complete one orbit around Earth.".into() },
        QuizQuestion { question: "What is the Great Red Spot?".into(), options: vec!["A volcano on Mars".into(), "A storm on Jupiter".into(), "A crater on the Moon".into(), "A sea on Titan".into()], correct_index: 1, explanation: "The Great Red Spot is a massive storm on Jupiter larger than Earth.".into() },
        QuizQuestion { question: "Which planet has no moons?".into(), options: vec!["Earth".into(), "Mercury and Venus".into(), "Mars".into(), "Jupiter".into()], correct_index: 1, explanation: "Mercury and Venus are the only two planets with zero moons.".into() },
        QuizQuestion { question: "What is the asteroid belt?".into(), options: vec!["A ring of ice".into(), "Rocky bodies between Mars and Jupiter".into(), "A type of galaxy".into(), "Comet debris".into()], correct_index: 1, explanation: "The asteroid belt is a region of rocky bodies orbiting between Mars and Jupiter.".into() },
        QuizQuestion { question: "What is the Sun made of?".into(), options: vec!["Rock and metal".into(), "Hydrogen and helium".into(), "Liquid fire".into(), "Carbon and oxygen".into()], correct_index: 1, explanation: "The Sun is about 73% hydrogen and 25% helium by mass.".into() },
        QuizQuestion { question: "How old is the solar system?".into(), options: vec!["1 billion years".into(), "4.6 billion years".into(), "10 billion years".into(), "100 million years".into()], correct_index: 1, explanation: "Our solar system formed about 4.6 billion years ago.".into() },
        QuizQuestion { question: "What is the Kuiper Belt?".into(), options: vec!["A belt of asteroids".into(), "Icy bodies beyond Neptune".into(), "A type of galaxy".into(), "A storm on Saturn".into()], correct_index: 1, explanation: "The Kuiper Belt is a region of icy bodies beyond Neptune, home to Pluto and comets.".into() },
        QuizQuestion { question: "Which planet has a day longer than its year?".into(), options: vec!["Earth".into(), "Venus".into(), "Mars".into(), "Jupiter".into()], correct_index: 1, explanation: "Venus has a day of 243 Earth days but a year of only 225 Earth days.".into() },
        QuizQuestion { question: "What is a comet made of?".into(), options: vec!["Liquid water".into(), "Ice, dust, and rock".into(), "Pure gas".into(), "Metal".into()], correct_index: 1, explanation: "Comets are icy bodies made of frozen gases, dust, and rock.".into() },
        QuizQuestion { question: "How many galaxies are in the observable universe?".into(), options: vec!["1 million".into(), "100 billion".into(), "1 trillion".into(), "100 trillion".into()], correct_index: 2, explanation: "There are an estimated 1 to 2 trillion galaxies in the observable universe.".into() },
        QuizQuestion { question: "What is the name of our galaxy?".into(), options: vec!["Andromeda".into(), "Milky Way".into(), "Triangulum".into(), "Whirlpool".into()], correct_index: 1, explanation: "Our galaxy is called the Milky Way, a spiral galaxy containing 100-400 billion stars.".into() },
        QuizQuestion { question: "What is the most abundant element in the universe?".into(), options: vec!["Oxygen".into(), "Hydrogen".into(), "Helium".into(), "Carbon".into()], correct_index: 1, explanation: "Hydrogen is the most abundant element, making up about 75% of all normal matter.".into() },
        QuizQuestion { question: "Which spacecraft first reached interstellar space?".into(), options: vec!["Apollo 11".into(), "Voyager 1".into(), "Cassini".into(), "New Horizons".into()], correct_index: 1, explanation: "Voyager 1 crossed into interstellar space in 2012, the first human-made object to do so.".into() },
        QuizQuestion { question: "What is a supernova?".into(), options: vec!["A new star".into(), "An exploding star".into(), "A type of asteroid".into(), "A comet impact".into()], correct_index: 1, explanation: "A supernova is a powerful stellar explosion that occurs at the end of a massive star's life.".into() },
        QuizQuestion { question: "Which planet has the largest volcano?".into(), options: vec!["Earth".into(), "Mars".into(), "Venus".into(), "Jupiter".into()], correct_index: 1, explanation: "Mars has Olympus Mons, the largest volcano at 21.9 km tall.".into() },
        QuizQuestion { question: "What is dark matter?".into(), options: vec!["Black paint in space".into(), "Invisible matter with gravity".into(), "Dead stars".into(), "Empty space".into()], correct_index: 1, explanation: "Dark matter is a mysterious invisible substance that makes up about 27% of the universe.".into() },
        QuizQuestion { question: "How fast does light travel?".into(), options: vec!["300,000 km/s".into(), "150,000 km/s".into(), "1 million km/s".into(), "100,000 km/s".into()], correct_index: 0, explanation: "Light travels at about 300,000 km/s (186,282 miles per second).".into() },
        QuizQuestion { question: "What is Pluto classified as?".into(), options: vec!["Planet".into(), "Dwarf planet".into(), "Asteroid".into(), "Comet".into()], correct_index: 1, explanation: "Pluto is classified as a dwarf planet, not a full planet, since 2006.".into() },
        QuizQuestion { question: "Which planet has a prominent ring system?".into(), options: vec!["Jupiter".into(), "Saturn".into(), "Uranus".into(), "Neptune".into()], correct_index: 1, explanation: "Saturn has the most prominent and beautiful ring system in the solar system.".into() },
        QuizQuestion { question: "What causes the northern lights on Earth?".into(), options: vec!["Reflected sunlight".into(), "Solar particles hitting the atmosphere".into(), "Moonlight".into(), "Volcanic eruptions".into()], correct_index: 1, explanation: "The aurora borealis is caused by charged solar particles interacting with Earth's magnetic field.".into() },
        QuizQuestion { question: "What is a neutron star?".into(), options: vec!["A star made of neutrons".into(), "The collapsed core of a massive star".into(), "A type of black hole".into(), "A fusion reactor".into()], correct_index: 1, explanation: "A neutron star is the incredibly dense collapsed core of a massive star after a supernova.".into() },
        QuizQuestion { question: "Which moon of Jupiter has a subsurface ocean?".into(), options: vec!["Io".into(), "Europa".into(), "Ganymede".into(), "Callisto".into()], correct_index: 1, explanation: "Europa is believed to have a global subsurface ocean with more water than Earth.".into() },
        QuizQuestion { question: "What is the habitable zone?".into(), options: vec!["A zone on Earth".into(), "The region where liquid water can exist".into(), "A type of orbit".into(), "A galaxy region".into()], correct_index: 1, explanation: "The habitable zone is the region around a star where liquid water could exist on a planet's surface.".into() },
        QuizQuestion { question: "Which planet experiences extreme 42-year-long seasons?".into(), options: vec!["Earth".into(), "Uranus".into(), "Neptune".into(), "Saturn".into()], correct_index: 1, explanation: "Uranus's 98-degree axial tilt causes extreme seasons lasting 42 Earth years each.".into() },
        QuizQuestion { question: "How many Earths could fit inside the Sun?".into(), options: vec!["1,000".into(), "1.3 million".into(), "10,000".into(), "100 million".into()], correct_index: 1, explanation: "About 1.3 million Earths could fit inside the Sun's volume.".into() },
        QuizQuestion { question: "What is the Oort Cloud?".into(), options: vec!["A cloud on Jupiter".into(), "A spherical shell of icy bodies".into(), "A type of nebula".into(), "An asteroid belt".into()], correct_index: 1, explanation: "The Oort Cloud is a theoretical spherical shell of icy bodies surrounding the solar system.".into() },
        QuizQuestion { question: "What is the closest star to Earth?".into(), options: vec!["Alpha Centauri".into(), "The Sun".into(), "Proxima Centauri".into(), "Sirius".into()], correct_index: 1, explanation: "The Sun is the closest star to Earth at about 150 million km away.".into() },
        QuizQuestion { question: "How long does sunlight take to reach Earth?".into(), options: vec!["1 second".into(), "8.3 minutes".into(), "1 hour".into(), "1 day".into()], correct_index: 1, explanation: "Sunlight takes about 8 minutes and 20 seconds to travel from the Sun to Earth.".into() },
        QuizQuestion { question: "What is a nebula?".into(), options: vec!["A type of star".into(), "A cloud of gas and dust".into(), "A galaxy".into(), "A black hole".into()], correct_index: 1, explanation: "A nebula is a giant cloud of gas and dust in space where stars are born.".into() },
        QuizQuestion { question: "Which planet has the most volcanoes?".into(), options: vec!["Mars".into(), "Venus".into(), "Earth".into(), "Mercury".into()], correct_index: 1, explanation: "Venus has over 1,600 known volcanoes, more than any other planet.".into() },
        QuizQuestion { question: "What is an exoplanet?".into(), options: vec!["A dead planet".into(), "A planet orbiting another star".into(), "A comet".into(), "An asteroid".into()], correct_index: 1, explanation: "An exoplanet is any planet that orbits a star outside our solar system.".into() },
        QuizQuestion { question: "Which spacecraft visited Pluto?".into(), options: vec!["Voyager 1".into(), "New Horizons".into(), "Cassini".into(), "Galileo".into()], correct_index: 1, explanation: "NASA's New Horizons spacecraft flew by Pluto in 2015, giving us our first close-up views.".into() },
        QuizQuestion { question: "What is the temperature of the Sun's core?".into(), options: vec!["5,500°C".into(), "15 million°C".into(), "1 million°C".into(), "100 million°C".into()], correct_index: 1, explanation: "The Sun's core reaches about 15 million degrees Celsius, hot enough for nuclear fusion.".into() },
        QuizQuestion { question: "How many stars are in the Milky Way?".into(), options: vec!["1 billion".into(), "100-400 billion".into(), "1 trillion".into(), "10 million".into()], correct_index: 1, explanation: "The Milky Way contains an estimated 100 to 400 billion stars.".into() },
    ];
    data.insert("Mercury".into(), PlanetData { description: "The smallest planet and closest to the Sun. A world of extremes.".into(), mass: 0.055, radius: 2440.0, day_length: 4222.6, surface_temp: "-180 to 430 C".into(), moons: 0, has_geo_map: false, continent_info: vec![], educational_facts: vec!["Mercury is only slightly larger than Earth's Moon.".into(),"A year on Mercury is just 88 Earth days.".into(),"Despite being closest to the Sun, Mercury is not the hottest planet - Venus is.".into(),"Mercury has a massive iron core that makes up about 75% of its radius.".into(),"The MESSENGER spacecraft orbited Mercury from 2011 to 2015.".into()], fun_facts: vec!["Mercury has no atmosphere, so temperatures swing wildly.".into(),"You could fit 18 Mercurys inside Earth.".into(),"Mercury's surface looks similar to our Moon.".into()], missions: vec![Mission { name: "Mariner 10".into(), year: "1974-1975".into(), agency: "NASA".into(), description: "First spacecraft to visit Mercury.".into() },Mission { name: "MESSENGER".into(), year: "2011-2015".into(), agency: "NASA".into(), description: "First to orbit Mercury.".into() }], quiz_questions: general_questions.clone() });
    data.insert("Venus".into(), PlanetData { description: "Earth's twin, shrouded in thick clouds of sulfuric acid.".into(), mass: 0.815, radius: 6052.0, day_length: 2802.0, surface_temp: "462 C".into(), moons: 0, has_geo_map: false, continent_info: vec![], educational_facts: vec!["Venus rotates backwards compared to most planets (retrograde rotation).".into(),"A day on Venus (243 Earth days) is longer than its year (225 Earth days).".into(),"Venus has a runaway greenhouse effect making it the hottest planet at 462C.".into(),"Venus is often called Earth's 'sister planet' due to similar size and mass.".into(),"More than 40 spacecraft have visited Venus, including NASA's Magellan.".into()], fun_facts: vec!["Venus is the brightest planet in our sky after the Moon.".into(),"It rains sulfuric acid on Venus, but it evaporates before reaching the surface.".into(),"Venus has over 1,600 volcanoes, more than any other planet.".into()], missions: vec![Mission { name: "Venera 7".into(), year: "1970".into(), agency: "Soviet Union".into(), description: "First spacecraft to land on another planet.".into() },Mission { name: "Magellan".into(), year: "1990-1994".into(), agency: "NASA".into(), description: "Mapped 98% of Venus's surface with radar.".into() }], quiz_questions: general_questions.clone() });
    data.insert("Earth".into(), PlanetData { description: "The Blue Marble - our home, the only known planet with life.".into(), mass: 1.0, radius: 6371.0, day_length: 24.0, surface_temp: "-89 to 57 C".into(), moons: 1, has_geo_map: true, continent_info: vec![ContinentInfo { name: "Africa".into(), area: "30,370,000 km2".into(), population: "1.4B".into(), countries: 54, description: "The cradle of humankind, rich in biodiversity.".into() },ContinentInfo { name: "Antarctica".into(), area: "14,200,000 km2".into(), population: "~5K".into(), countries: 0, description: "Icy continent, home to penguins & research stations.".into() },ContinentInfo { name: "Asia".into(), area: "44,579,000 km2".into(), population: "4.7B".into(), countries: 49, description: "Largest continent, home to 60% of the world population.".into() },ContinentInfo { name: "Europe".into(), area: "10,180,000 km2".into(), population: "745M".into(), countries: 44, description: "Birthplace of Western civilisation & the Renaissance.".into() },ContinentInfo { name: "North America".into(), area: "24,709,000 km2".into(), population: "592M".into(), countries: 23, description: "From Arctic tundra to tropical rainforests.".into() },ContinentInfo { name: "South America".into(), area: "17,840,000 km2".into(), population: "430M".into(), countries: 12, description: "Amazon rainforest, Andes mountains, rich cultures.".into() },ContinentInfo { name: "Australia/Oceania".into(), area: "8,600,000 km2".into(), population: "43M".into(), countries: 14, description: "Island continent with unique wildlife & coral reefs.".into() }], educational_facts: vec!["Earth is the only planet known to support life, with over 8.7 million species.".into(),"71% of Earth's surface is covered in water, mostly oceans.".into(),"Earth's atmosphere is 78% nitrogen and 21% oxygen.".into(),"The Earth's core is about as hot as the Sun's surface - 5,500C.".into(),"Earth's magnetic field protects us from harmful solar radiation.".into()], fun_facts: vec!["Earth is the densest planet in the solar system.".into(),"A day on Earth is 23 hours, 56 minutes, and 4 seconds (sidereal day).".into(),"Earth's rotation is slowing down by about 1.7 milliseconds per century.".into()], missions: vec![Mission { name: "Apollo 11".into(), year: "1969".into(), agency: "NASA".into(), description: "First humans landed on the Moon.".into() },Mission { name: "ISS".into(), year: "1998-present".into(), agency: "International".into(), description: "International Space Station orbits Earth every 90 minutes.".into() },Mission { name: "James Webb".into(), year: "2021-present".into(), agency: "NASA/ESA".into(), description: "Most powerful space telescope ever built.".into() }], quiz_questions: general_questions.clone() });
    data.insert("Mars".into(), PlanetData { description: "The Red Planet - humanity's next frontier for exploration.".into(), mass: 0.107, radius: 3390.0, day_length: 24.6, surface_temp: "-87 to -5 C".into(), moons: 2, has_geo_map: false, continent_info: vec![], educational_facts: vec!["Mars has the tallest mountain in the solar system - Olympus Mons at 21.9 km.".into(),"Mars has the largest canyon - Valles Marineris, 4,000 km long.".into(),"A day on Mars (called a 'sol') is just 40 minutes longer than an Earth day.".into(),"Mars has evidence of ancient rivers, lakes, and possibly oceans.".into(),"NASA's Perseverance rover is currently searching for signs of ancient life on Mars.".into()], fun_facts: vec!["Mars has the largest dust storms in the solar system, lasting months.".into(),"Mars's sun appears about half the size as it does on Earth.".into(),"Mars has two small moons: Phobos and Deimos.".into()], missions: vec![Mission { name: "Viking 1".into(), year: "1976".into(), agency: "NASA".into(), description: "First successful Mars lander.".into() },Mission { name: "Curiosity".into(), year: "2012-present".into(), agency: "NASA".into(), description: "Car-sized rover exploring Gale Crater.".into() },Mission { name: "Perseverance".into(), year: "2021-present".into(), agency: "NASA".into(), description: "Searching for ancient microbial life.".into() }], quiz_questions: general_questions.clone() });
    data.insert("Jupiter".into(), PlanetData { description: "The largest planet, a gas giant with a Great Red Spot storm.".into(), mass: 317.8, radius: 69911.0, day_length: 9.9, surface_temp: "-110 C".into(), moons: 95, has_geo_map: false, continent_info: vec![], educational_facts: vec!["Jupiter is the largest planet in our solar system - 1,300 Earths could fit inside it.".into(),"The Great Red Spot is a storm larger than Earth that has raged for hundreds of years.".into(),"Jupiter has the shortest day of any planet - just 9.9 hours.".into(),"Jupiter has at least 95 known moons, including the four large Galilean moons.".into(),"Jupiter's magnetic field is 20,000 times stronger than Earth's.".into()], fun_facts: vec!["Jupiter's Great Red Spot is shrinking.".into(),"Jupiter has a ring system, though it's very faint.".into(),"Jupiter's moon Europa may have a subsurface ocean with more water than Earth.".into()], missions: vec![Mission { name: "Pioneer 10".into(), year: "1973".into(), agency: "NASA".into(), description: "First spacecraft to visit Jupiter.".into() },Mission { name: "Galileo".into(), year: "1995-2003".into(), agency: "NASA".into(), description: "First to orbit Jupiter.".into() },Mission { name: "Juno".into(), year: "2016-present".into(), agency: "NASA".into(), description: "Studying Jupiter's magnetic field.".into() }], quiz_questions: general_questions.clone() });
    data.insert("Saturn".into(), PlanetData { description: "The ringed giant - its rings span 282,000 km.".into(), mass: 95.2, radius: 58232.0, day_length: 10.7, surface_temp: "-140 C".into(), moons: 146, has_geo_map: false, continent_info: vec![], educational_facts: vec!["Saturn's rings are made of billions of ice and rock particles.".into(),"Saturn is the least dense planet - it would float in water!".into(),"Saturn has at least 146 known moons, the most of any planet.".into(),"Titan, Saturn's largest moon, has a thick atmosphere and liquid methane lakes.".into(),"The Cassini spacecraft studied Saturn for 13 years before diving into its atmosphere.".into()], fun_facts: vec!["Saturn's rings are only 10 meters thick in most places.".into(),"Saturn's moon Enceladus has geysers shooting water into space.".into()], missions: vec![Mission { name: "Pioneer 11".into(), year: "1979".into(), agency: "NASA".into(), description: "First spacecraft to visit Saturn.".into() },Mission { name: "Cassini-Huygens".into(), year: "2004-2017".into(), agency: "NASA/ESA".into(), description: "Studied Saturn for 13 years.".into() }], quiz_questions: general_questions.clone() });
    data.insert("Uranus".into(), PlanetData { description: "An ice giant that rotates on its side.".into(), mass: 14.5, radius: 25362.0, day_length: 17.2, surface_temp: "-195 C".into(), moons: 27, has_geo_map: false, continent_info: vec![], educational_facts: vec!["Uranus rotates on its side with an axial tilt of 98 degrees.".into(),"Uranus was the first planet discovered with a telescope (by William Herschel in 1781).".into(),"Uranus has a blue-green color due to methane in its atmosphere.".into(),"Uranus has 27 known moons, all named after characters from Shakespeare and Pope.".into(),"Uranus has faint rings that were discovered in 1977.".into()], fun_facts: vec!["Uranus has 13 known rings.".into(),"Uranus takes 84 Earth years to orbit the Sun.".into()], missions: vec![Mission { name: "Voyager 2".into(), year: "1986".into(), agency: "NASA".into(), description: "Only spacecraft to visit Uranus.".into() }], quiz_questions: general_questions.clone() });
    data.insert("Neptune".into(), PlanetData { description: "The windiest planet - gusts reach 2,100 km/h.".into(), mass: 17.1, radius: 24622.0, day_length: 16.1, surface_temp: "-200 C".into(), moons: 16, has_geo_map: false, continent_info: vec![], educational_facts: vec!["Neptune has the strongest winds of any planet, reaching 2,100 km/h.".into(),"Neptune was the first planet located through mathematical prediction rather than observation.".into(),"Neptune takes 165 Earth years to orbit the Sun once.".into(),"Neptune has a faint ring system and 16 known moons.".into(),"Triton, Neptune's largest moon, orbits in the opposite direction of Neptune's rotation.".into()], fun_facts: vec!["Neptune emits more than twice as much heat as it receives from the Sun.".into(),"Neptune's winds are the fastest in the solar system.".into()], missions: vec![Mission { name: "Voyager 2".into(), year: "1989".into(), agency: "NASA".into(), description: "Only spacecraft to visit Neptune.".into() }], quiz_questions: general_questions.clone() });
    Encyclopedia { data }
}

fn get_challenges() -> Vec<Challenge> {
    vec![
        Challenge { title: "Earth Explorer".into(), description: "Learn 5 facts about Earth".into(), objective: "Complete Earth quiz".into(), time_limit: 120.0, reward: "Earth Badge + 100 pts".into(), difficulty: "Easy".into() },
        Challenge { title: "Mars Mission".into(), description: "Study Mars".into(), objective: "Perfect score on Mars quiz".into(), time_limit: 180.0, reward: "Mars Badge + 200 pts".into(), difficulty: "Medium".into() },
        Challenge { title: "Solar System Master".into(), description: "Complete all quizzes".into(), objective: "Pass all 8 planet quizzes".into(), time_limit: 600.0, reward: "Master Badge + 1000 pts".into(), difficulty: "Hard".into() },
        Challenge { title: "Speed Runner".into(), description: "Fast quiz completion".into(), objective: "Complete quiz in 30s".into(), time_limit: 30.0, reward: "Speed Badge + 150 pts".into(), difficulty: "Medium".into() },
        Challenge { title: "Knowledge Seeker".into(), description: "Read facts for 3 planets".into(), objective: "View educational panels".into(), time_limit: 300.0, reward: "Scholar Badge + 250 pts".into(), difficulty: "Easy".into() },
        Challenge { title: "Space Warrior".into(), description: "Defeat 10 space enemies".into(), objective: "Kill 10 enemies in combat".into(), time_limit: 600.0, reward: "Warrior Badge + 500 pts".into(), difficulty: "Hard".into() },
    ]
}

// ── MAIN ──
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "GALACTIC EXPLORER: DISCOVERY PRO".into(),
                resolution: (1400.0, 900.0).into(),
                resize_constraints: WindowResizeConstraints { min_width: 800.0, min_height: 600.0, max_width: 2560.0, max_height: 1440.0, ..default() },
                ..default()
            }),
            ..default()
        }))
        .add_plugins((AssetPipelinePlugin, PhysicsPlugin, RenderingPlugin, DataPlugin))
        .insert_resource(Flow { screen: ScreenMode::Playing, loading_progress: 1.0 })
        .insert_resource(Settings::default())
        .insert_resource(VirtualControls::default())
        .init_state::<AppState>()
        .insert_resource(build_encyclopedia())
        .insert_resource(build_translations())
        .insert_resource(AppSettings { ui_scale: 1.0, graphics_quality: "High".into(), night_sky_enabled: true, show_planet_orbits: false, show_geo_map: false, show_educational_tips: true, show_planet_labels: true, language: "English".into(), audio_enabled: true, music_volume: 0.7, sfx_volume: 0.8, fullscreen: false, vsync: true, antialiasing: true, show_fps: false, auto_rotate: true, combat_difficulty: "Normal".into(), shield_level: 100 })
        .insert_resource(PlayerShip { health: 100.0, max_health: 100.0, shield: 50.0, max_shield: 100.0, score: 0, kills: 0, level: 1, speed: 5.0 })
        .insert_resource(CombatState { enemies: vec![], wave: 0, active: false, spawn_timer: 0.0, enemy_count: 0, has_won: false, has_lost: false, total_to_kill: 10, fire_cooldown: 0.15, fire_timer: 0.0 })
        .insert_resource(SimulationTime { current_date: "July 13, 2026".into(), current_time: "12:00:00 UTC".into(), speed_multiplier: 1.0 })
        .insert_resource(SearchQuery::default())
        .insert_resource(SelectedPlanet { name: "Earth".into(), index: 2 })
        .insert_resource(LoadingProgress::default())
        .insert_resource(QuizState::default())
        .insert_resource(ChallengeState::default())
        .insert_resource(ChallengesResource { challenges: get_challenges() })
        .insert_resource(FpsText::default())
        .insert_resource(AmbientLight { color: Color::rgb(0.35, 0.38, 0.5), brightness: 0.45 })
        .add_systems(Startup, (setup_loading_screen, setup_advanced_hud, setup_fps_counter, setup_audio))
        .add_systems(Update, (
            load_loading.run_if(in_state(AppState::Loading)),
            handle_toolbar_buttons.run_if(in_state(AppState::Exploration)),
            fly_settings.run_if(in_state(AppState::Settings)),
            fly_geo.run_if(in_state(AppState::GeoMap)),
            fly_edu.run_if(in_state(AppState::Educational)),
            fly_quiz.run_if(in_state(AppState::Quiz)),
            fly_challenges.run_if(in_state(AppState::Challenges)),
            fly_combat.run_if(in_state(AppState::Combat)),
            fly_info.run_if(in_state(AppState::PlanetInfo)),
            fly_landing.run_if(in_state(AppState::Landing)),
            hide_panels_on_state_exit,
            update_hud_clock,
            search_planets,
            update_planet_labels,
        ))
        .add_systems(Update, (
            sync_health_from_physics,
            update_hud_bars,
            update_combat,
            update_combat_ui,
            update_enemy_bullets,
            update_night_sky_visibility,
            update_orbit_visibility,
            handle_quiz_buttons.run_if(in_state(AppState::Quiz)),
            update_settings_ui.run_if(in_state(AppState::Settings)),
            apply_all_settings,
            apply_language_translations,
            destroy_asteroids_with_player_bullets,
            play_button_sfx,
            play_lose_sfx,
            play_quiz_sfx.run_if(in_state(AppState::Quiz)),
            handle_window_close,
            control_music,
        ))
        
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
            // HUD HEALTH BAR
            t.spawn(NodeBundle {
                style: Style { width: Val::Px(160.0), height: Val::Px(14.0), flex_direction: FlexDirection::Row, ..default() },
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
                    }, HudHealthBar));
                });
            });
            // HUD SHIELD BAR
            t.spawn(NodeBundle {
                style: Style { width: Val::Px(120.0), height: Val::Px(10.0), flex_direction: FlexDirection::Row, ..default() },
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
                    }, HudShieldBar));
                });
            });
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
            style: Style { position_type: PositionType::Absolute, left: Val::Percent(2.0), top: Val::Percent(3.0), width: Val::Px(950.0), max_height: Val::Px(900.0), padding: UiRect::all(Val::Px(16.0)), display: Display::None, flex_direction: FlexDirection::Column, row_gap: Val::Px(4.0), border: UiRect::all(Val::Px(2.0)), overflow: Overflow::clip(), ..default() },
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
            style: Style { position_type: PositionType::Absolute, right: Val::Percent(1.0), top: Val::Percent(3.0), width: Val::Px(380.0), max_height: Val::Px(560.0), padding: UiRect::all(Val::Px(12.0)), display: Display::None, flex_direction: FlexDirection::Column, row_gap: Val::Px(4.0), border: UiRect::all(Val::Px(2.0)), overflow: Overflow::clip_y(), ..default() },
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
            style: Style { position_type: PositionType::Absolute, left: Val::Percent(10.0), top: Val::Percent(5.0), width: Val::Px(620.0), height: Val::Px(660.0), padding: UiRect::all(Val::Px(20.0)), display: Display::None, flex_direction: FlexDirection::Column, row_gap: Val::Px(8.0), border: UiRect::all(Val::Px(2.0)), overflow: Overflow::clip(), ..default() },
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

        // COMBAT PANEL - smaller, more compact
        c.spawn((NodeBundle {
            style: Style { position_type: PositionType::Absolute, left: Val::Percent(10.0), top: Val::Percent(10.0), width: Val::Px(500.0), height: Val::Px(380.0), padding: UiRect::all(Val::Px(16.0)), display: Display::None, flex_direction: FlexDirection::Column, row_gap: Val::Px(8.0), border: UiRect::all(Val::Px(2.0)), overflow: Overflow::clip(), ..default() },
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
            m.spawn(TextBundle::from_section("[WASD] to pilot your ship | Destroy all enemies to win!", TextStyle { font_size: 15.0, color: Color::rgb(0.9, 0.6, 0.4), ..default() }));
            m.spawn(TextBundle::from_section("Enemies approach! Maneuver carefully - collisions damage your ship!", TextStyle { font_size: 18.0, color: Color::rgb(0.3, 1.0, 0.3), ..default() }));
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

fn _hbtn(parent: &mut ChildBuilder, text: &str, color: Color) {
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
        if let Some(mut s) = iter.next() { s.sections[0].value = format!("Mass: {} M  |  Radius: {} km  |  Day: {}h", d.mass, d.radius, d.day_length); }
        if let Some(mut s) = iter.next() { s.sections[0].value = format!("Moons: {}  |  Surface: {}", d.moons, d.surface_temp); }
    }
    for (interact, children) in interaction.iter() {
        if *interact == Interaction::Pressed {
            for &child in children.iter() {
                if let Ok(text) = text_queries.p0().get(child) {
                    if let Some(section) = text.sections.first() {
                        let clicked = &section.value;
                        if clicked.contains("X") || clicked.contains("CLOSE") { ns.set(AppState::Exploration); }
                        else if clicked.contains("LEARN") { ns.set(AppState::Educational); }
                        else if clicked.contains("QUIZ") { ns.set(AppState::Quiz); }
                        else if clicked.contains("GEO MAP") { ns.set(AppState::GeoMap); }
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

fn setup_audio(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut audio_assets: ResMut<Assets<AudioSource>>,
) {
    // Play the ambient space music track with looping
    let music_handle: Handle<AudioSource> = asset_server.load("audio/space_ambient.ogg");
    commands.insert_resource(MusicHandle { handle: music_handle.clone() });
    commands.spawn(AudioBundle {
        source: music_handle,
        settings: PlaybackSettings {
            mode: PlaybackMode::Loop,
            volume: bevy::audio::Volume::new(0.7),
            ..default()
        },
    });

    // Generate short sine-wave SFX tones directly and register them as audio assets
    let sfx = SfxHandles {
        button_click: audio_assets.add(AudioSource { bytes: encode_wav_tone(660.0, 0.08).into() }),
        close: audio_assets.add(AudioSource { bytes: encode_wav_tone(330.0, 0.12).into() }),
        quiz_success: audio_assets.add(AudioSource { bytes: encode_wav_tone(880.0, 0.20).into() }),
        quiz_fail: audio_assets.add(AudioSource { bytes: encode_wav_tone(220.0, 0.25).into() }),
        lose: audio_assets.add(AudioSource { bytes: encode_wav_tone_descending(392.0, 0.8).into() }),
    };
    commands.insert_resource(sfx);
}

/// Encode a downward-pitched defeat tone (sad "lose" sound) into a WAV byte buffer
fn encode_wav_tone_descending(start_frequency: f32, duration: f32) -> Vec<u8> {
    let sample_rate = 44100u32;
    let num_samples = (duration * sample_rate as f32) as usize;
    let bytes_per_sample = 2u16;
    let data_size = num_samples * bytes_per_sample as usize;
    let mut wav = Vec::with_capacity(44 + data_size);
    // RIFF header
    wav.extend_from_slice(b"RIFF");
    wav.extend_from_slice(&(36u32 + data_size as u32).to_le_bytes());
    wav.extend_from_slice(b"WAVE");
    // fmt chunk
    wav.extend_from_slice(b"fmt ");
    wav.extend_from_slice(&16u32.to_le_bytes());
    wav.extend_from_slice(&1u16.to_le_bytes()); // PCM
    wav.extend_from_slice(&1u16.to_le_bytes()); // mono
    wav.extend_from_slice(&sample_rate.to_le_bytes());
    wav.extend_from_slice(&(sample_rate * bytes_per_sample as u32).to_le_bytes());
    wav.extend_from_slice(&bytes_per_sample.to_le_bytes());
    wav.extend_from_slice(&(bytes_per_sample * 8).to_le_bytes());
    // data chunk
    wav.extend_from_slice(b"data");
    wav.extend_from_slice(&(data_size as u32).to_le_bytes());
    for i in 0..num_samples {
        let t = i as f32 / sample_rate as f32;
        let envelope = (1.0 - t / duration).max(0.0);
        // Frequency slides downward for a defeat feel
        let freq = start_frequency * (1.0 - 0.6 * (t / duration));
        let sample = (t * freq * std::f32::consts::TAU).sin() * envelope * 0.45;
        let s = (sample.clamp(-1.0, 1.0) * i16::MAX as f32) as i16;
        wav.extend_from_slice(&s.to_le_bytes());
    }
    wav
}

/// Encode a short sine-wave tone (with decay envelope) into a WAV byte buffer
fn encode_wav_tone(frequency: f32, duration: f32) -> Vec<u8> {
    let sample_rate = 44100u32;
    let num_samples = (duration * sample_rate as f32) as usize;
    let bytes_per_sample = 2u16;
    let data_size = num_samples * bytes_per_sample as usize;
    let mut wav = Vec::with_capacity(44 + data_size);
    // RIFF header
    wav.extend_from_slice(b"RIFF");
    wav.extend_from_slice(&(36u32 + data_size as u32).to_le_bytes());
    wav.extend_from_slice(b"WAVE");
    // fmt chunk
    wav.extend_from_slice(b"fmt ");
    wav.extend_from_slice(&16u32.to_le_bytes());
    wav.extend_from_slice(&1u16.to_le_bytes()); // PCM
    wav.extend_from_slice(&1u16.to_le_bytes()); // mono
    wav.extend_from_slice(&sample_rate.to_le_bytes());
    wav.extend_from_slice(&(sample_rate * bytes_per_sample as u32).to_le_bytes());
    wav.extend_from_slice(&bytes_per_sample.to_le_bytes());
    wav.extend_from_slice(&(bytes_per_sample * 8).to_le_bytes());
    // data chunk
    wav.extend_from_slice(b"data");
    wav.extend_from_slice(&(data_size as u32).to_le_bytes());
    for i in 0..num_samples {
        let t = i as f32 / sample_rate as f32;
        let envelope = (1.0 - t / duration).max(0.0);
        let sample = (t * frequency * std::f32::consts::TAU).sin() * envelope * 0.5;
        let s = (sample.clamp(-1.0, 1.0) * i16::MAX as f32) as i16;
        wav.extend_from_slice(&s.to_le_bytes());
    }
    wav
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
        if let Ok(mut name) = texts.p0().get_single_mut() { name.sections[0].value = format!("{} {} - {}", emoji, pn, d.description); }
        if let Ok(mut desc) = texts.p1().get_single_mut() { desc.sections[0].value = d.description.clone(); }
        for (mut text, fm) in texts.p2().iter_mut() { text.sections[0].value = if fm.0 < d.educational_facts.len() { format!("• {}", d.educational_facts[fm.0]) } else { String::new() }; }
        let phys = vec![format!("Mass: {} Earth masses", d.mass), format!("Radius: {} km", d.radius), format!("Day Length: {} hours", d.day_length), format!("Surface Temperature: {}", d.surface_temp), format!("Number of Moons: {}", d.moons)];
        for (mut text, pm) in texts.p3().iter_mut() { text.sections[0].value = if pm.0 < phys.len() { format!("• {}", phys[pm.0]) } else { String::new() }; }
        for (mut text, fm) in texts.p4().iter_mut() { text.sections[0].value = if fm.0 < d.fun_facts.len() { format!("• {}", d.fun_facts[fm.0]) } else { String::new() }; }
        let mnames: Vec<String> = d.missions.iter().map(|m| format!("🚀 {} ({}) - {}: {}", m.name, m.year, m.agency, m.description)).collect();
        for (mut text, mm) in texts.p5().iter_mut() { text.sections[0].value = if mm.0 < mnames.len() { mnames[mm.0].clone() } else { String::new() }; }
    }
    for (interact, children) in interaction.iter() {
        if *interact == Interaction::Pressed { for &child in children.iter() {
            if let Ok(text) = texts.p6().get(child) { if let Some(section) = text.sections.first() { let v = &section.value;
                if v.contains("CLOSE") { ns.set(AppState::Exploration); return; }
                if v.contains("TAKE QUIZ") { ns.set(AppState::Quiz); return; }
                if v.contains("GEO MAP") { ns.set(AppState::GeoMap); return; }
            } }
            if let Ok(sel) = texts.p6().get(child) { if let Some(section) = sel.sections.first() { let v = &section.value;
                for p in ["Mercury","Venus","Earth","Mars","Jupiter","Saturn","Uranus","Neptune"] { if v.contains(p) { *planet_selector = p.to_string(); return; } }
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
        if let Ok(mut st) = texts.p0().get_single_mut() { st.sections[0].value = format!("Score: {}/{}  |  Correct: {}  |  Wrong: {}  |  Question {}/{}", qs.score, total, qs.correct_count, qs.wrong_count, current + 1, total); }
        if current < total { let q = &d.quiz_questions[current]; if let Ok(mut qt) = texts.p1().get_single_mut() { qt.sections[0].value = q.question.clone(); }
            for (mut text, om) in texts.p2().iter_mut() { text.sections[0].value = if om.0 < q.options.len() { q.options[om.0].clone() } else { String::new() }; }
            for (om, mut bg) in option_bg.iter_mut() { if qs.answered { if om.0 == q.correct_index { *bg = BackgroundColor(Color::rgb(0.1, 0.5, 0.1)); } else if Some(om.0) == Some(qs.selected_answer) { *bg = BackgroundColor(Color::rgb(0.5, 0.1, 0.1)); } else { *bg = BackgroundColor(Color::rgba(0.1, 0.15, 0.25, 0.5)); } } else { *bg = BackgroundColor(Color::rgba(0.1, 0.15, 0.25, 0.8)); } }
            if let Ok(mut et) = texts.p3().get_single_mut() { et.sections[0].value = if qs.answered { q.explanation.clone() } else { String::new() }; }
            if let Ok(mut rt) = texts.p4().get_single_mut() { rt.sections[0].value = String::new(); }
        } else { qs.active = false; qs.finished = true; }
    } }
    if qs.finished {
        if let Ok(mut qt) = texts.p1().get_single_mut() { qt.sections[0].value = "🎉 QUIZ COMPLETE! 🎉".to_string(); }
        if let Ok(mut st) = texts.p0().get_single_mut() { st.sections[0].value = format!("Final Score: {}/{}", qs.score, qs.total_questions); }
        if let Ok(mut rt) = texts.p4().get_single_mut() { let pct = if qs.total_questions > 0 { (qs.correct_count as f32 / qs.total_questions as f32) * 100.0 } else { 0.0 }; rt.sections[0].value = format!("✅ Correct: {} | ❌ Wrong: {} | 📊 {:.0}%", qs.correct_count, qs.wrong_count, pct); }
        for (mut text, _) in texts.p2().iter_mut() { text.sections[0].value = String::new(); }
        if let Ok(mut et) = texts.p3().get_single_mut() { et.sections[0].value = String::new(); }
        for (_, mut bg) in option_bg.iter_mut() { *bg = BackgroundColor(Color::rgba(0.1, 0.15, 0.25, 0.8)); }
    }
}

fn handle_quiz_buttons(
    interaction: Query<(&Interaction, &Children), (Changed<Interaction>, With<Button>, Without<SettingsPanel>)>,
    button_texts: Query<&Text>, enc: Res<Encyclopedia>, mut qs: ResMut<QuizState>,
    mut panel: Query<&mut Style, With<QuizPanel>>, mut ns: ResMut<NextState<AppState>>,
) {
    for (interact, children) in interaction.iter() { if *interact == Interaction::Pressed { for &child in children.iter() { if let Ok(text) = button_texts.get(child) { if let Some(section) = text.sections.first() { let v = &section.value;
        if v == "X" || v.contains("✕") || v.contains("CLOSE QUIZ") { if let Ok(mut q) = panel.get_single_mut() { q.display = Display::None; } qs.active = false; qs.finished = false; ns.set(AppState::Exploration); return; }
        if v.contains("NEXT") { if qs.active && !qs.finished && qs.answered { if let Some(d) = enc.data.get(&qs.current_planet) { qs.current_question += 1; qs.answered = false; qs.selected_answer = 0; if qs.current_question >= d.quiz_questions.len() { qs.active = false; qs.finished = true; } } } return; }
        if qs.active && !qs.finished && !qs.answered { if let Some(d) = enc.data.get(&qs.current_planet) { if qs.current_question < d.quiz_questions.len() { let q = &d.quiz_questions[qs.current_question]; for (i, opt) in q.options.iter().enumerate() { if v == opt { qs.selected_answer = i; qs.answered = true; if i == q.correct_index { qs.correct_count += 1; qs.score += 1; } else { qs.wrong_count += 1; } break; } } } } }
    } } } } }
}

fn fly_challenges(
    interaction: Query<(&Interaction, &Children), (Changed<Interaction>, With<Button>)>,
    mut ns: ResMut<NextState<AppState>>,
    challenges: Res<ChallengesResource>, mut cs: ResMut<ChallengeState>,
    mut panel: Query<&mut Style, With<ChallengesPanel>>,
    mut param_set: ParamSet<(
        Query<&Text>,
        Query<&mut Text, (With<TimerText>, Without<ChallengesPanel>)>,
        Query<&mut Text, (With<ActiveChallengeText>, Without<TimerText>, Without<ChallengesPanel>)>,
    )>,
    time: Res<Time>,
) {
    if let Ok(mut c) = panel.get_single_mut() { c.display = Display::Flex; }
    if cs.active { cs.time_remaining -= time.delta_seconds(); if cs.time_remaining <= 0.0 { cs.active = false; cs.time_remaining = 0.0; } }
    if let Ok(mut t) = param_set.p1().get_single_mut() { t.sections[0].value = if cs.active { format!("Time: {:.0}s", cs.time_remaining) } else { "Time Remaining: --:--".into() }; }
    for (interact, children) in interaction.iter() {
        if *interact != Interaction::Pressed { continue; }
        for &child in children.iter() {
            if let Ok(text) = param_set.p0().get(child) {
                if let Some(section) = text.sections.first() {
                    let clicked = section.value.clone();
                    if clicked == "X" || clicked.contains("CLOSE") || clicked.contains("✕") { ns.set(AppState::Exploration); return; }
                    for (i, ch) in challenges.challenges.iter().enumerate() {
                        let title = ch.title.as_str();
                        if clicked.contains(title) {
                            cs.active = true;
                            cs.current_challenge = i;
                            cs.time_remaining = ch.time_limit;
                            cs.total_time = ch.time_limit;
                            if let Ok(mut at) = param_set.p2().get_single_mut() {
                                at.sections[0].value = format!("▶ {} - {} | Objective: {} | Reward: {} | Difficulty: {}", ch.title, ch.description, ch.objective, ch.reward, ch.difficulty);
                            }
                            // Map each challenge to its correct destination page
                            match title {
                                "Earth Explorer" => {
                                    // Learn about Earth - go to Educational
                                    ns.set(AppState::Educational);
                                }
                                "Mars Mission" => {
                                    // Study Mars - go to Educational
                                    ns.set(AppState::Educational);
                                }
                                "Solar System Master" => {
                                    // Complete all quizzes - go to Quiz
                                    ns.set(AppState::Quiz);
                                }
                                "Speed Runner" => {
                                    // Fast quiz completion - go to Quiz
                                    ns.set(AppState::Quiz);
                                }
                                "Knowledge Seeker" => {
                                    // Read facts - go to Educational
                                    ns.set(AppState::Educational);
                                }
                                "Space Warrior" => {
                                    // Space combat - go to Combat
                                    ns.set(AppState::Combat);
                                }
                                _ => { ns.set(AppState::Quiz); }
                            }
                            return;
                        }
                    }
                }
            }
        }
    }
}

fn fly_combat(
    mut commands: Commands,
    interaction: Query<(&Interaction, &Children), (Changed<Interaction>, With<Button>)>,
    button_texts: Query<&Text>,
    enemy_query: Query<Entity, With<Enemy>>,
    bullet_query: Query<Entity, With<Bullet>>,
    mut ns: ResMut<NextState<AppState>>, mut combat: ResMut<CombatState>, mut ship: ResMut<PlayerShip>,
    mut param_set: ParamSet<(
        Query<&mut Style, With<CombatPanel>>,
        Query<&mut Style, (With<CombatWinScreen>, Without<CombatLoseScreen>)>,
        Query<&mut Style, (With<CombatLoseScreen>, Without<CombatWinScreen>)>,
    )>,
) {
    if let Ok(mut c) = param_set.p0().get_single_mut() { c.display = Display::Flex; }
    if let Ok(mut ws) = param_set.p1().get_single_mut() { ws.display = if combat.has_won { Display::Flex } else { Display::None }; }
    if let Ok(mut ls) = param_set.p2().get_single_mut() { ls.display = if combat.has_lost { Display::Flex } else { Display::None }; }
    for (interact, children) in interaction.iter() { if *interact == Interaction::Pressed { for &child in children.iter() { if let Ok(text) = button_texts.get(child) { if let Some(section) = text.sections.first() { let v = &section.value;
        if v == "X" || v.contains("CLOSE") { 
            // Despawn all enemies and bullets when closing combat
            for e in enemy_query.iter() { commands.entity(e).despawn(); }
            for b in bullet_query.iter() { commands.entity(b).despawn(); }
            ns.set(AppState::Exploration); combat.active = false; combat.has_won = false; combat.has_lost = false; 
            ship.health = 100.0; ship.shield = 50.0;
        }
        else if v.contains("TRY AGAIN") { 
            // CRITICAL FIX: Despawn all leftover enemies and bullets so they don't drain health instantly
            for e in enemy_query.iter() { commands.entity(e).despawn(); }
            for b in bullet_query.iter() { commands.entity(b).despawn(); }
            combat.has_lost = false; combat.has_won = false; combat.wave = 1; combat.spawn_timer = 0.0; combat.enemy_count = 0; combat.active = true; 
            ship.health = 100.0; ship.kills = 0; ship.shield = 50.0;
        }
        else if v.contains("RETURN TO EXPLORATION") { 
            // Despawn all enemies and bullets when returning
            for e in enemy_query.iter() { commands.entity(e).despawn(); }
            for b in bullet_query.iter() { commands.entity(b).despawn(); }
            ns.set(AppState::Exploration); combat.active = false; 
            ship.health = 100.0; ship.shield = 50.0;
        }
    } } } } }
}

fn fly_landing(
    interaction: Query<(&Interaction, &Children), (Changed<Interaction>, With<Button>)>,
    mut panel: Query<&mut Style, With<LandingPanel>>,
    mut text_queries: ParamSet<(Query<&Text>, Query<&mut Text, (With<LandingPanel>, Without<HudClock>, Without<SearchInput>)>)>,
    mut ns: ResMut<NextState<AppState>>,
) {
    if let Ok(mut p) = panel.get_single_mut() { p.display = Display::Flex; }
    for (interact, children) in interaction.iter() { if *interact == Interaction::Pressed { for &child in children.iter() { if let Ok(text) = text_queries.p0().get(child) { if let Some(section) = text.sections.first() { let v = &section.value;
        if v.contains("LAUNCH") || v.contains("X") { ns.set(AppState::Exploration); }
        else if v.contains("EDUCATIONAL") { ns.set(AppState::Educational); }
        else if v.contains("QUIZ") { ns.set(AppState::Quiz); }
        else if v.contains("GEO MAP") { ns.set(AppState::GeoMap); }
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

fn update_hud_clock(_time: Res<Time>, mut sim_time: ResMut<SimulationTime>, mut clock: Query<&mut Text, With<HudClock>>) {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let total_seconds = now.as_secs();
    let hours = (total_seconds / 3600) % 24;
    let minutes = (total_seconds / 60) % 60;
    let seconds = total_seconds % 60;
    sim_time.current_time = format!("{:02}:{:02}:{:02} IST", hours, minutes, seconds);
    sim_time.current_date = "Today".into();
    if let Ok(mut t) = clock.get_single_mut() { t.sections[0].value = format!("Dublin, Ireland  {}", sim_time.current_time); }
}

fn search_planets(keys: Res<ButtonInput<KeyCode>>, mut search: ResMut<SearchQuery>, mut st: Query<&mut Text, With<SearchInput>>) {
    if keys.just_pressed(KeyCode::Slash) { search.active = !search.active; }
    if search.active {
        for k in keys.get_just_pressed() { match k { KeyCode::Backspace => { search.text.pop(); } KeyCode::Enter | KeyCode::Escape => { search.text.clear(); search.active = false; } _ => {} } }
        if let Ok(mut t) = st.get_single_mut() { t.sections[0].value = format!(" SEARCH: [ {} ]", search.text); }
    }
}

fn update_planet_labels(settings: Res<AppSettings>, planets: Query<(&Planet, &Transform)>) { let _ = settings; let _ = planets; }

fn sync_health_from_physics(health: Res<Health>, mut ship: ResMut<PlayerShip>, current_state: Res<State<AppState>>) {
    // Only sync health from physics when NOT in combat mode to avoid conflicts
    if *current_state.get() != AppState::Combat {
        if health.is_changed() { ship.health = (health.hearts as f32 / health.max_hearts as f32) * 100.0; }
    }
}

fn update_hud_bars(
    ship: Res<PlayerShip>,
    mut bars: ParamSet<(
        Query<&mut Style, With<HudHealthBar>>,
        Query<&mut Style, With<HudShieldBar>>,
    )>,
) {
    if let Ok(mut style) = bars.p0().get_single_mut() {
        let hp = (ship.health / ship.max_health * 100.0).clamp(0.0, 100.0);
        style.width = Val::Percent(hp);
    }
    if let Ok(mut style) = bars.p1().get_single_mut() {
        let sp = (ship.shield / ship.max_shield * 100.0).clamp(0.0, 100.0);
        style.width = Val::Percent(sp);
    }
}

fn update_combat_ui(
    ship: Res<PlayerShip>, combat: Res<CombatState>,
    mut queries: ParamSet<(
        Query<&mut Style, With<CombatHealthBar>>,
        Query<&mut Style, With<CombatShieldBar>>,
        Query<&mut Text, With<CombatStatusText>>,
        Query<&mut Text, With<CombatResultStatsText>>,
    )>,
) {
    if !combat.active && !combat.has_won && !combat.has_lost { return; }
    if let Ok(mut style) = queries.p0().get_single_mut() { let hp = (ship.health / ship.max_health * 100.0).max(0.0); style.width = Val::Percent(hp); }
    if let Ok(mut style) = queries.p1().get_single_mut() { let sp = (ship.shield / ship.max_shield * 100.0).max(0.0); style.width = Val::Percent(sp); }
    if let Ok(mut text) = queries.p2().get_single_mut() {
        text.sections[0].value = format!("Ship Status: Health {:.0}%  Shield {:.0}%  Wave: {}  Kills: {}/{}", (ship.health / ship.max_health * 100.0).max(0.0), (ship.shield / ship.max_shield * 100.0).max(0.0), combat.wave, ship.kills, combat.total_to_kill);
    }
    if let Ok(mut text) = queries.p3().get_single_mut() {
        if combat.has_won {
            text.sections[0].value = format!("Kills: {} | Score: {} | Waves: {}", combat.enemy_count, ship.score, combat.wave);
        } else if combat.has_lost {
            text.sections[0].value = format!("Enemies killed: {} | Score: {} | Wave: {}", combat.enemy_count, ship.score, combat.wave);
        }
    }
}

fn update_enemy_bullets(mut commands: Commands, bullets: Query<(Entity, &mut Bullet, &mut Transform)>, mut ship: ResMut<PlayerShip>, combat: Res<CombatState>) {
    if !combat.active { return; }
    for (entity, _bullet, transform) in bullets.iter() {
        if transform.translation.length() > 50.0 { commands.entity(entity).despawn(); }
        if transform.translation.length() < 2.0 { 
            ship.health = (ship.health - 7.0).max(0.0); 
            commands.entity(entity).despawn(); 
        }
    }
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
        if full.starts_with("Night Sky") || full.starts_with("Cielo") || full.starts_with("Ciel") { text.sections[0].value = format!("{}: {}", translations.get(lang, "night_sky"), if settings.night_sky_enabled { translations.get(lang, "on") } else { translations.get(lang, "off") }); }
        else if full.starts_with("Quality") || full.starts_with("Calidad") || full.starts_with("Qualité") { text.sections[0].value = format!("{}: {}", translations.get(lang, "quality"), match settings.graphics_quality.as_str() { "High" => translations.get(lang, "high"), "Medium" => translations.get(lang, "medium_q"), _ => translations.get(lang, "low") }); }
        else if full.starts_with("Fullscreen") || full.starts_with("Pantalla") || full.starts_with("Plein") { text.sections[0].value = format!("{}: {}", translations.get(lang, "fullscreen"), if settings.fullscreen { translations.get(lang, "on") } else { translations.get(lang, "off") }); }
        else if full.starts_with("Show Orbits") || full.starts_with("Mostrar") || full.starts_with("Afficher") { text.sections[0].value = format!("{}: {}", translations.get(lang, "show_orbits"), if settings.show_planet_orbits { translations.get(lang, "on") } else { translations.get(lang, "off") }); }
        else if full.starts_with("Planet Labels") || full.starts_with("Etiquetas") || full.starts_with("Étiquettes") { text.sections[0].value = format!("{}: {}", translations.get(lang, "planet_labels"), if settings.show_planet_labels { translations.get(lang, "on") } else { translations.get(lang, "off") }); }
        else if full.starts_with("Show FPS") || full.starts_with("Mostrar FPS") || full.starts_with("Afficher FPS") { text.sections[0].value = format!("{}: {}", translations.get(lang, "show_fps"), if settings.show_fps { translations.get(lang, "on") } else { translations.get(lang, "off") }); }
        else if full.starts_with("Auto Rotate") || full.starts_with("Rotación") || full.starts_with("Rotation") { text.sections[0].value = format!("{}: {}", translations.get(lang, "auto_rotate"), if settings.auto_rotate { translations.get(lang, "on") } else { translations.get(lang, "off") }); }
        else if full.contains("Audio") { text.sections[0].value = format!("{}: {}", translations.get(lang, "audio"), if settings.audio_enabled { translations.get(lang, "on") } else { translations.get(lang, "off") }); }
        else if full.starts_with("Music") || full.starts_with("Música") || full.starts_with("Musique") { if full.ends_with('+') { text.sections[0].value = format!("{}: {:.0}% +", translations.get(lang, "music"), settings.music_volume * 100.0); } else { text.sections[0].value = format!("{}: {:.0}% -", translations.get(lang, "music"), settings.music_volume * 100.0); } }
        else if full.starts_with("SFX") || full.starts_with("EFX") || full.starts_with("Effets") { if full.ends_with('+') { text.sections[0].value = format!("{}: {:.0}% +", translations.get(lang, "sfx"), settings.sfx_volume * 100.0); } else { text.sections[0].value = format!("{}: {:.0}% -", translations.get(lang, "sfx"), settings.sfx_volume * 100.0); } }
        else if full.starts_with("Language") || full.starts_with("Idioma") || full.starts_with("Langue") { text.sections[0].value = format!("{}: {}", translations.get(lang, "language_label"), settings.language); }
        else if full.starts_with("Difficulty") || full.starts_with("Dificultad") || full.starts_with("Difficulté") { let dt = match settings.combat_difficulty.as_str() { "Easy" => translations.get(lang, "easy"), "Normal" => translations.get(lang, "normal"), _ => translations.get(lang, "hard") }; text.sections[0].value = format!("{}: {}", translations.get(lang, "difficulty"), dt); }
        else if full.starts_with("Shield") || full.starts_with("Escudo") || full.starts_with("Bouclier") { text.sections[0].value = format!("{}: {:.0}/{}", translations.get(lang, "shield"), ship.shield, ship.max_shield); }
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
    for (mut vis, mut text) in param_set.p0().iter_mut() { *vis = if settings.show_fps { Visibility::Visible } else { Visibility::Hidden }; if settings.show_fps { *last_fps_update += time.delta_seconds(); if *last_fps_update >= 0.5 { text.sections[0].value = format!("FPS: {:.0}", (1.0 / time.delta_seconds().max(0.001)).round()); *last_fps_update = 0.0; } } }
    for mut border in quality_border.iter_mut() { *border = match settings.graphics_quality.as_str() { "Low" => BorderColor(Color::rgb(0.8, 0.2, 0.2)), "Medium" => BorderColor(Color::rgb(0.8, 0.6, 0.2)), _ => BorderColor(Color::rgb(0.2, 0.8, 0.4)) }; }
    for mut bg in main_bg.iter_mut() { *bg = match settings.graphics_quality.as_str() { "Low" => BackgroundColor(Color::rgba(0.08, 0.02, 0.02, 0.95)), "Medium" => BackgroundColor(Color::rgba(0.08, 0.08, 0.02, 0.95)), _ => BackgroundColor(Color::rgba(0.0, 0.03, 0.08, 0.85)) }; }
    if settings.auto_rotate { let er = 0.3 * time.delta_seconds(); for (mut transform, _) in rot_query.iter_mut() { transform.rotate_y(er); } }
    for mut text in param_set.p1().iter_mut() { if text.sections[0].value.contains("Press [SPACE]") { text.sections[0].value = format!("Press [SPACE] to start combat! (Difficulty: {})", settings.combat_difficulty); } }
}

fn update_combat(
    mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>,
    mut combat: ResMut<CombatState>, keys: Res<ButtonInput<KeyCode>>, time: Res<Time>,
    mut transforms: ParamSet<(Query<&mut Transform, With<MainCamera>>, Query<(Entity, &mut Enemy, &mut Transform)>, Query<(Entity, &mut Bullet, &mut Transform)>, Query<(&Transform, &CelestialBody, &Visual)>)>,
    mut ship: ResMut<PlayerShip>,
    mut planet_hit_cooldown: Local<f32>,
) {
    if !combat.active || combat.has_won || combat.has_lost { return; }
    // Check if ship is already dead before allowing any movement
    if ship.health <= 0.0 {
        combat.has_lost = true;
        combat.active = false;
        return;
    }
    let cam_translation;
    { let mut cam_query = transforms.p0(); let Ok(mut cam) = cam_query.get_single_mut() else { return };
        if keys.pressed(KeyCode::KeyW) { cam.translation.z += 0.2; }
        if keys.pressed(KeyCode::KeyS) { cam.translation.z -= 0.2; }
        if keys.pressed(KeyCode::KeyA) { cam.translation.x -= 0.2; }
        if keys.pressed(KeyCode::KeyD) { cam.translation.x += 0.2; }
        // LASERS REMOVED - combat is now based on maneuvering and collision avoidance
        cam_translation = cam.translation;
    }
    let mut enemy_data = Vec::new();
    { let mut eq = transforms.p1(); for (entity, enemy, mut transform) in eq.iter_mut() { 
        // Move enemies toward the camera/ship position instead of (0,0,0)
        let dir = Vec3::new(cam_translation.x - transform.translation.x, 0.0, cam_translation.z - transform.translation.z); 
        if dir.length() > 0.0 { let dir = dir.normalize(); transform.translation += dir * enemy.speed * time.delta_seconds(); } 
        enemy_data.push((entity, enemy.health, enemy.reward_score, enemy.damage, transform.translation)); 
    } }
    // LASERS REMOVED - combat is now based on ramming enemies
    // Enemies are destroyed when the player rams into them (collision-based combat)
    let mut enemies_to_despawn: Vec<Entity> = Vec::new();
    for (ee, _, _, _ed, ep) in enemy_data.iter() {
        let dist_to_ship = cam_translation.distance(*ep);
        if dist_to_ship < 1.5 {
            // RAMMING: Player destroys the enemy by colliding with it
            // But takes damage from the impact (realistic space combat)
            ship.kills += 1;
            ship.score += 10 + combat.wave as u32 * 3;
            combat.enemy_count += 1;
            combat.wave = ((combat.enemy_count / 3) + 1).min(10);
            enemies_to_despawn.push(*ee);
            // Impact damage - ramming is risky but effective (~7 HP per ram)
            ship.health = (ship.health - 7.0).max(0.0);
            if ship.health <= 0.0 {
                combat.has_lost = true;
                combat.active = false;
                return;
            }
        }
    }
    for entity in enemies_to_despawn {
        if let Some(mut e) = commands.get_entity(entity) { e.despawn(); }
    }
    if combat.enemy_count >= combat.total_to_kill { combat.has_won = true; combat.active = false; return; }
    // Planet collision detection in combat mode (query is p3 in ParamSet - LAST access)
    // Collect planet data first to avoid simultaneous mutable borrows on transforms
    // NOTE: Do NOT re-access p0 here as ParamSet doesn't allow revisiting previous parameters
    let planet_data: Vec<(Vec3, f32)> = transforms.p3().iter().map(|(t, _body, v)| (t.translation, v.radius)).collect();
    // Cooldown prevents instant death: at most 1 hit per second so the health bar visibly drains
    *planet_hit_cooldown = (*planet_hit_cooldown - time.delta_seconds()).max(0.0);
    for (planet_pos, radius) in &planet_data {
        let distance = cam_translation.distance(*planet_pos);
        let collision_radius = radius + 5.0; // Increased collision radius for easier detection
        if distance < collision_radius {
            // Collision with planet/moon - reduce health a little bit (~4 HP per hit)
            // 1-second cooldown so the health bar visibly moves instead of instantly draining
            if *planet_hit_cooldown <= 0.0 {
                *planet_hit_cooldown = 1.0;
                ship.health = (ship.health - 4.0).max(0.0);
                if ship.health <= 0.0 {
                    combat.has_lost = true;
                    combat.active = false;
                    return;
                }
            }
            break;
        }
    }
    
    combat.spawn_timer -= time.delta_seconds();
    if combat.spawn_timer <= 0.0 { combat.spawn_timer = 2.0; let x = (rand() * 10.0) - 5.0; let z = (rand() * 10.0) - 5.0 - 25.0; let health = 20.0 + combat.wave as f32 * 3.0; let speed = 1.0 + combat.wave as f32 * 0.08; commands.spawn((PbrBundle { mesh: meshes.add(Sphere { radius: 0.15 }), material: materials.add(StandardMaterial { base_color: Color::rgb(1.0, 0.2, 0.0), emissive: Color::rgb(1.0, 0.3, 0.0), unlit: true, ..default() }), transform: Transform::from_xyz(x, cam_translation.y, cam_translation.z + z), ..default() }, Enemy { health, speed, damage: 5.0, reward_score: 10 + combat.wave as u32 * 3 })); }
}

fn destroy_asteroids_with_player_bullets(
    mut commands: Commands,
    bullets: Query<(Entity, &Transform), (With<PlayerBullet>, Without<Asteroid>)>,
    asteroids: Query<(Entity, &Transform), (With<Asteroid>, Without<PlayerBullet>)>,
) {
    let ap: Vec<(Entity, Vec3)> = asteroids.iter().map(|(e, t)| (e, t.translation)).collect();
    let bp: Vec<(Entity, Vec3)> = bullets.iter().map(|(e, t)| (e, t.translation)).collect();
    for (be, bpos) in &bp { for (ae, apos) in &ap { if bpos.distance(*apos) < 2.0 { if let Some(mut e) = commands.get_entity(*be) { e.despawn(); } if let Some(e) = commands.get_entity(*ae) { e.despawn_recursive(); } break; } } }
}

// ── SOUND EFFECTS ──
/// Play a click sound when any button is pressed, and a close sound when closing panels
fn play_button_sfx(
    mut commands: Commands,
    interaction: Query<(&Interaction, &Children), (Changed<Interaction>, With<Button>)>,
    button_texts: Query<&Text>,
    sfx: Res<SfxHandles>,
    settings: Res<AppSettings>,
) {
    if !settings.audio_enabled { return; }
    for (interact, children) in interaction.iter() {
        if *interact != Interaction::Pressed { continue; }
        let mut is_close = false;
        for &child in children.iter() {
            if let Ok(text) = button_texts.get(child) {
                if let Some(section) = text.sections.first() {
                    let v = &section.value;
                    if v == "X" || v.contains("CLOSE") || v.contains("Close") || v.contains("Cerrar") || v.contains("Fermer") || v.contains("LAUNCH") {
                        is_close = true;
                    }
                }
            }
        }
        let handle = if is_close { sfx.close.clone() } else { sfx.button_click.clone() };
        commands.spawn(AudioBundle {
            source: handle,
            settings: PlaybackSettings {
                mode: PlaybackMode::Despawn,
                volume: bevy::audio::Volume::new(settings.sfx_volume),
                ..default()
            },
        });
    }
}

/// Play the defeat sound when the player loses combat
fn play_lose_sfx(
    mut commands: Commands,
    combat: Res<CombatState>,
    sfx: Res<SfxHandles>,
    settings: Res<AppSettings>,
    mut last_lost: Local<bool>,
) {
    if !settings.audio_enabled { return; }
    if combat.has_lost && !*last_lost {
        commands.spawn(AudioBundle {
            source: sfx.lose.clone(),
            settings: PlaybackSettings {
                mode: PlaybackMode::Despawn,
                volume: bevy::audio::Volume::new(settings.sfx_volume),
                ..default()
            },
        });
    }
    *last_lost = combat.has_lost;
}

/// Play success/fail sound when the quiz is completed
fn play_quiz_sfx(
    mut commands: Commands,
    qs: Res<QuizState>,
    sfx: Res<SfxHandles>,
    settings: Res<AppSettings>,
    mut last_finished: Local<bool>,
) {
    if !settings.audio_enabled { return; }
    if qs.finished && !*last_finished {
        let handle = if qs.correct_count >= qs.wrong_count { sfx.quiz_success.clone() } else { sfx.quiz_fail.clone() };
        commands.spawn(AudioBundle {
            source: handle,
            settings: PlaybackSettings {
                mode: PlaybackMode::Despawn,
                volume: bevy::audio::Volume::new(settings.sfx_volume),
                ..default()
            },
        });
    }
    *last_finished = qs.finished;
}

/// Handle the OS window close button - exit the app when the window is closed
fn handle_window_close(
    mut exit: EventWriter<AppExit>,
    mut close_events: EventReader<bevy::window::WindowCloseRequested>,
) {
    for _ in close_events.read() {
        exit.send(AppExit);
    }
}

/// Control the background music volume and pause/play based on settings
fn control_music(
    settings: Res<AppSettings>,
    music_sinks: Query<&AudioSink>,
) {
    for sink in music_sinks.iter() {
        // Set volume based on music_volume setting
        sink.set_volume(settings.music_volume);
        // Pause or play based on audio_enabled
        if !settings.audio_enabled {
            sink.pause();
        } else {
            sink.play();
        }
    }
}
