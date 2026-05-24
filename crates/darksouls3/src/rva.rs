use pelite::pe64::{Pe, PeView};
use std::sync::LazyLock;
use thiserror::Error;
use windows::Win32::System::LibraryLoader::GetModuleHandleA;
use windows::core::PCSTR;

mod bundle;
mod rva_data;

pub use bundle::*;

const NAME: &str = "dark souls iii";

const LANG_ID_EN: u16 = 0x0009;
const LANG_ID_JP: u16 = 0x0011;

#[derive(Debug, Error)]
pub enum DetectError {
    #[error("Executable doesn't contain version metadata")]
    MissingVersionMetadata,
    #[error("Executable doesn't contain language metadata")]
    MissingLanguageMetadata,
    #[error("Executable doesn't contain product name metadata")]
    MissingProductName,
    #[error("Expected executable name to be \"{NAME}\", was \"{0}\"")]
    WrongProduct(String),
    #[error(
        "Expected executable language ID to be {LANG_ID_EN:#04x} or {LANG_ID_JP:#04x}, was {0:#04x}"
    )]
    UnsupportedLanguage(u16),
    #[error("Unsupported game version {0}")]
    UnsupportedVersion(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GameVersion {
    Ww1152,
    Jp11521,
}

impl GameVersion {
    fn detect(module: &PeView) -> Result<Self, DetectError> {
        let resources = module.resources().unwrap();
        let info = resources.version_info().unwrap();

        let product_version = info
            .fixed()
            .ok_or(DetectError::MissingVersionMetadata)?
            .dwProductVersion;
        let version = format!(
            "{}.{}.{}.{}",
            product_version.Major,
            product_version.Minor,
            product_version.Patch,
            product_version.Build,
        );

        let language = *info
            .translation()
            .first()
            .ok_or(DetectError::MissingLanguageMetadata)?;
        let mut product_name: Option<String> = None;
        info.strings(language, |k, v| {
            if k == "ProductName" {
                product_name = Some(v.to_string());
            }
        });

        let product = product_name.ok_or(DetectError::MissingProductName)?;
        let normalized = Self::normalize(&product);
        if normalized != NAME {
            return Err(DetectError::WrongProduct(product));
        }

        let lang_id = language.lang_id & 0x03FF;
        if lang_id != LANG_ID_EN && lang_id != LANG_ID_JP {
            return Err(DetectError::UnsupportedLanguage(lang_id));
        }

        match (lang_id, version.as_str()) {
            (LANG_ID_EN, "1.15.2.0") => Ok(Self::Ww1152),
            (LANG_ID_JP, "1.15.2.1") => Ok(Self::Jp11521),
            _ => Err(DetectError::UnsupportedVersion(version)),
        }
    }

    fn rvas(self) -> RvaBundle {
        match self {
            // For lack of a better option, we're assuming that (like DS3, unlike
            // Elden Ring) the RVAs are the same between the worldwide and Japanese
            // versions, but we haven't actually tested that yet. It's possible
            // we'll need to generate separate Japanese RVAs from a memory dump of
            // the decrypted Japanese executable.
            Self::Ww1152 => rva_data::RVAS,
            Self::Jp11521 => rva_data::RVAS,
        }
    }

    fn normalize(product: &str) -> String {
        product
            .chars()
            .filter(|c| c.is_alphanumeric() || c.is_whitespace())
            .collect::<String>()
            .to_lowercase()
    }
}

/// Returns the RVA bundle for the current executable region and version.
///
/// This will panic if the current executable isn't supported by this package.
pub fn get() -> &'static RvaBundle {
    static RVAS: LazyLock<RvaBundle> = LazyLock::new(|| {
        let module = unsafe {
            PeView::module(GetModuleHandleA(PCSTR(std::ptr::null())).unwrap().0 as *const u8)
        };
        GameVersion::detect(&module)
            .unwrap_or_else(|e| panic!("{e}"))
            .rvas()
    });

    &RVAS
}
