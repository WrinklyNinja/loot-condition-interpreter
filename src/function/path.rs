use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

use crate::{GameType, State};

const GHOST_EXTENSION: &str = "ghost";
const GHOST_EXTENSION_WITH_PERIOD: &str = ".ghost";

fn is_unghosted_plugin_file_extension(game_type: GameType, extension: &str) -> bool {
    match extension {
        "esp" | "esm" => true,
        "esl" if game_type.supports_light_plugins() => true,
        _ => false,
    }
}

fn has_unghosted_plugin_file_extension(game_type: GameType, path: &Path) -> bool {
    match path.extension().and_then(OsStr::to_str) {
        Some(ext) => is_unghosted_plugin_file_extension(game_type, ext),
        _ => false,
    }
}

pub fn has_plugin_file_extension(game_type: GameType, path: &Path) -> bool {
    match path.extension().and_then(OsStr::to_str) {
        Some(GHOST_EXTENSION) => path
            .file_stem()
            .map(|s| has_unghosted_plugin_file_extension(game_type, Path::new(s)))
            .unwrap_or(false),
        Some(ext) => is_unghosted_plugin_file_extension(game_type, ext),
        _ => false,
    }
}

fn add_ghost_extension(path: PathBuf) -> PathBuf {
    match path.extension() {
        Some(e) => {
            let mut new_extension = e.to_os_string();
            new_extension.push(GHOST_EXTENSION_WITH_PERIOD);
            path.with_extension(&new_extension)
        }
        None => path.with_extension(GHOST_EXTENSION),
    }
}

pub fn normalise_file_name<'a>(game_type: GameType, name: &'a str) -> &'a str {
    if name.ends_with(GHOST_EXTENSION_WITH_PERIOD) {
        let stem = &name[..name.len() - GHOST_EXTENSION_WITH_PERIOD.len()];
        if has_unghosted_plugin_file_extension(game_type, Path::new(stem)) {
            return stem;
        }
    }

    name
}

