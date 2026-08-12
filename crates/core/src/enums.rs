//! Type-safe enums for the Galactic Explorer domain.
//! These drive all game logic via pattern matching, avoiding stringly-typed code.

use bevy::prelude::Component;
use serde::{Deserialize, Serialize};

/// All discoverable celestial bodies in the game.
#[derive(Component, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum PlanetKind {
    Mercury,
    Venus,
    Earth,
    Moon,
    Mars,
    Jupiter,
    Saturn,
    Uranus,
    Neptune,
    Ceres,
    Eris,
    Haumea,
    Makemake,
    Sun,
}

impl PlanetKind {
    pub const ALL: [PlanetKind; 14] = [
        PlanetKind::Mercury,
        PlanetKind::Venus,
        PlanetKind::Earth,
        PlanetKind::Moon,
        PlanetKind::Mars,
        PlanetKind::Jupiter,
        PlanetKind::Saturn,
        PlanetKind::Uranus,
        PlanetKind::Neptune,
        PlanetKind::Ceres,
        PlanetKind::Eris,
        PlanetKind::Haumea,
        PlanetKind::Makemake,
        PlanetKind::Sun,
    ];

    pub fn display_name(self) -> &'static str {
        match self {
            PlanetKind::Mercury => "Mercury",
            PlanetKind::Venus => "Venus",
            PlanetKind::Earth => "Earth",
            PlanetKind::Moon => "Moon",
            PlanetKind::Mars => "Mars",
            PlanetKind::Jupiter => "Jupiter",
            PlanetKind::Saturn => "Saturn",
            PlanetKind::Uranus => "Uranus",
            PlanetKind::Neptune => "Neptune",
            PlanetKind::Ceres => "Ceres",
            PlanetKind::Eris => "Eris",
            PlanetKind::Haumea => "Haumea",
            PlanetKind::Makemake => "Makemake",
            PlanetKind::Sun => "Sun",
        }
    }

    pub fn texture_path(self) -> &'static str {
        match self {
            PlanetKind::Mercury => "textures/mercury.png",
            PlanetKind::Venus => "textures/venus.png",
            PlanetKind::Earth => "textures/earth_map.png",
            PlanetKind::Moon => "textures/moon.png",
            PlanetKind::Mars => "textures/mars_map.png",
            PlanetKind::Jupiter => "textures/jupiter.png",
            PlanetKind::Saturn => "textures/saturn.png",
            PlanetKind::Uranus => "textures/uranus.png",
            PlanetKind::Neptune => "textures/neptune.png",
            PlanetKind::Ceres => "textures/ceres.png",
            PlanetKind::Eris => "textures/eris.png",
            PlanetKind::Haumea => "textures/haumea.png",
            PlanetKind::Makemake => "textures/makemake.png",
            PlanetKind::Sun => "textures/sun.png",
        }
    }

    pub fn radius(self) -> f32 {
        match self {
            PlanetKind::Mercury => 1.4,
            PlanetKind::Venus => 2.1,
            PlanetKind::Earth => 2.6,
            PlanetKind::Moon => 1.1,
            PlanetKind::Mars => 2.2,
            PlanetKind::Jupiter => 5.2,
            PlanetKind::Saturn => 4.8,
            PlanetKind::Uranus => 3.3,
            PlanetKind::Neptune => 3.4,
            PlanetKind::Ceres => 0.6,
            PlanetKind::Eris => 0.8,
            PlanetKind::Haumea => 0.7,
            PlanetKind::Makemake => 0.7,
            PlanetKind::Sun => 7.8,
        }
    }

    pub fn lore(self) -> &'static str {
        match self {
            PlanetKind::Mercury => "🌑 MERCURY - The Swift Wanderer\nDiameter: 4,879 km | Gravity: 3.7 m/s²\n• No atmosphere, extreme temperature variations (-173°C to 427°C)\n• Iron core makes up 75% of planet's radius\n• Mission Notes: Avoid long exposure during solar flares.",
            PlanetKind::Venus => "🌕 VENUS - Earth's Toxic Twin\nDiameter: 12,104 km | Gravity: 8.87 m/s²\n• Dense CO2 atmosphere creates runaway greenhouse effect\n• Surface pressure 92 times Earth's\n• Mission Notes: Acid-resistant coating required.",
            PlanetKind::Earth => "🌍 EARTH - The Blue Marble\nDiameter: 12,742 km | Gravity: 9.8 m/s²\n• Only known planet with liquid water on surface\n• 71% ocean coverage\n• Mission Notes: Home base. Optimal conditions.",
            PlanetKind::Moon => "🌙 LUNA - Earth's Silent Companion\nDiameter: 3,474 km | Gravity: 1.62 m/s²\n• Formed from debris after Mars-sized impact\n• No atmosphere, temperature extremes -173°C to 127°C\n• Mission Notes: Excellent staging ground.",
            PlanetKind::Mars => "🔴 MARS - The Red Planet\nDiameter: 6,779 km | Gravity: 3.71 m/s²\n• Iron oxide surface gives distinctive red color\n• Olympus Mons: tallest volcano (21 km)\n• Mission Notes: Prime colonization target.",
            PlanetKind::Jupiter => "🟠 JUPITER - King of Planets\nDiameter: 139,820 km | Gravity: 24.79 m/s²\n• Great Red Spot: storm larger than Earth\n• 79+ moons including four Galilean moons\n• Mission Notes: Avoid radiation belts.",
            PlanetKind::Saturn => "🪐 SATURN - The Ringed Beauty\nDiameter: 116,460 km | Gravity: 10.44 m/s²\n• Ring system: billions of ice and rock particles\n• 82+ moons including Titan and Enceladus\n• Mission Notes: Rings navigation hazard.",
            PlanetKind::Uranus => "🔵 URANUS - The Tilted Giant\nDiameter: 50,724 km | Gravity: 8.87 m/s²\n• Extreme axial tilt: 98 degrees\n• Methane atmosphere gives blue-green color\n• Mission Notes: Extreme cold requires special equipment.",
            PlanetKind::Neptune => "🟦 NEPTUNE - The Windy World\nDiameter: 49,244 km | Gravity: 11.15 m/s²\n• Fastest winds: up to 2,100 km/h\n• Deep blue color from atmospheric methane\n• Mission Notes: Extreme winds make landing dangerous.",
            PlanetKind::Sun => "☀️ SOL - Our Living Star\nDiameter: 1.39M km | Gravity: 274 m/s²\n• 99.86% of solar system's mass\n• Core temperature: 15 million°C\n• Mission Notes: Extreme radiation zone.",
            PlanetKind::Ceres => "🌑 CERES - The Asteroid Queen\nDiameter: 946 km | Gravity: 0.27 m/s²\n• Largest object in asteroid belt\n• Possible subsurface water reservoir\n• Mission Notes: Water ice potential for fuel.",
            PlanetKind::Eris => "⚪ ERIS - The Distant World\nDiameter: 2,377 km | Gravity: 0.62 m/s²\n• Reclassified as dwarf planet in 2006\n• Heart-shaped glacier the size of Texas\n• Mission Notes: Extreme cold requires nuclear power.",
            PlanetKind::Haumea => "⚪ HAUMEA - The Elliptical Dwarf\nDiameter: 1,900 km | Gravity: 0.45 m/s²\n• Highly elliptical orbit\n• Two moons: Hiʻiaka and Namaka\n• Mission Notes: Unique orbital characteristics.",
            PlanetKind::Makemake => "⚪ MAKEMAKE - The Red Dwarf\nDiameter: 1,400 km | Gravity: 0.5 m/s²\n• Located in the Kuiper Belt\n• Surface covered in frozen methane\n• Mission Notes: Extreme cold requires nuclear power.",
        }
    }

    pub fn from_query(query: &str) -> Option<Self> {
        let needle = query.trim().to_ascii_lowercase();
        if needle.is_empty() {
            return None;
        }
        PlanetKind::ALL
            .iter()
            .copied()
            .find(|kind| kind.display_name().to_ascii_lowercase().contains(&needle))
    }
}

