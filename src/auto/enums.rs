// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::translate::*;
use std::fmt;
use webkit2_webextension_sys;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum ContextMenuAction {
    NoAction,
    OpenLink,
    OpenLinkInNewWindow,
    DownloadLinkToDisk,
    CopyLinkToClipboard,
    OpenImageInNewWindow,
    DownloadImageToDisk,
    CopyImageToClipboard,
    CopyImageUrlToClipboard,
    OpenFrameInNewWindow,
    GoBack,
    GoForward,
    Stop,
    Reload,
    Copy,
    Cut,
    Paste,
    Delete,
    SelectAll,
    InputMethods,
    Unicode,
    SpellingGuess,
    NoGuessesFound,
    IgnoreSpelling,
    LearnSpelling,
    IgnoreGrammar,
    FontMenu,
    Bold,
    Italic,
    Underline,
    Outline,
    InspectElement,
    OpenVideoInNewWindow,
    OpenAudioInNewWindow,
    CopyVideoLinkToClipboard,
    CopyAudioLinkToClipboard,
    ToggleMediaControls,
    ToggleMediaLoop,
    EnterVideoFullscreen,
    MediaPlay,
    MediaPause,
    MediaMute,
    DownloadVideoToDisk,
    DownloadAudioToDisk,
    InsertEmoji,
    PasteAsPlainText,
    Custom,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for ContextMenuAction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ContextMenuAction::{}",
            match *self {
                ContextMenuAction::NoAction => "NoAction",
                ContextMenuAction::OpenLink => "OpenLink",
                ContextMenuAction::OpenLinkInNewWindow => "OpenLinkInNewWindow",
                ContextMenuAction::DownloadLinkToDisk => "DownloadLinkToDisk",
                ContextMenuAction::CopyLinkToClipboard => "CopyLinkToClipboard",
                ContextMenuAction::OpenImageInNewWindow => "OpenImageInNewWindow",
                ContextMenuAction::DownloadImageToDisk => "DownloadImageToDisk",
                ContextMenuAction::CopyImageToClipboard => "CopyImageToClipboard",
                ContextMenuAction::CopyImageUrlToClipboard => "CopyImageUrlToClipboard",
                ContextMenuAction::OpenFrameInNewWindow => "OpenFrameInNewWindow",
                ContextMenuAction::GoBack => "GoBack",
                ContextMenuAction::GoForward => "GoForward",
                ContextMenuAction::Stop => "Stop",
                ContextMenuAction::Reload => "Reload",
                ContextMenuAction::Copy => "Copy",
                ContextMenuAction::Cut => "Cut",
                ContextMenuAction::Paste => "Paste",
                ContextMenuAction::Delete => "Delete",
                ContextMenuAction::SelectAll => "SelectAll",
                ContextMenuAction::InputMethods => "InputMethods",
                ContextMenuAction::Unicode => "Unicode",
                ContextMenuAction::SpellingGuess => "SpellingGuess",
                ContextMenuAction::NoGuessesFound => "NoGuessesFound",
                ContextMenuAction::IgnoreSpelling => "IgnoreSpelling",
                ContextMenuAction::LearnSpelling => "LearnSpelling",
                ContextMenuAction::IgnoreGrammar => "IgnoreGrammar",
                ContextMenuAction::FontMenu => "FontMenu",
                ContextMenuAction::Bold => "Bold",
                ContextMenuAction::Italic => "Italic",
                ContextMenuAction::Underline => "Underline",
                ContextMenuAction::Outline => "Outline",
                ContextMenuAction::InspectElement => "InspectElement",
                ContextMenuAction::OpenVideoInNewWindow => "OpenVideoInNewWindow",
                ContextMenuAction::OpenAudioInNewWindow => "OpenAudioInNewWindow",
                ContextMenuAction::CopyVideoLinkToClipboard => "CopyVideoLinkToClipboard",
                ContextMenuAction::CopyAudioLinkToClipboard => "CopyAudioLinkToClipboard",
                ContextMenuAction::ToggleMediaControls => "ToggleMediaControls",
                ContextMenuAction::ToggleMediaLoop => "ToggleMediaLoop",
                ContextMenuAction::EnterVideoFullscreen => "EnterVideoFullscreen",
                ContextMenuAction::MediaPlay => "MediaPlay",
                ContextMenuAction::MediaPause => "MediaPause",
                ContextMenuAction::MediaMute => "MediaMute",
                ContextMenuAction::DownloadVideoToDisk => "DownloadVideoToDisk",
                ContextMenuAction::DownloadAudioToDisk => "DownloadAudioToDisk",
                ContextMenuAction::InsertEmoji => "InsertEmoji",
                ContextMenuAction::PasteAsPlainText => "PasteAsPlainText",
                ContextMenuAction::Custom => "Custom",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for ContextMenuAction {
    type GlibType = webkit2_webextension_sys::WebKitContextMenuAction;

    fn to_glib(&self) -> webkit2_webextension_sys::WebKitContextMenuAction {
        match *self {
            ContextMenuAction::NoAction => {
                webkit2_webextension_sys::WEBKIT_CONTEXT_MENU_ACTION_NO_ACTION
            }
            ContextMenuAction::OpenLink => {
                webkit2_webextension_sys::WEBKIT_CONTEXT_MENU_ACTION_OPEN_LINK
            }
            ContextMenuAction::OpenLinkInNewWindow => {
                webkit2_webextension_sys::WEBKIT_CONTEXT_MENU_ACTION_OPEN_LINK_IN_NEW_WINDOW
            }
            ContextMenuAction::DownloadLinkToDisk => {
                webkit2_webextension_sys::WEBKIT_CONTEXT_MENU_ACTION_DOWNLOAD_LINK_TO_DISK
            }
            ContextMenuAction::CopyLinkToClipboard => {
                webkit2_webextension_sys::WEBKIT_CONTEXT_MENU_ACTION_COPY_LINK_TO_CLIPBOARD
            }
            ContextMenuAction::OpenImageInNewWindow => {
                webkit2_webextension_sys::WEBKIT_CONTEXT_MENU_ACTION_OPEN_IMAGE_IN_NEW_WINDOW
            }
            ContextMenuAction::DownloadImageToDisk => {
                webkit2_webextension_sys::WEBKIT_CONTEXT_MENU_ACTION_DOWNLOAD_IMAGE_TO_DISK
            }
            ContextMenuAction::CopyImageToClipboard => {
                webkit2_webextension_sys::WEBKIT_CONTEXT_MENU_ACTION_COPY_IMAGE_TO_CLIPBOARD
            }
            ContextMenuAction::CopyImageUrlToClipboard => {
                webkit2_webextension_sys::WEBKIT_CONTEXT_MENU_ACTION_COPY_IMAGE_URL_TO_CLIPBOARD
            }
            ContextMenuAction::OpenFrameInNewWindow => {
                webkit2_webextension_sys::WEBKIT_CONTEXT_MENU_ACTION_OPEN_FRAME_IN_NEW_WINDOW
            }
            ContextMenuAction::GoBack => {
                webkit2_webextension_sys::WEBKIT_CONTEXT_MENU_ACTION_GO_BACK
            }
            ContextMenuAction::GoForward => {
                webkit2_webextension_sys::WEBKIT_CONTEXT_MENU_ACTION_GO_FORWARD
            }
            ContextMenuAction::Stop => webkit2_webextension_sys::WEBKIT_CONTEXT_MENU_ACTION_STOP,
            ContextMenuAction::Reload => {
                webkit2_webextension_sys::WEBKIT_CONTEXT_MENU_ACTION_RELOAD
            }
            ContextMenuAction::Copy => webkit2_webextension_sys::WEBKIT_CONTEXT_MENU_ACTION_COPY,
            ContextMenuAction::Cut => webkit2_webextension_sys::WEBKIT_CONTEXT_MENU_ACTION_CUT,
            ContextMenuAction::Paste => webkit2_webextension_sys::WEBKIT_CONTEXT_MENU_ACTION_PASTE,
            ContextMenuAction::Delete => {
                webkit2_webextension_sys::WEBKIT_CONTEXT_MENU_ACTION_DELETE
            }
            ContextMenuAction::SelectAll => {
                webkit2_webextension_sys::WEBKIT_CONTEXT_MENU_ACTION_SELECT_ALL
            }
            ContextMenuAction::InputMethods => {
                webkit2_webextension_sys::WEBKIT_CONTEXT_MENU_ACTION_INPUT_METHODS
            }
            ContextMenuAction::Unicode => {
                webkit2_webextension_sys::WEBKIT_CONTEXT_MENU_ACTION_UNICODE
            }
            ContextMenuAction::SpellingGuess => {
                webkit2_webextension_sys::WEBKIT_CONTEXT_MENU_ACTION_SPELLING_GUESS
            }
            ContextMenuAction::NoGuessesFound => {
                webkit2_webextension_sys::WEBKIT_CONTEXT_MENU_ACTION_NO_GUESSES_FOUND
            }
            ContextMenuAction::IgnoreSpelling => {
                webkit2_webextension_sys::WEBKIT_CONTEXT_MENU_ACTION_IGNORE_SPELLING
            }
            ContextMenuAction::LearnSpelling => {
                webkit2_webextension_sys::WEBKIT_CONTEXT_MENU_ACTION_LEARN_SPELLING
            }
            ContextMenuAction::IgnoreGrammar => {
                webkit2_webextension_sys::WEBKIT_CONTEXT_MENU_ACTION_IGNORE_GRAMMAR
            }
            ContextMenuAction::FontMenu => {
                webkit2_webextension_sys::WEBKIT_CONTEXT_MENU_ACTION_FONT_MENU
            }
            ContextMenuAction::Bold => webkit2_webextension_sys::WEBKIT_CONTEXT_MENU_ACTION_BOLD,
            ContextMenuAction::Italic => {
                webkit2_webextension_sys::WEBKIT_CONTEXT_MENU_ACTION_ITALIC
            }
            ContextMenuAction::Underline => {
                webkit2_webextension_sys::WEBKIT_CONTEXT_MENU_ACTION_UNDERLINE
            }
            ContextMenuAction::Outline => {
                webkit2_webextension_sys::WEBKIT_CONTEXT_MENU_ACTION_OUTLINE
            }
            ContextMenuAction::InspectElement => {
                webkit2_webextension_sys::WEBKIT_CONTEXT_MENU_ACTION_INSPECT_ELEMENT
            }
            ContextMenuAction::OpenVideoInNewWindow => {
                webkit2_webextension_sys::WEBKIT_CONTEXT_MENU_ACTION_OPEN_VIDEO_IN_NEW_WINDOW
            }
            ContextMenuAction::OpenAudioInNewWindow => {
                webkit2_webextension_sys::WEBKIT_CONTEXT_MENU_ACTION_OPEN_AUDIO_IN_NEW_WINDOW
            }
            ContextMenuAction::CopyVideoLinkToClipboard => {
                webkit2_webextension_sys::WEBKIT_CONTEXT_MENU_ACTION_COPY_VIDEO_LINK_TO_CLIPBOARD
            }
            ContextMenuAction::CopyAudioLinkToClipboard => {
                webkit2_webextension_sys::WEBKIT_CONTEXT_MENU_ACTION_COPY_AUDIO_LINK_TO_CLIPBOARD
            }
            ContextMenuAction::ToggleMediaControls => {
                webkit2_webextension_sys::WEBKIT_CONTEXT_MENU_ACTION_TOGGLE_MEDIA_CONTROLS
            }
            ContextMenuAction::ToggleMediaLoop => {
                webkit2_webextension_sys::WEBKIT_CONTEXT_MENU_ACTION_TOGGLE_MEDIA_LOOP
            }
            ContextMenuAction::EnterVideoFullscreen => {
                webkit2_webextension_sys::WEBKIT_CONTEXT_MENU_ACTION_ENTER_VIDEO_FULLSCREEN
            }
            ContextMenuAction::MediaPlay => {
                webkit2_webextension_sys::WEBKIT_CONTEXT_MENU_ACTION_MEDIA_PLAY
            }
            ContextMenuAction::MediaPause => {
                webkit2_webextension_sys::WEBKIT_CONTEXT_MENU_ACTION_MEDIA_PAUSE
            }
            ContextMenuAction::MediaMute => {
                webkit2_webextension_sys::WEBKIT_CONTEXT_MENU_ACTION_MEDIA_MUTE
            }
            ContextMenuAction::DownloadVideoToDisk => {
                webkit2_webextension_sys::WEBKIT_CONTEXT_MENU_ACTION_DOWNLOAD_VIDEO_TO_DISK
            }
            ContextMenuAction::DownloadAudioToDisk => {
                webkit2_webextension_sys::WEBKIT_CONTEXT_MENU_ACTION_DOWNLOAD_AUDIO_TO_DISK
            }
            ContextMenuAction::InsertEmoji => {
                webkit2_webextension_sys::WEBKIT_CONTEXT_MENU_ACTION_INSERT_EMOJI
            }
            ContextMenuAction::PasteAsPlainText => {
                webkit2_webextension_sys::WEBKIT_CONTEXT_MENU_ACTION_PASTE_AS_PLAIN_TEXT
            }
            ContextMenuAction::Custom => {
                webkit2_webextension_sys::WEBKIT_CONTEXT_MENU_ACTION_CUSTOM
            }
            ContextMenuAction::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<webkit2_webextension_sys::WebKitContextMenuAction> for ContextMenuAction {
    fn from_glib(value: webkit2_webextension_sys::WebKitContextMenuAction) -> Self {
        skip_assert_initialized!();
        match value {
            0 => ContextMenuAction::NoAction,
            1 => ContextMenuAction::OpenLink,
            2 => ContextMenuAction::OpenLinkInNewWindow,
            3 => ContextMenuAction::DownloadLinkToDisk,
            4 => ContextMenuAction::CopyLinkToClipboard,
            5 => ContextMenuAction::OpenImageInNewWindow,
            6 => ContextMenuAction::DownloadImageToDisk,
            7 => ContextMenuAction::CopyImageToClipboard,
            8 => ContextMenuAction::CopyImageUrlToClipboard,
            9 => ContextMenuAction::OpenFrameInNewWindow,
            10 => ContextMenuAction::GoBack,
            11 => ContextMenuAction::GoForward,
            12 => ContextMenuAction::Stop,
            13 => ContextMenuAction::Reload,
            14 => ContextMenuAction::Copy,
            15 => ContextMenuAction::Cut,
            16 => ContextMenuAction::Paste,
            17 => ContextMenuAction::Delete,
            18 => ContextMenuAction::SelectAll,
            19 => ContextMenuAction::InputMethods,
            20 => ContextMenuAction::Unicode,
            21 => ContextMenuAction::SpellingGuess,
            22 => ContextMenuAction::NoGuessesFound,
            23 => ContextMenuAction::IgnoreSpelling,
            24 => ContextMenuAction::LearnSpelling,
            25 => ContextMenuAction::IgnoreGrammar,
            26 => ContextMenuAction::FontMenu,
            27 => ContextMenuAction::Bold,
            28 => ContextMenuAction::Italic,
            29 => ContextMenuAction::Underline,
            30 => ContextMenuAction::Outline,
            31 => ContextMenuAction::InspectElement,
            32 => ContextMenuAction::OpenVideoInNewWindow,
            33 => ContextMenuAction::OpenAudioInNewWindow,
            34 => ContextMenuAction::CopyVideoLinkToClipboard,
            35 => ContextMenuAction::CopyAudioLinkToClipboard,
            36 => ContextMenuAction::ToggleMediaControls,
            37 => ContextMenuAction::ToggleMediaLoop,
            38 => ContextMenuAction::EnterVideoFullscreen,
            39 => ContextMenuAction::MediaPlay,
            40 => ContextMenuAction::MediaPause,
            41 => ContextMenuAction::MediaMute,
            42 => ContextMenuAction::DownloadVideoToDisk,
            43 => ContextMenuAction::DownloadAudioToDisk,
            44 => ContextMenuAction::InsertEmoji,
            45 => ContextMenuAction::PasteAsPlainText,
            10000 => ContextMenuAction::Custom,
            value => ContextMenuAction::__Unknown(value),
        }
    }
}