pub fn resolve_path(state: &State, path: &Path) -> PathBuf {
    if path == Path::new("LOOT") {
        state.loot_path.clone()
    } else {
        let path = state.data_path.join(path);

        if !path.exists() && has_unghosted_plugin_file_extension(state.game_type, &path) {
            add_ghost_extension(path)
        } else {
            path
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_unghosted_plugin_file_extension_should_be_true_for_esp_for_all_game_types() {
        let extension = "esp";

        assert!(is_unghosted_plugin_file_extension(
            GameType::Morrowind,
            extension
        ));
        assert!(is_unghosted_plugin_file_extension(
            GameType::Oblivion,
            extension
        ));
        assert!(is_unghosted_plugin_file_extension(
            GameType::Skyrim,
            extension
        ));
        assert!(is_unghosted_plugin_file_extension(
            GameType::SkyrimSE,
            extension
        ));
        assert!(is_unghosted_plugin_file_extension(
            GameType::SkyrimVR,
            extension
        ));
        assert!(is_unghosted_plugin_file_extension(
            GameType::Fallout3,
            extension
        ));
        assert!(is_unghosted_plugin_file_extension(
            GameType::FalloutNV,
            extension
        ));
        assert!(is_unghosted_plugin_file_extension(
            GameType::Fallout4,
            extension
        ));
        assert!(is_unghosted_plugin_file_extension(
            GameType::Fallout4VR,
            extension
        ));
    }

    #[test]
    fn is_unghosted_plugin_file_extension_should_be_true_for_esm_for_all_game_types() {
        let extension = "esm";

        assert!(is_unghosted_plugin_file_extension(
            GameType::Morrowind,
            extension
        ));
        assert!(is_unghosted_plugin_file_extension(
            GameType::Oblivion,
            extension
        ));
        assert!(is_unghosted_plugin_file_extension(
            GameType::Skyrim,
            extension
        ));
        assert!(is_unghosted_plugin_file_extension(
            GameType::SkyrimSE,
            extension
        ));
        assert!(is_unghosted_plugin_file_extension(
            GameType::SkyrimVR,
            extension
        ));
        assert!(is_unghosted_plugin_file_extension(
            GameType::Fallout3,
            extension
        ));
        assert!(is_unghosted_plugin_file_extension(
            GameType::FalloutNV,
            extension
        ));
        assert!(is_unghosted_plugin_file_extension(
            GameType::Fallout4,
            extension
        ));
        assert!(is_unghosted_plugin_file_extension(
            GameType::Fallout4VR,
            extension
        ));
    }

    #[test]
    fn is_unghosted_plugin_file_extension_should_be_true_for_esl_for_tes5se_tes5vr_fo4_and_fo4vr() {
        let extension = "esl";

        assert!(is_unghosted_plugin_file_extension(
            GameType::SkyrimSE,
            extension
        ));
        assert!(is_unghosted_plugin_file_extension(
            GameType::SkyrimVR,
            extension
        ));
        assert!(is_unghosted_plugin_file_extension(
            GameType::Fallout4,
            extension
        ));
        assert!(is_unghosted_plugin_file_extension(
            GameType::Fallout4VR,
            extension
        ));
    }

    #[test]
    fn is_unghosted_plugin_file_extension_should_be_false_for_esl_for_tes3_to_5_fo3_and_fonv() {
        let extension = "esl";

        assert!(!is_unghosted_plugin_file_extension(
            GameType::Morrowind,
            extension
        ));
        assert!(!is_unghosted_plugin_file_extension(
            GameType::Oblivion,
            extension
        ));
        assert!(!is_unghosted_plugin_file_extension(
            GameType::Skyrim,
            extension
        ));
        assert!(!is_unghosted_plugin_file_extension(
            GameType::Fallout3,
            extension
        ));
        assert!(!is_unghosted_plugin_file_extension(
            GameType::FalloutNV,
            extension
        ));
    }

    #[test]
    fn is_unghosted_plugin_file_extension_should_be_false_for_ghost_for_all_game_types() {
        let extension = "ghost";

        assert!(!is_unghosted_plugin_file_extension(
            GameType::Morrowind,
            extension
        ));
        assert!(!is_unghosted_plugin_file_extension(
            GameType::Oblivion,
            extension
        ));
        assert!(!is_unghosted_plugin_file_extension(
            GameType::Skyrim,
            extension
        ));
        assert!(!is_unghosted_plugin_file_extension(
            GameType::SkyrimSE,
            extension
        ));
        assert!(!is_unghosted_plugin_file_extension(
            GameType::SkyrimVR,
            extension
        ));
        assert!(!is_unghosted_plugin_file_extension(
            GameType::Fallout3,
            extension
        ));
        assert!(!is_unghosted_plugin_file_extension(
            GameType::FalloutNV,
            extension
        ));
        assert!(!is_unghosted_plugin_file_extension(
            GameType::Fallout4,
            extension
        ));
        assert!(!is_unghosted_plugin_file_extension(
            GameType::Fallout4VR,
            extension
        ));
    }

    #[test]
    fn is_unghosted_plugin_file_extension_should_be_false_for_non_esp_esm_esl_for_all_game_types() {
        let extension = "txt";

        assert!(!is_unghosted_plugin_file_extension(
            GameType::Morrowind,
            extension
        ));
        assert!(!is_unghosted_plugin_file_extension(
            GameType::Oblivion,
            extension
        ));
        assert!(!is_unghosted_plugin_file_extension(
            GameType::Skyrim,
            extension
        ));
        assert!(!is_unghosted_plugin_file_extension(
            GameType::SkyrimSE,
            extension
        ));
        assert!(!is_unghosted_plugin_file_extension(
            GameType::SkyrimVR,
            extension
        ));
        assert!(!is_unghosted_plugin_file_extension(
            GameType::Fallout3,
            extension
        ));
        assert!(!is_unghosted_plugin_file_extension(
            GameType::FalloutNV,
            extension
        ));
        assert!(!is_unghosted_plugin_file_extension(
            GameType::Fallout4,
            extension
        ));
        assert!(!is_unghosted_plugin_file_extension(
            GameType::Fallout4VR,
            extension
        ));
    }

    #[test]
    fn has_unghosted_plugin_file_extension_should_return_false_if_the_path_has_no_extension() {
        assert!(!has_unghosted_plugin_file_extension(
            GameType::Skyrim,
            Path::new("file")
        ));
    }

    #[test]
    fn has_unghosted_plugin_file_extension_should_return_false_if_the_path_has_a_non_plugin_extension(
    ) {
        assert!(!has_unghosted_plugin_file_extension(
            GameType::Skyrim,
            Path::new("plugin.bsa")
        ));
    }

    #[test]
    fn has_unghosted_plugin_file_extension_should_return_false_if_the_path_has_a_ghosted_plugin_extension(
    ) {
        assert!(!has_unghosted_plugin_file_extension(
            GameType::Skyrim,
            Path::new("plugin.esp.ghost")
        ));
    }

    #[test]
    fn has_unghosted_plugin_file_extension_should_return_true_if_the_path_has_an_unghosted_plugin_extension(
    ) {
        assert!(has_unghosted_plugin_file_extension(
            GameType::Skyrim,
            Path::new("plugin.esp")
        ));
    }

    #[test]
    fn has_plugin_file_extension_should_return_true_if_the_path_has_an_unghosted_plugin_extension()
    {
        assert!(has_plugin_file_extension(
            GameType::Skyrim,
            Path::new("plugin.esp")
        ));
    }

    #[test]
    fn has_plugin_file_extension_should_return_true_if_the_path_has_a_ghosted_plugin_extension() {
        assert!(has_plugin_file_extension(
            GameType::Skyrim,
            Path::new("plugin.esp.ghost")
        ));
    }

    #[test]
    fn has_plugin_file_extension_should_return_false_if_the_path_has_a_non_plugin_extension() {
        assert!(!has_plugin_file_extension(
            GameType::Skyrim,
            Path::new("plugin.bsa")
        ));
    }

    #[test]
    fn has_plugin_file_extension_should_return_false_if_the_path_has_a_ghosted_non_plugin_extension(
    ) {
        assert!(!has_plugin_file_extension(
            GameType::Skyrim,
            Path::new("plugin.bsa.ghost")
        ));
    }

    #[test]
    fn has_plugin_file_extension_should_return_false_if_the_path_has_only_ghost_extension() {
        assert!(!has_plugin_file_extension(
            GameType::Skyrim,
            Path::new("plugin.ghost")
        ));
    }

    #[test]
    fn has_plugin_file_extension_should_return_false_if_the_path_has_no_extension() {
        assert!(!has_plugin_file_extension(
            GameType::Skyrim,
            Path::new("plugin")
        ));
    }

    #[test]
    fn add_ghost_extension_should_add_dot_ghost_to_an_existing_extension() {
        let path = add_ghost_extension("plugin.esp".into());
        assert_eq!(PathBuf::from("plugin.esp.ghost"), path);
    }

    #[test]
    fn add_ghost_extension_should_add_dot_ghost_to_an_a_path_with_no_extension() {
        let path = add_ghost_extension("plugin".into());
        assert_eq!(PathBuf::from("plugin.ghost"), path);
    }

    #[test]
    #[allow(non_snake_case)]
    fn resolve_path_should_return_loot_path_if_given_LOOT() {
        let loot_path = PathBuf::from("loot.exe");
        let state = State::new(GameType::Skyrim, "data".into(), loot_path.clone());
        let path = resolve_path(&state, Path::new("LOOT"));

        assert_eq!(loot_path, path);
    }

    #[test]
    fn resolve_path_should_return_the_data_path_prefixed_path_if_it_exists() {
        let data_path = PathBuf::from(".");
        let state = State::new(GameType::Skyrim, data_path.clone(), "loot.exe".into());
        let input_path = Path::new("README.md");
        let resolved_path = resolve_path(&state, input_path);

        assert_eq!(data_path.join(input_path), resolved_path);
    }

    #[test]
    fn resolve_path_should_return_the_data_path_prefixed_path_if_it_does_not_exist_and_is_not_an_unghosted_plugin_filename(
    ) {
        let data_path = PathBuf::from(".");
        let state = State::new(GameType::Skyrim, data_path.clone(), "loot.exe".into());
        let input_path = Path::new("plugin.esp.ghost");
        let resolved_path = resolve_path(&state, input_path);

        assert_eq!(data_path.join(input_path), resolved_path);

        let input_path = Path::new("file.txt");
        let resolved_path = resolve_path(&state, input_path);

        assert_eq!(data_path.join(input_path), resolved_path);
    }

    #[test]
    fn resolve_path_should_return_the_given_data_relative_path_plus_a_ghost_extension_if_the_plugin_path_does_not_exist(
    ) {
        let data_path = PathBuf::from(".");
        let state = State::new(GameType::Skyrim, data_path.clone(), "loot.exe".into());
        let input_path = Path::new("plugin.esp");
        let resolved_path = resolve_path(&state, input_path);

        assert_eq!(
            data_path.join(input_path.with_extension("esp.ghost")),
            resolved_path
        );
    }
}