/// Game difficulty levels that affect all gameplay mechanics.
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, Debug)]
pub enum GameDifficulty {
    #[default]
    Easy,
    Medium,
    Hard,
    Extreme,
}

impl GameDifficulty {
    pub fn next(self) -> Self {
        match self {
            GameDifficulty::Easy => GameDifficulty::Medium,
            GameDifficulty::Medium => GameDifficulty::Hard,
            GameDifficulty::Hard => GameDifficulty::Extreme,
            GameDifficulty::Extreme => GameDifficulty::Easy,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            GameDifficulty::Easy => "Easy",
            GameDifficulty::Medium => "Medium",
            GameDifficulty::Hard => "Hard",
            GameDifficulty::Extreme => "Extreme",
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            GameDifficulty::Easy => "Forgiving flight controls, generous scanner range",
            GameDifficulty::Medium => "Standard flight physics, balanced scanner range",
            GameDifficulty::Hard => "Realistic physics, narrow scanner range",
            GameDifficulty::Extreme => "Harsh environment, minimal assistance",
        }
    }

    pub fn fuel_consumption_rate(self) -> f32 {
        match self {
            GameDifficulty::Easy => 0.5,
            GameDifficulty::Medium => 1.0,
            GameDifficulty::Hard => 1.5,
            GameDifficulty::Extreme => 2.0,
        }
    }

    pub fn asteroid_damage(self) -> f32 {
        match self {
            GameDifficulty::Easy => 5.0,
            GameDifficulty::Medium => 10.0,
            GameDifficulty::Hard => 20.0,
            GameDifficulty::Extreme => 40.0,
        }
    }

    pub fn scanner_range(self) -> f32 {
        match self {
            GameDifficulty::Easy => 15.0,
            GameDifficulty::Medium => 10.0,
            GameDifficulty::Hard => 7.0,
            GameDifficulty::Extreme => 5.0,
        }
    }
}

/// Graphics quality presets.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum GraphicsQuality {
    Low,
    Medium,
    High,
    Ultra,
}

impl GraphicsQuality {
    pub fn next(self) -> Self {
        match self {
            GraphicsQuality::Low => GraphicsQuality::Medium,
            GraphicsQuality::Medium => GraphicsQuality::High,
            GraphicsQuality::High => GraphicsQuality::Ultra,
            GraphicsQuality::Ultra => GraphicsQuality::Low,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            GraphicsQuality::Low => "Low",
            GraphicsQuality::Medium => "Medium",
            GraphicsQuality::High => "High",
            GraphicsQuality::Ultra => "Ultra",
        }
    }
}

/// Planet detail level.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PlanetDetail {
    Basic,
    Standard,
    Detailed,
    Realistic,
}

impl PlanetDetail {
    pub fn next(self) -> Self {
        match self {
            PlanetDetail::Basic => PlanetDetail::Standard,
            PlanetDetail::Standard => PlanetDetail::Detailed,
            PlanetDetail::Detailed => PlanetDetail::Realistic,
            PlanetDetail::Realistic => PlanetDetail::Basic,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            PlanetDetail::Basic => "Basic",
            PlanetDetail::Standard => "Standard",
            PlanetDetail::Detailed => "Detailed",
            PlanetDetail::Realistic => "Realistic",
        }
    }
}

/// Menu button actions for clickable UI.
#[derive(Component, Clone, Copy, PartialEq, Eq, Debug)]
pub struct MenuButton(pub MenuAction);

/// Menu button action types.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MenuAction {
    Start,
    Settings,
    Help,
    Learning,
    Combat,
    Back,
}

/// Screen/menu navigation states.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ScreenMode {
    Welcome,
    Loading,
    Settings,
    Help,
    Learning,
    Combat,
    Playing,
}

/// Virtual touch control actions.
#[derive(Clone, Copy, Debug)]
pub enum TouchAction {
    Forward,
    Backward,
    Left,
    Right,
    Up,
    Down,
    YawLeft,
    YawRight,
    Boost,
}

/// Camera modes.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CameraView {
    Chase,
    Cockpit,
    Overview,
}
